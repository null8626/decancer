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
	All           = 0x1ffffff
	PureHomoglyph = 0x3ffffc
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

func Cure(text string, options Option) (*CuredString, error) {
	if text == "" {
		return nil, errors.New("unable to cure an empty string")
	}

	textBytes := []byte(text)
	var err C.decancer_error_t

	ptr := C.decancer_cure((*C.uint8_t)(unsafe.Pointer(&textBytes[0])), C.size_t(len(textBytes)), C.decancer_options_t(options), &err)

	if ptr == nil {
		return nil, errors.New(C.GoStringN(err.message, C.int(err.message_length)))
	}

	return &CuredString{ptr: ptr}, nil
}

func (cured *CuredString) Find(other string) []Match {
	matches := []Match{}

	if other == "" {
		return matches
	}

	otherBytes := []byte(other)
	matcher := C.decancer_find(cured.ptr, (*C.uint8_t)(unsafe.Pointer(&otherBytes[0])), C.size_t(len(otherBytes)))

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

func (cured *CuredString) FindMultiple(keywords []string) ([]Match, error) {
	rawKeywords := []C.decancer_keyword_t{}
	keywordsPtrs := make([]unsafe.Pointer, 0, len(keywords))

	defer func() {
		for _, keywordPtr := range keywordsPtrs {
			C.free(keywordPtr)
		}
	}()

	for _, keyword := range keywords {
		if keyword != "" {
			keywordBytes := []byte(keyword)
			keywordSize := C.size_t(len(keywordBytes))
			keywordPtr := C.malloc(keywordSize)

			if keywordPtr == nil {
				return nil, errors.New("out of memory")
			}

			C.memcpy(keywordPtr, unsafe.Pointer(&keywordBytes[0]), keywordSize)

			keywordsPtrs = append(keywordsPtrs, keywordPtr)
			rawKeywords = append(rawKeywords, C.decancer_keyword_t{
				string: (*C.uint8_t)(keywordPtr),
				size:   keywordSize,
			})
		}
	}

	rawKeywordsSize := len(rawKeywords)
	matches := []Match{}

	if rawKeywordsSize == 0 {
		return matches, nil
	}

	rawKeywordsPtr := (*C.decancer_keyword_t)(unsafe.Pointer(&rawKeywords[0]))
	rawMatchesWrapped := C.decancer_find_multiple(cured.ptr, rawKeywordsPtr, C.size_t(rawKeywordsSize))

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

	otherBytes := []byte(other)

	return bool(C.decancer_equals(cured.ptr, (*C.uint8_t)(unsafe.Pointer(&otherBytes[0])), C.size_t(len(otherBytes))))
}

func (cured *CuredString) StartsWith(other string) bool {
	if other == "" {
		return true
	}

	otherBytes := []byte(other)

	return bool(C.decancer_starts_with(cured.ptr, (*C.uint8_t)(unsafe.Pointer(&otherBytes[0])), C.size_t(len(otherBytes))))
}

func (cured *CuredString) EndsWith(other string) bool {
	if other == "" {
		return true
	}

	otherBytes := []byte(other)

	return bool(C.decancer_ends_with(cured.ptr, (*C.uint8_t)(unsafe.Pointer(&otherBytes[0])), C.size_t(len(otherBytes))))
}

func (cured *CuredString) Contains(other string) bool {
	if other == "" {
		return true
	}

	otherBytes := []byte(other)

	return bool(C.decancer_contains(cured.ptr, (*C.uint8_t)(unsafe.Pointer(&otherBytes[0])), C.size_t(len(otherBytes))))
}

func (cured *CuredString) Censor(other string, replacement rune) error {
	if other == "" {
		return nil
	}

	otherBytes := []byte(other)

	if C.decancer_censor(cured.ptr, (*C.uint8_t)(unsafe.Pointer(&otherBytes[0])), C.size_t(len(otherBytes)), C.uint32_t(replacement)) {
		return nil
	} else {
		return errors.New("got a malformed encoding")
	}
}

func (cured *CuredString) CensorMultiple(keywords []string, replacement rune) error {
	rawKeywords := []C.decancer_keyword_t{}
	keywordsPtrs := make([]unsafe.Pointer, 0, len(keywords))

	defer func() {
		for _, keywordPtr := range keywordsPtrs {
			C.free(keywordPtr)
		}
	}()

	for _, keyword := range keywords {
		if keyword != "" {
			keywordBytes := []byte(keyword)
			keywordSize := C.size_t(len(keywordBytes))
			keywordPtr := C.malloc(keywordSize)

			if keywordPtr == nil {
				return errors.New("out of memory")
			}

			C.memcpy(keywordPtr, unsafe.Pointer(&keywordBytes[0]), keywordSize)

			keywordsPtrs = append(keywordsPtrs, keywordPtr)
			rawKeywords = append(rawKeywords, C.decancer_keyword_t{
				string: (*C.uint8_t)(keywordPtr),
				size:   keywordSize,
			})
		}
	}

	rawKeywordsSize := len(rawKeywords)

	if rawKeywordsSize == 0 {
		return nil
	}

	rawKeywordsPtr := (*C.decancer_keyword_t)(unsafe.Pointer(&rawKeywords[0]))

	if C.decancer_censor_multiple(cured.ptr, rawKeywordsPtr, C.size_t(rawKeywordsSize), C.uint32_t(replacement)) {
		return nil
	} else {
		return errors.New("got a malformed encoding")
	}
}

func (cured *CuredString) Replace(other string, replacement string) error {
	if other == "" {
		return nil
	}

	otherBytes := []byte(other)
	otherPtr := (*C.uint8_t)(unsafe.Pointer(&otherBytes[0]))
	otherSize := C.size_t(len(otherBytes))

	var replacementPtr *C.uint8_t
	var replacementBytes []byte

	replacementSize := 0

	if replacement != "" {
		replacementBytes = []byte(replacement)
		replacementPtr = (*C.uint8_t)(unsafe.Pointer(&replacementBytes[0]))
		replacementSize = len(replacementBytes)
	}

	if C.decancer_replace(cured.ptr, otherPtr, otherSize, replacementPtr, C.size_t(replacementSize)) {
		return nil
	} else {
		return errors.New("got a malformed encoding")
	}
}

func (cured *CuredString) ReplaceMultiple(keywords []string, replacement string) error {
	rawKeywords := []C.decancer_keyword_t{}
	keywordsPtrs := make([]unsafe.Pointer, 0, len(keywords))

	defer func() {
		for _, keywordPtr := range keywordsPtrs {
			C.free(keywordPtr)
		}
	}()

	for _, keyword := range keywords {
		if keyword != "" {
			keywordBytes := []byte(keyword)
			keywordSize := C.size_t(len(keywordBytes))
			keywordPtr := C.malloc(keywordSize)

			if keywordPtr == nil {
				return errors.New("out of memory")
			}

			C.memcpy(keywordPtr, unsafe.Pointer(&keywordBytes[0]), keywordSize)

			keywordsPtrs = append(keywordsPtrs, keywordPtr)
			rawKeywords = append(rawKeywords, C.decancer_keyword_t{
				string: (*C.uint8_t)(keywordPtr),
				size:   keywordSize,
			})
		}
	}

	rawKeywordsSize := len(rawKeywords)

	if rawKeywordsSize == 0 {
		return nil
	}

	var replacementPtr *C.uint8_t
	var replacementBytes []byte

	replacementSize := 0

	if replacement != "" {
		replacementBytes = []byte(replacement)
		replacementPtr = (*C.uint8_t)(unsafe.Pointer(&replacementBytes[0]))
		replacementSize = len(replacementBytes)
	}

	if C.decancer_replace_multiple(cured.ptr, (*C.decancer_keyword_t)(unsafe.Pointer(&rawKeywords[0])), C.size_t(rawKeywordsSize), replacementPtr, C.size_t(replacementSize)) {
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
