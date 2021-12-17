local bench = {}

function bench.cpu_run(func, runs, type)
    local time_table = {
        s = 1,
        ms = 1e+3,
        us = 1e+6,
        ns = 1e+9
    }

    local time = time_table[type] or time_table.s

    local r = runs or 20
    
    local s = os.clock()

    for i = 1, r do
        func()
    end

    local e = os.clock()

    return ((e - s) / r) * time
end

return bench
