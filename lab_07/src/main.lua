local s = require 'split'
local csv = require 'csv'
local game = require 'type'
local search = require 'search'
local bench = require 'benchmark'
local cmp = require 'cmp_count'

function main()
    data, err = csv.read('../data/data1.csv', ';')

    local dict = {}

    for i, v in ipairs(data) do
        local g = Game:New(v[1], v[2])

        table.insert(dict, g)
    end

    io.write("Ipnut game name: ")
    local g_name = io.stdin:read('l')

    io.write('Benchmark? (y/n): ')

    local y_n_debug = io.stdin:read('l')
    local finder = search:New(dict)

    if y_n_debug == 'y' then
        local res = total_cmp(dict)

        dump_cmps(res, '../data/cmps.csv')

        print(string.format("Default: %f", bench.cpu_run(function ()
            finder:default(g_name)
        end, 1000, 'ns')))
        print(string.format("Binary:  %f", bench.cpu_run(function ()
            finder:binary(g_name)
        end, 1000, 'ns')))
        print(string.format("Segment: %f", bench.cpu_run(function ()
            finder:segment(g_name)
        end, 1000, 'ns')))
    else
        local sd = finder:default(g_name) or 'not found'
        local sb = finder:binary(g_name)  or 'not found'
        local ss = finder:segment(g_name) or 'not found'

        print('Default:', sd[1], sd[2])
        print('Binary: ', sb[1], sb[2])
        print('Segment:', ss[1], ss[2])
    end
end

main()
