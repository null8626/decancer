// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2021-2026 null8626

//go:generate go run ../../scripts/setup_go_binding.go

package decancer

/*
#cgo CFLAGS: -I../native
#cgo LDFLAGS: -ldecancer
#cgo windows LDFLAGS: -lntdll

#define DECANCER_STATIC
#define DECANCER_UTF8_ONLY

#include <decancer.h>

#include <stdlib.h>
#include <string.h>
*/
import "C"
import (
	"errors"
	"unsafe"
)

type Option uint32

const (
	RetainCapitalization Option = 1 << iota
	DisableBidi
	DisableLeetspeak
	RetainDiacritics
	RetainGreek
	RetainCyrillic
	RetainHebrew
	RetainArabic
	RetainDevanagari
	RetainBengali
	RetainArmenian
	RetainGujarati
	RetainTamil
	RetainThai
	RetainLao
	RetainBurmese
	RetainKhmer
	RetainMongolian
	RetainChinese
	RetainJapanese
	RetainKorean
	RetainBraille
	RetainEmojis
	RetainTurkish
	AsciiOnly
	AlphanumericOnly
	Default       = 0
	All           = 0x3ffffff
	PureHomoglyph = 0x7ffff8
)

type CuredString struct {
	ptr C.decancer_cured_t
}

type Match struct {
	Start int
	End   int
}

func CureChar(character rune, options Option) string {
	var translation C.decancer_translation_t

	C.decancer_translation_init(&translation)

	C.decancer_cure_char(C.uint32_t(character), C.decancer_options_t(options), &translation)

	switch translation.kind {
	case C.DECANCER_TRANSLATION_KIND_NONE:
		return ""
	case C.DECANCER_TRANSLATION_KIND_CHARACTER:
		return string(rune(*(*uint32)(unsafe.Pointer(&translation.contents))))
	default:
		{
			defer C.decancer_translation_free(&translation)

			str := (*struct {
				contents *C.uint8_t
				size     C.size_t
				heap     unsafe.Pointer
			})(unsafe.Pointer(&translation.contents))

			return string(C.GoBytes(unsafe.Pointer(str.contents), C.int(str.size)))
		}
	}
}

type processedString struct {
	bytes []byte
	size  int
}

func processString(text string) *processedString {
	var bytes []byte

	size := 0

	if text != "" {
		bytes = []byte(text)
		size = len(bytes)
	}

	return &processedString{
		bytes: bytes,
		size:  size,
	}
}

func (pString *processedString) Len() int {
	return len(pString.bytes)
}

func (pString *processedString) Pointer() *C.uint8_t {
	return (*C.uint8_t)(unsafe.Pointer(&pString.bytes[0]))
}

func Cure(text string, options Option) (*CuredString, error) {
	if text == "" {
		return nil, errors.New("unable to cure an empty string")
	}

	pText := processString(text)
	var err C.decancer_error_t

	ptr := C.decancer_cure(pText.Pointer(), C.size_t(pText.Len()), C.decancer_options_t(options), &err)

	if ptr == nil {
		return nil, errors.New(C.GoStringN(err.message, C.int(err.message_length)))
	}

	return &CuredString{ptr: ptr}, nil
}

func (cured *CuredString) DisableLeetspeak(switch_ bool) {
	C.decancer_disable_leetspeak(cured.ptr, C.bool(switch_))
}

func (cured *CuredString) Find(other string) []Match {
	var matches []Match

	if other == "" {
		return matches
	}

	pOther := processString(other)
	matcher := C.decancer_find(cured.ptr, pOther.Pointer(), C.size_t(pOther.Len()))

	if matcher == nil {
		return matches
	}

	defer C.decancer_matcher_free(matcher)

	var match C.decancer_match_t

	for bool(C.decancer_matcher_next(matcher, &match)) {
		matches = append(matches, Match{
			Start: int(match.start),
			End:   int(match.end),
		})
	}

	return matches
}

type processedKeywords struct {
	structs []C.decancer_keyword_t
	ptrs    []unsafe.Pointer
}

func processKeywords(keywords []string) (*processedKeywords, error) {
	output := &processedKeywords{
		structs: []C.decancer_keyword_t{},
		ptrs:    make([]unsafe.Pointer, 0, len(keywords)),
	}

	for _, keyword := range keywords {
		if keyword != "" {
			keywordBytes := []byte(keyword)
			keywordSize := C.size_t(len(keywordBytes))
			keywordPtr := C.malloc(keywordSize)

			if keywordPtr == nil {
				output.Close()

				return nil, errors.New("out of memory")
			}

			C.memcpy(keywordPtr, unsafe.Pointer(&keywordBytes[0]), keywordSize)

			output.ptrs = append(output.ptrs, keywordPtr)
			output.structs = append(output.structs, C.decancer_keyword_t{
				string: (*C.uint8_t)(keywordPtr),
				size:   keywordSize,
			})
		}
	}

	return output, nil
}

