local game = require 'type'
local csv  = require 'csv'
local srch = require 'search'

function total_cmp(dict)
    local search = srch:New(dict)

    local def_cmps = {}
    local bin_cmps = {}
    local seg_cmps = {}

    for i, v in ipairs(dict) do
        local d = search:default(v.game)
        local b = search:binary(v.game)
        local s = search:segment(v.game)

        table.insert(def_cmps, d[2])
        table.insert(bin_cmps, b[2])
        table.insert(seg_cmps, s[2])

        io.write(string.format("Done %d/%d\r", i, #dict))
    end

    print("Done")

    return  { def_cmps, bin_cmps, seg_cmps }
end

function dump_cmps(cmps, path)
    local f = io.open(path, 'w')

    if f == nil then
        error("Can't open file.")
    end

    print("len:", #cmps[1])

    for i = 1, #cmps[1] do
        f:write(string.format("%d,%d,%d\n", cmps[1][i], cmps[2][i], cmps[3][i]))
    end

    f:close()
end
