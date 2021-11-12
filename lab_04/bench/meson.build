incdir = include_directories('../src/')
google_bench = dependency('benchmark')

if google_bench.found()
    benchmark_exe = executable('matrix_benchmark', ['./benchmark.cpp', '../src/matrix.cpp'], dependencies: [google_bench], include_directories : incdir)
endif

