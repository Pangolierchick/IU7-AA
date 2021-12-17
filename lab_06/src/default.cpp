#include "default.h"
#include <algorithm>
#include <iostream>
#include <iterator>

std::vector<size_t> default_salesman(Matrix<size_t> graph) {
    std::vector<size_t> res(graph.getcols());
    for (size_t s = 0; s < graph.getcols(); s++) {
        std::vector<size_t> vertex;
        for (size_t i = 0; i < graph.getcols(); i++)
            if (i != s)
                vertex.push_back(i);

        size_t min_path = std::string::npos;
        do {
            size_t current_pathweight = 0;

            size_t k = s;
            for (size_t i = 0; i < vertex.size(); i++) {
                current_pathweight += graph(k, vertex[i]);
                k = vertex[i];
            }
            current_pathweight += graph(k, s);

            min_path = std::min(min_path, current_pathweight);

        } while (
            std::next_permutation(vertex.begin(), vertex.end()));
        
        res[s] = min_path;
    }

    return res;
}
