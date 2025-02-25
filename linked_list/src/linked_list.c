#include <assert.h>
#include <stdlib.h>

#include "linked_list.h"

void linked_list_init(linked_list *self) {
    self->head = NULL;
    self->tail = NULL;
}

size_t linked_list_len(const linked_list *self) {
    size_t len = 0;
    node *cursor = self->head;
    while (cursor != NULL) {
        len++;
        cursor = cursor->next;
    }
    return len;
}

void linked_list_push(linked_list *self, void *elem) {
    node *new_tail = malloc(sizeof(node));
    new_tail->elem = elem;
    new_tail->next = NULL;
    new_tail->prev = self->tail;
    if (self->tail != NULL) {
        assert(self->head != NULL);
        self->tail->next = new_tail;
    } else {
        assert(self->head == NULL);
        self->head = new_tail;
    }
    self->tail = new_tail;
}

void *linked_list_pop(linked_list *self) {
    if (self->tail == NULL) {
        assert(self->head == NULL);
        return NULL;
    }
    assert(self->head != NULL);
    void *ret = self->tail->elem;
    node *new_tail = self->tail->prev;
    if (new_tail != NULL) {
        new_tail->next = NULL;
    } else {
        self->head = NULL;
    }
    self->tail = new_tail;
    return ret;
}

void linked_list_concat(linked_list *self, linked_list *other) {
    if (self->head == NULL) {
        *self = *other;
        linked_list_init(other);
        return;
    }
    if (other->head == NULL) {
        return;
    }
    self->tail->next = other->head;
    other->head->prev = self->tail;
    self->tail = other->tail;
    linked_list_init(other);
}
