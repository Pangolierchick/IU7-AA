#include "ant.h"
#include "colony.h"
#include "default.h"
#include "matrix.h"
#include "matrix.hpp"
#include <fstream>
#include <iostream>

int main() {
    std::ifstream f("/Users/kirill/Study/third_course/IU7-AA/lab_06/data/4.txt");
    auto mf = Matrix<size_t>::load(f);

    std::cout << "Введенная матрица:\n";

    mf.print();

    Colony c(mf);

    auto r = c.simulate(100);

    std::cout << "\n\n\n\t\tМУРАВЬИНЫЙ АЛГОРИТМ\n\n\n";

    for (const auto& i : r) {
        std::cout << i << " ";
    }

    std::cout << "\n";

    r = default_salesman(mf);

    std::cout << "\n\n\n\t\tАЛГОРИТМ ПОЛНОГО ПЕРЕБОРА\n\n\n";

    for (const auto& i : r) {
        std::cout << i << " ";
    }

    std::cout << "\n";

    return 0;
}
