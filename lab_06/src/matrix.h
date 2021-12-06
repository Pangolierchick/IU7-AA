#pragma once

#include <iostream>
#include <vector>

template <typename T>
class Matrix {
public:
    Matrix() : rows(0), cols(0) {}
    Matrix(size_t r, size_t c) : rows(r), cols(c) { data.resize(r * c); }
    Matrix(const Matrix<T>& m) = default;

    Matrix(const std::vector<T>& d, size_t r, size_t c) : rows(r), cols(c) {
        data.resize(r * c);
        std::copy(d.begin(), d.end(), data.begin());
    }

    Matrix(Matrix<T>&& m) {
        cols = m.cols;
        rows = m.rows;
        data = m.data;

        m.data.clear();
        m.cols = 0;
        m.rows = 0;
    }

    Matrix<T> operator=(Matrix<T>&& m) {
        cols = m.cols;
        rows = m.rows;
        data = m.data;

        m.data.clear();
        m.cols = 0;
        m.rows = 0;

        return *this;
    }

    Matrix<T> operator=(const Matrix<T>& m) { return m.clone(); }

    void print(std::ostream& s = std::cout);

    inline T operator()(size_t r, size_t c) const { return data.at(r * cols + c); }
    inline T& operator()(size_t r, size_t c) { return data.at(r * cols + c); }

    size_t getcols() const { return cols; }
    size_t getrows() const { return rows; }
    const std::vector<T>& getdata() const { return data; }

    Matrix<T> clone() const {
        Matrix<T> m_c(rows, cols);

        std::copy(data.begin(), data.end(), m_c.data.begin());

        return m_c;
    }

    static Matrix<T> load(std::istream& i);

private:
    std::vector<T> data;
    size_t rows;
    size_t cols;
};
