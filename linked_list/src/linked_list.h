#ifndef LINKED_LIST_H
#define LINKED_LIST_H

#include <stddef.h>

typedef struct node {
    void *elem;
    struct node *next;
    struct node *prev;
} node;

typedef struct linked_list {
    node *head;
    node *tail;
} linked_list;

void linked_list_init(linked_list *self);
size_t linked_list_len(const linked_list *self);
void linked_list_push(linked_list *self, void *elem);
void *linked_list_pop(linked_list *self);
void linked_list_concat(linked_list *self, linked_list *other);

#endif
