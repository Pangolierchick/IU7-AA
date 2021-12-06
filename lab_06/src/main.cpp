#include "ant.h"
#include "colony.h"
#include "matrix.h"
#include "matrix.hpp"
#include <fstream>
#include "default.h"
#include <iostream>

int main() {
    std::ifstream f("/Users/kirill/Study/third_course/IU7-AA/lab_06/data/3.txt");
    auto mf = Matrix<size_t>::load(f);

    mf.print();

    Colony c(mf);

    auto r = c.simulate(100);

    for (const auto& i : r) {
        std::cout << i << " ";
    }

    std::cout << "\n";

    std::cout << "========= DEFAULT =========\n";

    r = default_salesman(mf);

    for (const auto& i : r) {
        std::cout << i << " ";
    }

    std::cout << "\n";

    return 0;
}
