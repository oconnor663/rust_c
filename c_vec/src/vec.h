#include <stddef.h>

typedef struct vec {
    void **buffer;
    size_t length;
    size_t capacity;
} vec;

void vec_init(vec *self);
void vec_free(vec *self);
void vec_push(vec *self, void *elem);
void *vec_pop(vec *self);
