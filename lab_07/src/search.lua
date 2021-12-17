local search = {}

search.__index = search

function search:New(h)
    local self = setmetatable({}, search)

    self.haystack = h
    self.segments = nil

    return self
end

function search:default(needle)
    local haystack = self.haystack

    local comp = 0

    for i, v in ipairs(haystack) do
        comp = comp + 1
        if v.game == needle then
            return {v, comp}
        end
    end

    return {nil, comp}
end

function search:binary(needle)
    local haystack = self.haystack
    local left = 1
    local right = #haystack

    local comp = 0

    while left <= right do
        local middle = math.floor((left + right) / 2)

        if (haystack[middle].game == needle) then
            comp = comp + 1
            return {haystack[middle], comp}
        elseif haystack[middle].game < needle then
            comp = comp + 2
            left = middle + 1
        else
            comp = comp + 2
            right = middle - 1
        end
    end

    return {nil, comp}
end

local function letter_freq(haystack)
    local res = {}

    for i, v in ipairs(haystack) do
        if res[v.game:sub(1, 1)] == nil then
            res[v.game:sub(1, 1)] = { v }
        else
            table.insert(res[v.game:sub(1, 1)], v)
        end
    end

    return res
end

function search:segment(needle)
    local haystack = self.haystack

    if self.segments == nil then
        self.segments = letter_freq(haystack)
    end

    local letter = needle:sub(1, 1)
    local cmp = 0
    for k, v in pairs(self.segments) do
        cmp = cmp + 1
        if k == letter then
            local value = search:New(v)
            local res = value:binary(needle)
            
            res[2] = res[2] + cmp
            return res
        end
    end

    return nil
end

return search
