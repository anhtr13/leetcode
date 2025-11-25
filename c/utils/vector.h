#pragma once

// Static vector with fixed capacity, prevent frequent memory reallocation
// Use when having a dataset with a limited length
// Don't touch this struct's members, otherwise it may break the algorithm!
typedef struct StaticVector {
  unsigned long capacity;
  unsigned long size;
  int *arr;
} static_vector;

static_vector *static_vector_create(unsigned long capacity);
void static_vector_push_back(static_vector *vec, int val);
int static_vector_pop_back(static_vector *vec);
void static_vector_set(static_vector *vec, unsigned long idx, int val);
int static_vector_get(static_vector *vec, unsigned long idx);
void static_vector_free(static_vector *vec);

// Dynamic vector with flexible capacity,
// Capacity += init_cap when vector reaches limit
// Use when having an unknow dataset
// Don't touch this struct's members, otherwise it may break the algorithm!
typedef struct DynamicVector {
  unsigned long init_cap;
  unsigned long capacity;
  unsigned long size;
  int *arr;
} dynamic_vector;

dynamic_vector *dynamic_vector_create(unsigned long init_cap);
void dynamic_vector_push_back(dynamic_vector *vec, int val);
int dynamic_vector_pop_back(dynamic_vector *vec);
void dynamic_vector_set(dynamic_vector *vec, unsigned long idx, int val);
int dynamic_vector_get(dynamic_vector *vec, unsigned long idx);
void dynamic_vector_free(dynamic_vector *vec);
