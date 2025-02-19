#include <stddef.h>

typedef struct c_vec {
    void **buffer;
    size_t length;
    size_t capacity;
} c_vec;

void c_vec_init(c_vec *self);
void c_vec_free(c_vec *self);
void c_vec_push(c_vec *self, void *elem);
void *c_vec_pop(c_vec *self);
void c_vec_take_all(c_vec *self, c_vec *other);
