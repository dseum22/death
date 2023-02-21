#include "Tests.hh"

void assert_to_env(env &var, char in[])
{
    std::istringstream ss(in);
    int temp;
    assert(ss >> temp);
    switch (temp)
    {
    case 0:
        var = env::production;
        break;
    case 1:
        var = env::test_heap;
        break;
    case 2:
        var = env::test_all;
    default:
        std::cout << "Invalid env." << std::endl;
    }
}

void assert_to_int(int &var, char in[])
{
    std::istringstream ss(in);
    assert(ss >> var);
}
