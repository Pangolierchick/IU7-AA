#pragma once

#include "matrix.h"
#include <utility>
#include <vector>

#include <cstdio>

class Colony {
public:
    Colony(Matrix<size_t>& m) : _a(default_a), _b(default_b), _q(default_q), _p(default_p), ph(m.getrows(), m.getcols()) {
        map = m.clone();

        for (size_t i = 0; i < ph.getrows(); i++) {
            for (size_t j = 0; j < ph.getcols(); j++) {
                ph(i, j) = 0.5f;
            }
        }
    }

    Matrix<size_t> copy_map() { return map.clone(); }

    float a() const { return _a; }
    float b() const { return _b; }
    float q() const { return _q; }
    float p() const { return _p; }

    Matrix<size_t>& getmap() { return map; }
    Matrix<float>& getph() { return ph; }

    std::vector<size_t> simulate(size_t days);

private:
    Matrix<size_t> map;
    Matrix<float> ph;
    float _a;
    float _b;
    float _q;
    float _p;

    constexpr static float default_a = 06.0;
    constexpr static float default_b = 04.0;
    constexpr static float default_q = 20.0;
    constexpr static float default_p = 0.6;
};
