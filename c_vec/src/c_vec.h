#include <stddef.h>

typedef struct c_vec {
    void **buffer;
    size_t length;
    size_t capacity;
} c_vec;

void vec_init(c_vec *self);
void vec_free(c_vec *self);
void vec_push(c_vec *self, void *elem);
void *vec_pop(c_vec *self);
