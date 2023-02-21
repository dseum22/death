#ifndef TESTS_H
#define TESTS_H

#include <iostream>
#include <sstream>
#include <cassert>
#include "Env.hh"

void assert_to_env(env &var, char in[]);
void assert_to_int(int &var, char in[]);

#endif
