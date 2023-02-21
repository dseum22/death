#ifndef HEAP_H
#define HEAP_H

#include <cassert>
#include <tuple>
#include "Graph.hh"

// assume all min
using HeapVertexWeight = std::tuple<Vertex, double>;
using HeapVertexIndex = std::tuple<bool, int>;

class BinaryHeap
{
public:
    int size;
    int max_n;
    HeapVertexWeight *heap;
    HeapVertexIndex *contained;
    BinaryHeap(int n);
    void swap(int i, int j);
    int get_parent(int i);
    int get_lchild(int i);
    int get_rchild(int i);
    void update(Vertex v, double weight);
    Vertex remove();
    void min_heapify(int i);
    void print(int i, int t = 0);
};

// #include <cassert>
// #include <tuple>
// #include <vector>
// #include <iostream>
// #include <string>
// #include <math.h>
// #include <stdio.h>

// class node
// {
// private:
//     node *parent;
//     node *child;
//     node *left;
//     node *right;
//     int key;
//     int degree;
//     bool mark;

// public:

// }

// class Fibonacci_heap
// {
// }
#endif
