#include <iostream>
#include "matrix.h"

int main() {
    Matrix m(10, 10);

    for (size_t i = 0; i < m.getrows(); i++) {
        for (size_t j = 0; j < m.getcols(); j++) {
            m(i, j) = i + j;
        }
    }

    m.print();

    auto v_p = m.rows_mean_parallel(6);
    auto v_i = m.rows_mean();

    std::cout << "Mean parallel: ";

    for (int i = 0; i < v_p.size(); i++) {
        std::cout << v_p[i] << " ";
    }

    std::cout << "\n";

    std::cout << "Mean iter    : ";

    for (int i = 0; i < v_i.size(); i++) {
        std::cout << v_i[i] << " ";
    }

    std::cout << "\n";

    return 0;
}
