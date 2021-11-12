project('lab_04', ['cpp'], default_options: ['cpp_std=c++17'])

i = include_directories('./src')

subdir('bench')

exe = executable('main.out', ['./src/main.cpp', './src/matrix.cpp'], cpp_args : ['-Wall', '-Wextra'])
