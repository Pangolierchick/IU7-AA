#include <vector>
#include <thread>
#include <iostream>
#include "queue.h"
#include "dns.h"
#include <queue>
// 191729
int main() {
    dns d1(5);
    dns d2(5);

    d1.run_parallel();
    d2.run();



    return 0;
}
