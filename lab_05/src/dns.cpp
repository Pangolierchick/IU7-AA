#include <vector>
#include <chrono>
#include <iostream>
#include <atomic>

#include "computer.h"
#include "queue.h"
#include "dns.h"
#include "pretty_print.h"

void dns::run_parallel() {
    std::vector<Computer> comps(ncomps);

    for (size_t i = 1; i <= comps.size(); i++) {
        comps[i - 1].set_id(i);
    }

    SafeQueue<Computer*> mb_man;
    SafeQueue<Computer*> cpu_man;
    SafeQueue<Computer*> gpu_man;

    std::atomic_bool mb_done  = false;
    std::atomic_bool cpu_done = false;

    auto qs = std::chrono::high_resolution_clock::now();


    auto mb_worker = [qs, &mb_done, &m = mb_man, &c = cpu_man]() {
        while (!m.empty()) {
            auto comp = m.dequeue();
            auto s = std::chrono::high_resolution_clock::now();
            comp->install_mb();
            auto e = std::chrono::high_resolution_clock::now();

            comp->mb_install_time_s = std::chrono::duration_cast<std::chrono::microseconds>(s - qs).count();
            comp->mb_install_time_e = std::chrono::duration_cast<std::chrono::microseconds>(e - qs).count();

            c.enqueue(comp);
        }

        mb_done = true;   
    };

    auto cpu_worker = [qs, &cpu_done, &mb_done, &c = cpu_man, &g = gpu_man]() {
        while (1) {
            if (!c.empty()) {
                auto comp = c.dequeue();
                auto s = std::chrono::high_resolution_clock::now();
                comp->install_cpu();
                auto e = std::chrono::high_resolution_clock::now();

                comp->cpu_install_time_s = std::chrono::duration_cast<std::chrono::microseconds>(s - qs).count();
                comp->cpu_install_time_e = std::chrono::duration_cast<std::chrono::microseconds>(e - qs).count();

                g.enqueue(comp);
            } else if (mb_done) {
                break;
            }
        }

        cpu_done = true;
    };

    auto gpu_worker = [qs, &cpu_done, &mb_done, &g = gpu_man]() {
        while (1) {
            if (!g.empty()) {
                auto comp = g.dequeue();
                auto s = std::chrono::high_resolution_clock::now();
                comp->install_gpu();
                auto e = std::chrono::high_resolution_clock::now();

                comp->gpu_install_time_s = std::chrono::duration_cast<std::chrono::microseconds>(s - qs).count();
                comp->gpu_install_time_e = std::chrono::duration_cast<std::chrono::microseconds>(e - qs).count();
            } else if (cpu_done && mb_done){
                break;
            }
        }
    };


    for (auto &&c : comps) {
        mb_man.enqueue(&c);
    }

    qs = std::chrono::high_resolution_clock::now();

    auto mb_t  = std::thread(mb_worker);
    auto cpu_t = std::thread(cpu_worker);
    auto gpu_t = std::thread(gpu_worker);


    mb_t.join();
    cpu_t.join();
    gpu_t.join();

    auto qe = std::chrono::high_resolution_clock::now();

    std::cout << "End time: " << std::chrono::duration_cast<std::chrono::microseconds>(qe - qs).count() << "\n";

    print_comps(comps);
}


void dns::run() {
    std::vector<Computer> comps(ncomps);

    for (size_t i = 1; i <= comps.size(); i++) {
        comps[i - 1].set_id(i);
    }

    auto qs = std::chrono::high_resolution_clock::now();

    for (size_t i = 0; i < comps.size(); i++) {
        auto s = std::chrono::high_resolution_clock::now();
        comps[i].install_mb();
        auto e = std::chrono::high_resolution_clock::now();

        comps[i].mb_install_time_s = std::chrono::duration_cast<std::chrono::microseconds>(s - qs).count();
        comps[i].mb_install_time_e = std::chrono::duration_cast<std::chrono::microseconds>(e - qs).count();

        s = std::chrono::high_resolution_clock::now();
        comps[i].install_cpu();
        e = std::chrono::high_resolution_clock::now();

        comps[i].cpu_install_time_s = std::chrono::duration_cast<std::chrono::microseconds>(s - qs).count();
        comps[i].cpu_install_time_e = std::chrono::duration_cast<std::chrono::microseconds>(e - qs).count();

        s = std::chrono::high_resolution_clock::now();
        comps[i].install_gpu();
        e = std::chrono::high_resolution_clock::now();

        comps[i].gpu_install_time_s = std::chrono::duration_cast<std::chrono::microseconds>(s - qs).count();
        comps[i].gpu_install_time_e = std::chrono::duration_cast<std::chrono::microseconds>(e - qs).count();
    }

    auto qe = std::chrono::high_resolution_clock::now();

    std::cout << "End time: " << std::chrono::duration_cast<std::chrono::microseconds>(qe - qs).count() << "\n";

    print_comps(comps);
}
