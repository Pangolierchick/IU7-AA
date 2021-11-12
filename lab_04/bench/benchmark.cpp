#include <benchmark/benchmark.h>
#include "matrix.h"

static void classic_mean(benchmark::State& state) {
    for (auto _ : state) {
        state.PauseTiming();
        auto s = state.range(0);
        Matrix m(s, s);
        
        m.fill_rand();

        state.ResumeTiming();

        m.rows_mean();
    }
}

BENCHMARK(classic_mean)->RangeMultiplier(2)->Range(2<<4, 2<<14);

static void parallel_mean(benchmark::State& state) {
    for (auto _ : state) {
        state.PauseTiming();
        auto s = state.range(0);
        Matrix m(s, s);
        
        m.fill_rand();

        state.ResumeTiming();

        m.rows_mean_parallel(state.range(1));
    }
}

BENCHMARK(parallel_mean)->RangeMultiplier(2)->Ranges({{2<<4, 2<<14}, {2, 32}})->UseRealTime();

BENCHMARK_MAIN();
