#pragma once

// Don't touch this struct's members, otherwise it may break the algorithm!
typedef struct HashTableItem {
  char *key;
  char *val;
} hash_table_item;

// Don't touch this struct's members, otherwise it may break the algorithm!
typedef struct HashTableLinkedList {
  hash_table_item *item;
  struct HashTableLinkedList *next;
} hash_table_linked_list;

// Hash map(hash table) data structure
// Don't touch this struct's members, otherwise it may break the algorithm!
typedef struct HashTable {
  unsigned long capacity;
  unsigned long size;
  hash_table_linked_list **overflow_bucket;
  hash_table_item **items;
} hash_table;

unsigned long hash_function(char *str, unsigned long capacity);

// Create a hash table with key-value of string
hash_table *hash_table_create(unsigned long capacity);

// Set a pair <key-value> to hash table
void hash_table_set(hash_table *table, char *key, char *val);

// Get value of key, if not set, return NULL
char *hash_table_get(hash_table *table, char *key);

// Get the hash_table's size
unsigned long hash_table_size(hash_table *table);

// Free the memory of the hash table
void hash_table_free(hash_table *table);
