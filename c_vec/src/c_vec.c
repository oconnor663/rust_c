#include <stdio.h>
#include <stdlib.h>

#include "c_vec.h"

void vec_init(c_vec *self) {
    self->buffer = NULL;
    self->length = 0;
    self->capacity = 0;
}

void vec_free(c_vec *self) {
    free(self->buffer);
}

void vec_push(c_vec *self, void *elem) {
    if (self->length == self->capacity) {
        size_t new_cap = (self->capacity > 0) ? (self->capacity * 2) : 1;
        self->buffer = realloc(self->buffer, new_cap * sizeof(void *));
        if (self->buffer == NULL) {
            fprintf(stderr, "realloc failed!");
            exit(1);
        }
        self->capacity = new_cap;
    }
    self->buffer[self->length] = elem;
    self->length++;
}

void *vec_pop(c_vec *self) {
    if (self->length > 0) {
        self->length--;
        return self->buffer[self->length];
    } else {
        return NULL;
    }
}
