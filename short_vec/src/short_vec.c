#include <stdio.h>
#include <stdlib.h>

#include "short_vec.h"

void short_vec_push(short_vec *self, void *elem) {
    if (self->length < CAPACITY) {
        self->buffer[self->length] = elem;
        self->length++;
    }
}

void *short_vec_pop(short_vec *self) {
    if (self->length > 0) {
        self->length--;
        return self->buffer[self->length];
    } else {
        return NULL;
    }
}

void short_vec_take_all(short_vec *self, short_vec *other) {
    for (size_t i = 0; i < other->length; i++) {
        short_vec_push(self, other->buffer[i]);
    }
    other->length = 0;
}
