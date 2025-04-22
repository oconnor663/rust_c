#include <stdlib.h>

#include "linked_list.h"

void linked_list_push(node **list, void *elem) {
    node *new_head = (node *)malloc(sizeof(node));
    new_head->elem = elem;
    new_head->next = *list;
    *list = new_head;
}

void *linked_list_pop(node **list) {
    node *old_head = *list;
    if (old_head == NULL) {
        return NULL;
    }
    void *elem = old_head->elem;
    *list = old_head->next;
    free(old_head);
    return elem;
}
