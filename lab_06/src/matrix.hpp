#pragma once

#include "matrix.h"
#include <iostream>
#include <sstream>

template <typename T>
void Matrix<T>::print(std::ostream& s) {
    s << rows << " " << cols << "\n";

    for (size_t i = 0; i < rows; i++) {
        for (size_t j = 0; j < cols; j++) {
            s << data[i * cols + j] << " ";
        }

        s << "\n";
    }
}

template <typename T>
Matrix<T> Matrix<T>::load(std::istream& i) {
    std::string s;

    size_t r = 0;
    size_t c = 0;

    getline(i, s);

    std::cout << "Read: " << s << "\n";

    std::istringstream ss(s);

    ss >> r >> c;

    std::vector<T> buf;

    while (std::getline(i, s)) {
        std::istringstream ss(s);

        for (size_t i = 0; i < c; i++) {
            T v;

            if (!(ss >> v))
                break;
            buf.push_back(v);
        }
    }

    Matrix<T> matrix(buf, r, c);

    return matrix;
}
