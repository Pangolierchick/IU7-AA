#pragma once

#include "colony.h"
#include "matrix.h"
#include <memory>
#include <vector>

class Ant {
public:
    Ant(Colony& c, size_t p) : start(p), pos(p), colony(std::make_shared<Colony>(c)) {
        to_visit = c.copy_map();
        visited = Matrix<int>(to_visit.getcols(), to_visit.getrows());
    }

    void move();
    void travel(size_t p);
    size_t distance();
    void update_phero();

private:
    Matrix<size_t> to_visit;
    Matrix<int> visited;
    size_t pos;
    size_t start;
    std::shared_ptr<Colony> colony;

    std::vector<float> getprob();
    size_t find_way(std::vector<float>& probs);

    void step_back();
};
