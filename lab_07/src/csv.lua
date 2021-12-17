local csv = {}
local s = require 'split'

csv.__index = csv

-- returns data, err.
function csv.read(path, sep)
    local f = io.open(path, 'r')

    if f == nil then
        return {nil, 'Bad file path.'}
    end

    local header = f:read('l')

    local data = {}
    local len = 1

    for line in f:lines('l') do
        local t = string.split(line, sep)

        table.insert(data, t)
    end

    f:close()

    return data, nil
end

function csv.dump(data, path, sep)
    local f = io.open(path, 'w')

    sep = sep or ','

    if f == nil then
        return {nil, 'Bad file path.'}
    end
    
    for i, v in ipairs(data) do
        for j, k in pairs(v) do
            f:write(k)

            f:write(sep)
        end

        f:write('\n')
    end

    f:close()
end

return csv
