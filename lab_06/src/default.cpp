#include "default.h"
#include <algorithm>
#include <iterator>
#include <iostream>

using matrix_t = std::vector<std::vector<size_t>>;

static void routes(size_t pos, const Matrix<size_t> &map, std::vector<size_t> &path, matrix_t &m) {
    path.push_back(pos);

    if (path.size() < map.getcols()) {
        for (size_t i = 0; i < map.getcols(); i++) {
            if (std::find(path.begin(), path.end(), i) == path.end()) {
                routes(i, map, path, m);
            }
        }
    } else {
        m.push_back(path);
    }
}


std::vector<size_t> default_salesman(const Matrix<size_t> &map) {
    std::vector<size_t> res(map.getcols());
    std::vector<size_t> path;

    for (size_t i = 0; i < map.getcols(); i++) {
        size_t sum = std::string::npos;
        size_t cur = 0;
        matrix_t rts;

        routes(i, map, path, rts);

        for (size_t j = 0; j < rts.size(); j++) {
            cur = 0;

            for (size_t k = 0; k < rts[j].size() - 1; k++) {
                cur += map(rts[j][k], rts[j][k + 1]);
            }

            sum = std::min(sum, cur);
        }

        res[i] = sum;
    }

    return res;
}
