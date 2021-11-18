#pragma once

class dns {
public:

    dns(size_t nc) : ncomps(nc) {}

    void run();
    void run_parallel();
private:
    size_t ncomps;
};