func (keywords *processedKeywords) Len() int {
	return len(keywords.structs)
}

func (keywords *processedKeywords) Pointer() *C.decancer_keyword_t {
	return (*C.decancer_keyword_t)(unsafe.Pointer(&keywords.structs[0]))
}

func (keywords *processedKeywords) Close() {
	for _, ptr := range keywords.ptrs {
		C.free(ptr)
	}
}

func (cured *CuredString) FindMultiple(keywords []string) ([]Match, error) {
	pKeywords, err := processKeywords(keywords)

	if err != nil {
		return nil, err
	}

	defer pKeywords.Close()

	rawKeywordsSize := pKeywords.Len()
	matches := []Match{}

	if rawKeywordsSize == 0 {
		return matches, nil
	}

	rawMatchesWrapped := C.decancer_find_multiple(cured.ptr, pKeywords.Pointer(), C.size_t(rawKeywordsSize))

	if rawMatchesWrapped == nil {
		return nil, errors.New("got an invalid keywords array")
	}

	defer C.decancer_matches_free(rawMatchesWrapped)

	var rawMatchesSize C.size_t

	rawMatchesPtr := C.decancer_matches_raw(rawMatchesWrapped, &rawMatchesSize)
	rawMatches := unsafe.Slice(rawMatchesPtr, rawMatchesSize)

	matches = make([]Match, rawMatchesSize)

	for i, match := range rawMatches {
		matches[i] = Match{
			Start: int(match.start),
			End:   int(match.end),
		}
	}

	return matches, nil
}

func (cured *CuredString) Equals(other string) bool {
	if other == "" {
		return bool(C.decancer_equals(cured.ptr, nil, 0))
	}

	pOther := processString(other)

	return bool(C.decancer_equals(cured.ptr, pOther.Pointer(), C.size_t(pOther.Len())))
}

func (cured *CuredString) StartsWith(other string) bool {
	if other == "" {
		return true
	}

	pOther := processString(other)

	return bool(C.decancer_starts_with(cured.ptr, pOther.Pointer(), C.size_t(pOther.Len())))
}

func (cured *CuredString) EndsWith(other string) bool {
	if other == "" {
		return true
	}

	pOther := processString(other)

	return bool(C.decancer_ends_with(cured.ptr, pOther.Pointer(), C.size_t(pOther.Len())))
}

func (cured *CuredString) Contains(other string) bool {
	if other == "" {
		return true
	}

	pOther := processString(other)

	return bool(C.decancer_contains(cured.ptr, pOther.Pointer(), C.size_t(pOther.Len())))
}

func (cured *CuredString) Censor(other string, replacement rune) error {
	if other == "" {
		return nil
	}

	pOther := processString(other)

	if C.decancer_censor(cured.ptr, pOther.Pointer(), C.size_t(pOther.Len()), C.uint32_t(replacement)) {
		return nil
	} else {
		return errors.New("got a malformed encoding")
	}
}

func (cured *CuredString) CensorMultiple(keywords []string, replacement rune) error {
	pKeywords, err := processKeywords(keywords)

	if err != nil {
		return err
	}

	defer pKeywords.Close()

	rawKeywordsSize := pKeywords.Len()

	if rawKeywordsSize == 0 || C.decancer_censor_multiple(cured.ptr, pKeywords.Pointer(), C.size_t(rawKeywordsSize), C.uint32_t(replacement)) {
		return nil
	} else {
		return errors.New("got a malformed encoding")
	}
}

func (cured *CuredString) Replace(other string, replacement string) error {
	if other == "" {
		return nil
	}

	pOther := processString(other)
	pReplacement := processString(replacement)

	if C.decancer_replace(cured.ptr, pOther.Pointer(), C.size_t(pOther.size), pReplacement.Pointer(), C.size_t(pReplacement.size)) {
		return nil
	} else {
		return errors.New("got a malformed encoding")
	}
}

func (cured *CuredString) ReplaceMultiple(keywords []string, replacement string) error {
	pKeywords, err := processKeywords(keywords)

	if err != nil {
		return err
	}

	defer pKeywords.Close()

	rawKeywordsSize := pKeywords.Len()

	if rawKeywordsSize == 0 {
		return nil
	}

	pReplacement := processString(replacement)

	if C.decancer_replace_multiple(cured.ptr, pKeywords.Pointer(), C.size_t(rawKeywordsSize), pReplacement.Pointer(), C.size_t(pReplacement.Len())) {
		return nil
	} else {
		return errors.New("got a malformed encoding")
	}
}

func (cured *CuredString) String() string {
	var rawSize C.size_t

	rawPtr := C.decancer_cured_raw(cured.ptr, nil, &rawSize)

	return string(C.GoBytes(unsafe.Pointer(rawPtr), C.int(rawSize)))
}

func (cured *CuredString) Close() {
	C.decancer_cured_free(cured.ptr)
}
