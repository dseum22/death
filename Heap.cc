#include "Heap.hh"

BinaryHeap::BinaryHeap(int n)
{
    size = 0;
    max_n = n;
    heap = new HeapVertexWeight[max_n];
    contained = new HeapVertexIndex[max_n];
}

void BinaryHeap::swap(int i, int j)
{
    HeapVertexWeight temp = heap[i];
    heap[i] = heap[j];
    heap[j] = temp;
    contained[std::get<0>(heap[i]).get()] = std::make_tuple(true, i);
    contained[std::get<0>(heap[j]).get()] = std::make_tuple(true, j);
}

int BinaryHeap::get_parent(int i)
{
    return (i - 1) / 2;
}

int BinaryHeap::get_lchild(int i)
{
    return 2 * i + 1;
}

int BinaryHeap::get_rchild(int i)
{
    return 2 * i + 2;
}

void BinaryHeap::update(Vertex v, double weight)
{
    HeapVertexIndex &contained_v = contained[v.get()];
    std::cout << "(" << std::get<0>(contained_v) << ", " << std::get<1>(contained_v) << ") " << vertex_to_str(v) << std::endl;
    if (std::get<0>(contained_v))
    {
        // assumes update to smaller value
        int i = std::get<1>(contained_v);
        heap[i] = std::make_tuple(v, weight);
        if (i != 0 && std::get<1>(heap[this->get_parent(i)]) > std::get<1>(heap[i]))
            while (i != 0 && std::get<1>(heap[this->get_parent(i)]) > std::get<1>(heap[i]))
            {
                int parent_i = this->get_parent(i);
                this->swap(parent_i, i);
                i = parent_i;
            }
        else
        {
            contained[v.get()] = std::make_tuple(true, i);
        }
    }
    else
    {
        assert(size != max_n);
        ++size;
        int i = size - 1;
        heap[i] = std::make_tuple(v, weight);
        if (i != 0 && std::get<1>(heap[this->get_parent(i)]) > std::get<1>(heap[i]))
        {
            while (i != 0 && std::get<1>(heap[this->get_parent(i)]) > std::get<1>(heap[i]))
            {
                int parent_i = this->get_parent(i);
                this->swap(parent_i, i);
                i = parent_i;
            }
        }
        else
        {
            contained[v.get()] = std::make_tuple(true, i);
        }
    }
}

Vertex BinaryHeap::remove()
{
    if (size == 0)
    {
        Vertex *never = new Vertex();
        return *never;
    }
    else if (size == 1)
    {
        --size;
        return std::get<0>(heap[0]);
    }
    else
    {
        Vertex out = std::get<0>(heap[0]);
        auto [v, weight] = heap[size - 1];
        heap[0] = std::make_tuple(v, weight);
        --size;
        this->min_heapify(0);
        return out;
    }
}

void BinaryHeap::min_heapify(int i)
{
    int lchild = this->get_lchild(i);
    int rchild = this->get_rchild(i);
    int min_i = i;
    if (lchild < size && std::get<1>(heap[lchild]) < std::get<1>(heap[i]))
    {
        min_i = lchild;
    }
    if (rchild < size && std::get<1>(heap[rchild]) < std::get<1>(heap[min_i]))
    {
        min_i = rchild;
    }
    if (min_i != i)
    {
        this->swap(min_i, i);
        this->min_heapify(min_i);
    }
}

void BinaryHeap::print(int i, int t)
{
    if (i == 0)
    {
        std::cout << "\nPrinting heap..." << std::endl;
    }
    if (i < size)
    {
        auto [v, weight] = heap[i];
        std::cout << std::string(t, '.') << weight << ": " << vertex_to_str(v) << std::endl;
        int lchild = this->get_lchild(i);
        int rchild = this->get_rchild(i);
        this->print(lchild, t + 1);
        this->print(rchild, t + 1);
    }
}
