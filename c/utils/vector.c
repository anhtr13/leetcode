#include "vector.h"
#include <stdio.h>
#include <stdlib.h>

static_vector *static_vector_create(unsigned long capacity) {
  int *arr = calloc(capacity, sizeof(int));
  static_vector *vec = malloc(sizeof(static_vector));
  vec->arr = arr;
  vec->capacity = capacity;
  vec->size = 0;
  return vec;
}
void static_vector_push_back(static_vector *vec, int val) {
  if (vec->size == vec->capacity) {
    printf("vector is full!");
    return;
  }
  vec->arr[vec->size] = val;
  vec->size++;
}
int static_vector_pop_back(static_vector *vec) {
  vec->size--;
  int ret = vec->arr[vec->size];
  vec->arr[vec->size] = 0;
  return ret;
}
void static_vector_set(static_vector *vec, unsigned long idx, int val) {
  if (idx >= vec->size) {
    printf("index out of range");
    return;
  }
  vec->arr[idx] = val;
}
int static_vector_get(static_vector *vec, unsigned long idx) {
  if (idx >= vec->size) {
    printf("index out of range");
    return -1;
  }
  return vec->arr[idx];
}
void static_vector_free(static_vector *vec) {
  free(vec->arr);
  free(vec);
}

dynamic_vector *dynamic_vector_create(unsigned long init_cap) {
  int *arr = calloc(init_cap, sizeof(int));
  dynamic_vector *vec = malloc(sizeof(dynamic_vector));
  vec->init_cap = init_cap;
  vec->capacity = init_cap;
  vec->size = 0;
  vec->arr = arr;
  return vec;
}
void dynamic_vector_push_back(dynamic_vector *vec, int val) {
  if (vec->size == vec->capacity) {
    vec->capacity += vec->init_cap;
  }
  vec->arr[vec->size] = val;
  vec->size++;
};
int dynamic_vector_pop_back(dynamic_vector *vec) {
  vec->size--;
  int ret = vec->arr[vec->size];
  vec->arr[vec->size] = 0;
  return ret;
}
void dynamic_vector_set(dynamic_vector *vec, unsigned long idx, int val) {
  if (idx >= vec->size) {
    printf("index out of range");
    return;
  }
  vec->arr[idx] = val;
}
int dynamic_vector_get(dynamic_vector *vec, unsigned long idx) {
  if (idx >= vec->size) {
    printf("index out of range");
    return -1;
  }
  return vec->arr[idx];
}
void dynamic_vector_free(dynamic_vector *vec) {
  free(vec->arr);
  free(vec);
}
