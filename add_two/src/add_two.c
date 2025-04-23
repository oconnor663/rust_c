#include <stdint.h>

int32_t add_one(int32_t x);

int32_t add_two(int32_t x) {
    return add_one(add_one(x)) + 7;
}
