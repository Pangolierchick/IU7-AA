#include "ant.h"
#include "matrix.hpp"
#include <cmath>
#include <cstdio>
#include <iostream>
#include <random>

void Ant::step_back() {
    travel(start);
}

void Ant::move() {
    for (;;) {
        auto p = getprob();
        auto w = find_way(p);

        if (w == std::string::npos) {
            step_back();
            break;
        }

        travel(w);
        update_phero();
    }
}

std::vector<double> Ant::getprob() {
    std::vector<double> prob;
    double sum = 0;

    // std::cout << "=================================\n";

    // std::cout << "Position: " << pos << "\n";

    // to_visit.print();

    for (size_t i = 0; i < to_visit.getcols(); i++) {
        auto dist = to_visit(pos, i);
        if (dist > 0) {
            double p1 = pow(1.0 / (double)dist, colony->a());
            double p2 = pow(colony->getph()(pos, i), colony->b());

            double p_t = p1 * p2;
            sum += p_t;

            prob.push_back(p_t);
        } else {
            prob.push_back(0.0);
        }
    }

    for (auto& p : prob) {
        p /= sum;
    }

    return prob;
}

size_t Ant::distance() {
    size_t d = 0;

    for (size_t i = 0; i < visited.getrows(); i++) {
        for (size_t j = 0; j < visited.getcols(); j++) {
            if (visited(i, j)) {
                d += colony->getmap()(i, j);
            }
        }
    }

    return d;
}

void Ant::update_phero() {
    double dt = 0;

    for (size_t i = 0; i < colony->getph().getrows(); i++) {
        for (size_t j = 0; j < colony->getph().getcols(); j++) {
            if (colony->getmap()(i, j) != 0) {
                if (visited(i, j)) {
                    dt = colony->q() / (double)colony->getmap()(i, j);
                } else {
                    dt = 0;
                }

                colony->getph()(i, j) = (1 - colony->p()) * colony->getph()(i, j) + dt;
            }

            if (colony->getph()(i, j) <= 0.0) {
                colony->getph()(i, j) = 0.1;
            }
        }
    }
}

void Ant::travel(size_t p) {
    for (size_t i = 0; i < to_visit.getrows(); i++) {
        to_visit(i, pos) = 0;
    }

    to_visit(p, pos) = 0;

    visited(pos, p) = 1;

    pos = p;
}

size_t Ant::find_way(std::vector<double>& probs) {
    std::random_device dev;
    std::mt19937 rng(dev());
    std::uniform_real_distribution<> dist(0.0, 1.0);

    double sum = std::accumulate(probs.begin(), probs.end(), 0.0);

    double p = dist(rng);
    double pn = p * sum;

    sum = 0;

    for (size_t i = 0; i < probs.size(); i++) {
        if (pn > sum && pn < sum + probs[i])
            return i;

        sum += probs[i];
    }

    return std::string::npos;
}
