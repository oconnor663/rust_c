#ifndef LINKED_LIST_H
#define LINKED_LIST_H

typedef struct node node;

struct node {
    void *elem;
    node *next;
};

// In Rust terms we could say that `list` is a
// `&mut Option<Box<Node>>`.
void linked_list_push(node **list, void *elem);

void *linked_list_pop(node **list);

#endif
