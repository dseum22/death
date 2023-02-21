#ifndef GRAPH_H
#define GRAPH_H

#include <cassert>
#include <tuple>
#include <vector>
#include <iostream>
#include <string>
#include <math.h>

class Vertex
{
    // vertex possibly with position
private:
    int self;
    int dim;
    double *pos;

public:
    Vertex();
    Vertex(int self, int dim);
    int get();
    int get_dim();
    double *&get_pos();
};

class EdgeList;

class Edge
{
    // undirected edge
private:
    Vertex *self;
    double weight;

public:
    Edge *prev;
    Edge *next;
    EdgeList *sister_list;
    Edge *sister_edge;
    Edge();
    Edge(Vertex a, Vertex b);
    Vertex other(Vertex a);
    Vertex get(int index);
    double get_weight();
    void set_sister(EdgeList *sister_list, Edge *e_ptr);
};

using VertexVector = std::vector<Vertex>;
using EdgeVector = std::vector<Edge>;

class EdgeList
{
public:
    int size = 0;
    Edge *tail;
    EdgeList();
    void insert(Edge *edge);
    void remove(bool is_sister);
    void remove(Edge *edge, bool is_sister);
};

class Graph
{
    // complete graph
private:
    int n;
    VertexVector vertices;
    EdgeList *edges;
    std::vector<Edge *> alloc_edges;

public:
    Graph(int n, int dim);
    int get_n();
    VertexVector get_vertices();
    EdgeList *get_edges();
};

std::string vertex_to_str(Vertex vertex);
std::string edge_to_str(Edge edge);
void print_edges(EdgeList *edges, int n);

#endif
