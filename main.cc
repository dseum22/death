
#include "Env.hh"
#include "Graph.hh"
#include "Heap.hh"
#include "Tests.hh"
// #include <time.h>

double prim_total_weight(Graph *g)
{
    std::cout << "Initializing algorithm..." << std::endl;
    int n = g->get_n();
    BinaryHeap *heap = new BinaryHeap(n);
    VertexVector vertices = g->get_vertices();
    EdgeList *edges = g->get_edges();
    double *dist = new double[n];
    for (int i = 0; i < n; ++i)
    {
        dist[i] = 2;
    }
    double total_weight = 0;
    heap->update(vertices[0], 0);
    dist[0] = 0;
    std::cout << "\t1. Running through heap." << std::endl;
    while (heap->size != 0)
    {
        Vertex v = heap->remove();
        int i = v.get();
        EdgeList &e_list = edges[i];
        Edge *curr = e_list.tail;
        while (e_list.size != 0)
        {
            Vertex w = curr->other(v);
            double e_weight = curr->get_weight();
            if (dist[w.get()] > e_weight)
            {
                dist[w.get()] = e_weight;
                heap->update(w, e_weight);
            }
            e_list.remove(curr);
            curr = e_list.tail;
        }
        // heap->print(0);
    }
    std::cout << "\t2. Summing weights." << std::endl;
    for (int i = 0; i < n; ++i)
    {
        total_weight += dist[i];
    }
    std::cout << "\tDone." << std::endl;
    return total_weight;
}

int main(int argc, char *argv[])
{
    int env, points, trials, dim;

    // input checking
    assert(argc == 5);
    assert_to_int(env, argv[1]);
    assert_to_int(points, argv[2]);
    assert_to_int(trials, argv[3]);
    assert_to_int(dim, argv[4]);

    if (dim == 0)
    {
        // dim 0 has no meaning
        dim = 1;
    }
    // body
    srand(time(NULL));
    double total_weight = 0;
    for (int i = 0; i < trials; ++i)
    {
        Graph *g = new Graph(points, dim);
        auto edges = g->get_edges();
        double run = prim_total_weight(g);
        total_weight += run;
        std::cout << "Run " << (i + 1) << ": " << run << std::endl;
        delete g;
    }
    std::cout << "Average: " << total_weight / trials << std::endl;

    // print_edges(edges, points);

    // test
    // std::cout << env << std::endl;
    // std::cout << points << std::endl;
    // std::cout << trials << std::endl;
    // std::cout << dim << std::endl;
    return 0;
}
