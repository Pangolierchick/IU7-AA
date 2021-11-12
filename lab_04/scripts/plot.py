import matplotlib.pyplot as plt
import json

def get_mean_benchmarks(path:str):
    with open(path, 'r') as f:
        bench_json = json.load(f)
    
    clear_benches = []

    for bench in bench_json['benchmarks']:
        try:
            if bench['aggregate_name'] == 'median':
                clear_benches.append(bench)
        except KeyError:
            pass
    
    return clear_benches

def get_xy_classic_mean(benches: dict) -> tuple:
    x = []
    y = []

    for b in benches:
        s = b['run_name'].split('/')

        name = s[0]
        size = s[1] 

        if name == 'classic_mean':
            x.append(int(size))
            y.append(float(b['real_time']))
        
    return (x, y)

def get_xy_threaded_mean(benches: dict) -> tuple:
    b_by_thread = {2 : [[], []], 4 : [[], []], 8: [[], []], 16: [[], []], 32: [[], []]}

    for b in benches:
        s = b['run_name'].split('/')

        name = s[0]
        size = int(s[1])

        if name == 'parallel_mean':
            t = int(s[2])

            b_by_thread[t][0].append(size) # pushing x
            b_by_thread[t][1].append(float(b['real_time'])) # pushing y
    
    return b_by_thread

def main():
    b = get_mean_benchmarks('/Users/kirill/Study/third_course/IU7-AA/lab_04/build/bench/benchmark.json')

    bc = get_xy_classic_mean(b)
    bt = get_xy_threaded_mean(b)

    fig, ax = plt.subplots()

    ax.plot(bc[0], bc[1], label='Последовательный алгоритм')

    for k, v in bt.items():
        ax.plot(v[0], v[1], label=f'Параллельный алгоритм (потоков: {k})')
    
    ax.legend()
    ax.grid()

    ax.set_xlabel('Размер квадратной матрицы')
    ax.set_ylabel('Время работы алгоритмы (нс)')


    plt.show()
    

if __name__ == '__main__':
    main()
