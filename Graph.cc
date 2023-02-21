#include "Graph.hh"

std::string vertex_to_str(Vertex vertex)
{
    std::string out = "Vertex(" + std::to_string(vertex.get()) + ", {";
    int dim = vertex.get_dim();
    double *&pos = vertex.get_pos();
    for (int i = 0; i < dim - 1; ++i)
    {
        out += std::to_string(pos[i]) + ", ";
    }
    out += std::to_string(pos[dim - 1]) + "})";
    return out;
}

std::string edge_to_str(Edge edge)
{
    std::string out = "Edge(" + std::to_string(edge.get_weight()) + ", " + vertex_to_str(edge.get(0)) + ", " + vertex_to_str(edge.get(1)) + ")";
    return out;
}

void print_vertices(VertexVector vertices)
{
    for (auto &vertex : vertices)
    {
        std::cout << vertex_to_str(vertex) << std::endl;
    }
}

void print_edges(EdgeList *edges, int n)
{
    for (int i = 0; i < n; ++i)
    {
        std::cout << "Vertex " << i << ":" << std::endl;
        EdgeList *e_list = &edges[i];
        Edge *curr = e_list->tail;
        while (e_list->size != 0 && curr != nullptr)
        {
            std::cout << "\t" << edge_to_str(*curr) << std::endl;
            curr = curr->next;
        }
    }
}

Vertex::Vertex() {}

Vertex::Vertex(int self, int dim)
{
    // pos used for edge weight generation
    this->self = self;
    this->dim = dim;
    pos = new double[dim];
    for (int i = 0; i < dim; ++i)
    {
        pos[i] = (double)rand() / RAND_MAX;
    }
}

int Vertex::get()
{
    return self;
}

int Vertex::get_dim()
{
    return dim;
}

double *&Vertex::get_pos()
{
    return pos;
}

Edge::Edge() {}

Edge::Edge(Vertex a, Vertex b) : self(new Vertex[2])
{
    weight = 0;
    self[0] = a;
    self[1] = b;
    int dim = a.get_dim();
    double *&a_pos = a.get_pos();
    double *&b_pos = b.get_pos();
    for (int i = 0; i < dim; ++i)
    {
        weight += pow(a_pos[i] - b_pos[i], 2);
    }
    weight = sqrt(weight);
}

Vertex Edge::other(Vertex a)
{
    if (this->get(0).get() == a.get())
    {
        return this->get(1);
    }
    return this->get(0);
}

Vertex Edge::get(int index)
{
    return self[index];
}

double Edge::get_weight()
{
    return weight;
}

void Edge::set_sister(EdgeList *sister_list, Edge *e_ptr)
{
    sister_list = sister_list;
    sister_edge = e_ptr;
}

EdgeList::EdgeList() {}

void EdgeList::insert(Edge *edge)
{
    if (size == 0)
    {
        this->tail = edge;
    }
    else
    {
        this->tail->prev = edge;
        edge->next = this->tail;
        this->tail = edge;
    }
    ++size;
}

void EdgeList::remove(bool is_sister = false)
{
    if (size == 1)
    {
        --size;
    }
    else if (size > 1)
    {
        Edge *old_e = this->tail;
        Edge *new_e = this->tail->next;
        old_e->next = nullptr;
        new_e->prev = nullptr;
        this->tail = new_e;
        if (!is_sister)
        {
            old_e->sister_list->remove(old_e->sister_edge, true);
        }
        --size;
    }
}

void EdgeList::remove(Edge *edge, bool is_sister = false)
{
    std::cout << "running" << std::endl;
    if (size == 1)
    {
        --size;
    }
    else if (size > 1)
    {
        Edge *prev = edge->prev;
        Edge *next = edge->next;
        std::cout << "running" << std::endl;
        if (prev == nullptr)
        {
            next->prev = nullptr;
            edge->next = nullptr;
            this->tail = next;
        }
        else if (next == nullptr)
        {
            prev->next = nullptr;
            edge->prev = nullptr;
        }
        else
        {
            next->prev = prev;
            prev->next = next;
            edge->prev = nullptr;
            edge->next = nullptr;
        }
        if (!is_sister)
        {
            edge->sister_list->remove(edge->sister_edge, true);
        }
        --size;
    }
}

Graph::Graph(int n, int dim)
{
    assert(n > 0);
    std::cout << "Constructing graph..." << std::endl;
    this->n = n;
    edges = new EdgeList[n];
    std::cout << "\t1. Adding vertices." << std::endl;
    for (int i = 0; i < n; ++i)
    {
        Vertex v(i, dim);
        vertices.push_back(v);
    }
    std::cout << "\t2. Adding edges." << std::endl;
    for (int i = 0; i < n; ++i)
    {
        for (int j = i + 1; j < n; ++j)
        {
            Edge *e_a = new Edge(vertices[i], vertices[j]);
            Edge *e_b = new Edge(vertices[j], vertices[i]);
            e_a->set_sister(&edges[j], e_b);
            e_b->set_sister(&edges[i], e_a);
            edges[i].insert(e_a);
            edges[j].insert(e_b);
        }
    }
    std::cout << "\tDone." << std::endl;
}

int Graph::get_n()
{
    return n;
}

VertexVector Graph::get_vertices()
{
    return vertices;
}

EdgeList *Graph::get_edges()
{
    return edges;
}
