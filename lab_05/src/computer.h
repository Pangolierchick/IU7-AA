#pragma once

#include <chrono>
#include <thread>


class Computer {
public:
    Computer() = default;

    bool is_built() const { return mother_board && cpu && gpu; }
    
    bool mb_installed() const { return mother_board; }
    bool cpu_installed() const { return cpu; }
    bool gpu_installed() const { return gpu; }

    void set_id(size_t i) { id = i; }
    size_t get_id() const { return id; }

    void install_mb()  { std::this_thread::sleep_for(mb_installation_time);  mother_board = true; }
    void install_cpu() { std::this_thread::sleep_for(cpu_installation_time); cpu = true; }
    void install_gpu() { std::this_thread::sleep_for(gpu_installation_time); gpu = true; }


    long long mb_install_time_s;
    long long mb_install_time_e;
    long long cpu_install_time_s;
    long long cpu_install_time_e;
    long long gpu_install_time_s;
    long long gpu_install_time_e;

private:
    bool mother_board;
    bool cpu;
    bool gpu;

    size_t id;

    const std::chrono::microseconds mb_installation_time  {1500};
    const std::chrono::microseconds cpu_installation_time {1000};
    const std::chrono::microseconds gpu_installation_time {1100};
};
