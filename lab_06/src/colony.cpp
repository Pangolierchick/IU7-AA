#include "colony.h"
#include "ant.h"
#include <cstdio>
#include <unordered_map>

std::vector<size_t> Colony::simulate(size_t days) {
    std::vector<size_t> p(map.getrows());
    std::fill(p.begin(), p.end(), std::string::npos);

    for (size_t i = 0; i < days; i++) {
        for (size_t j = 0; j < map.getrows(); j++) {
            Ant ant(*this, 0);

            ant.move();

            auto cur_p = ant.distance();

            p[j] = std::min(p[j], cur_p);
        }
    }

    return p;
}
