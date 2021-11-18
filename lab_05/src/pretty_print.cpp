#include <iostream>
#include <cstdio>
#include <chrono>
#include "computer.h"
#include "pretty_print.h"
// id mb_s mb_e cpu_s cpu_e gpu_s gpu_e
void print_comps(const std::vector<Computer> &c) {
    using namespace std::chrono;
    puts("+-----+------------+-------------+------------+------------+------------+------------+");
    printf("| ID  |  mb start  |   mb end    | cpu start  |   cpu end  |  gpu start |  gpu end   |\n");
    puts("+-----+------------+-------------+------------+------------+------------+------------+");
    for (auto &&comp : c) {
        printf("|%5zu|%10dus| %10dus|%10dus|%10dus|%10dus|%10dus|\n", comp.get_id(), 
        comp.mb_install_time_s, 
        comp.mb_install_time_e,
        comp.cpu_install_time_s,
        comp.cpu_install_time_e,
        comp.gpu_install_time_s,
        comp.gpu_install_time_e);
    }

    puts("+-----+------------+-------------+------------+------------+------------+------------+");
}
