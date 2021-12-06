project('lab_06', ['cpp'], default_options: ['cpp_std=c++17'])

i = include_directories('./src')

src = [
    './src/main.cpp',
    './src/ant.cpp',
    './src/colony.cpp',
    './src/default.cpp'
]

exe = executable('main.out', src, cpp_args : ['-Wall', '-Wextra'], include_directories : i)

run_target('format', command: 'format.sh')

run_target('check', command: ['cppcheck', '--enable=all', './src/*'])
