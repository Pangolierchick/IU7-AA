#include <iostream>
#include <cmath>
#include <thread>
#include "matrix.h"

#ifdef DEBUG
#include <cstdio>

#define debug_puts(s) puts(s)
#define debug_printf(s, ...) printf(s, __VA_ARGS__)
#else

#define debug_puts(s) (void) 0
#define debug_printf(s, ...) (void) 0
#endif

void Matrix::print(std::ostream &s) {
    s << rows << " " << cols << "\n";

    for (size_t i = 0; i < rows; i++) {
        for (size_t j = 0; j < cols; j++) {
            s << data[i * cols + j] << " ";
        }

        s << "\n";
    }
}

static real row_mean(const real *ptr, size_t l) {
    real acc = 0;

    for (size_t i = 0; i < l; i++) {
        acc += ptr[i];
    }

    return acc / l;
}

// HACK WARNING: race conditions
static void rows_worker(const Matrix &m, size_t rl, size_t rr, std::vector<real> &res) {
    for (size_t r = rl; r < rr; r++) {
        res[r] = row_mean(m.getdata().data() + (r * m.getcols()), m.getcols());
    }
}
// n 2
// 1, 2, 3 ... n
// 1, 2, 3 ... n
// 1, 2, 3 ... n
// 1, 2, 3 ... n
// 1, 2, 3 ... n

std::vector<real> Matrix::rows_mean_parallel(size_t t_num) {
    std::vector<std::thread> threads(t_num);
    std::vector<real> res(rows);

    size_t rpw       = rows / t_num; // rows for thread
    size_t remainder = rows % t_num;

    debug_printf("Rpw = %zu\n", rpw);
    debug_printf("Remainder = %zu\n", remainder);

    size_t t = 0;
    size_t r = 0;

    while (t < t_num && r < rows) {
        size_t right = r + rpw;

        if (remainder > 0) {
            right++;
            remainder--;
        }

        threads[t] = std::thread(rows_worker, std::cref(*this), r, right, std::ref(res));

        debug_printf("Thread %zu/%zu is running. Left = %zu, right = %zu\n", t + 1, t_num, r, right);
        
        r = right;
        t++;
    }

    for (auto && i : threads) {
        i.join();
    }

    return res;
}

real row_mean(const Matrix &m, size_t r) {
    real acc = 0;

    for (size_t c = 0; c < m.getcols(); c++) {
        acc += m(r, c);
    }

    return acc / m.getcols();
}

std::vector<real> Matrix::rows_mean() {
    std::vector<real> res(rows);


    for (size_t i = 0; i < rows; i++) {
        res[i] = row_mean(*this, i);
    }

    return res;
}

void Matrix::fill_rand() {
    for (size_t i = 0; i < data.size(); i++) {
        data[i] = rand();
    }
}
