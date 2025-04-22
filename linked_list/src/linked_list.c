#include <stdlib.h>

#include "linked_list.h"

void linked_list_push(node **list, void *elem) {
    node *new_node = (node *)malloc(sizeof(node));
    new_node->elem = elem;
    new_node->next = *list;
    *list = new_node;
}

void *linked_list_pop(node **list) {
    node *head = *list;
    if (head == NULL) {
        return NULL;
    }
    void *elem = head->elem;
    *list = head->next;
    free(head);
    return elem;
}
