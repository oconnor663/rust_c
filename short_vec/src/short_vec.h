#ifndef SHORT_VEC_H
#define SHORT_VEC_H

#include <stddef.h>

#define CAPACITY 10

typedef struct short_vec {
    void *buffer[CAPACITY];
    size_t length;
} short_vec;

void short_vec_push(short_vec *self, void *elem);
void *short_vec_pop(short_vec *self);

#endif
