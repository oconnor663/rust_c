#include "short_vec.h"

void short_vec_push(short_vec *self, void *elem) {
    self->buffer[self->length] = elem;
    self->length++;
}

void *short_vec_pop(short_vec *self) {
    if (self->length > 0) {
        self->length--;
        return self->buffer[self->length];
    } else {
        return NULL;
    }
}
