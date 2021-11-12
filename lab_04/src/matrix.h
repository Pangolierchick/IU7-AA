#pragma once

#include <iostream>
#include <vector>

using real = float;

class Matrix {
public:
    Matrix() : rows(0), cols(0) {}
    Matrix(size_t r, size_t c) : rows(r), cols(c) { data.reserve(r * c); }

    void print(std::ostream &s = std::cout);

    void fill_rand();

    std::vector<real> rows_mean_parallel(size_t threads=1);
    std::vector<real> rows_mean();

    inline real operator()(size_t r, size_t c) const { return data[r * cols + c]; }
    inline real& operator()(size_t r, size_t c) { return data[r * cols + c]; }

    size_t getcols() const { return cols; }
    size_t getrows() const { return rows; }
    const std::vector<real>& getdata() const { return data; }

private:

    std::vector<real> data;
    size_t rows;
    size_t cols;
};
