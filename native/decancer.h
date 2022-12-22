#ifndef __DECANCER_H__
#define __DECANCER_H__

#include <stdbool.h>
#include <stddef.h>

typedef void * decancer_t;

// cure function

decancer_t decancer_cure(unsigned char * utf8_string, size_t utf8_size);

// comparison functions

bool decancer_starts_with(decancer_t handle, unsigned char * utf8_string, size_t utf8_size);
bool decancer_ends_with(decancer_t handle, unsigned char * utf8_string, size_t utf8_size);
bool decancer_contains(decancer_t handle, unsigned char * utf8_string, size_t utf8_size);
bool decancer_equals(decancer_t handle, unsigned char * utf8_string, size_t utf8_size);

// misc functions

void decancer_retrieve_string(decancer_t handle, unsigned char ** output_utf8_string, size_t * output_utf8_size);
void decancer_free(decancer_t handle);

#endif