local point = {}




local p = { x = 150, y = 300 }

function add(p1, p2)
   local p3 = { x = p1.x + p2.x, y = p1.y + p2.y }

   return p3
end

local p1 = { x = 120, y = 130 }
local p2 = { x = 150, y = 40 }

local c = 30
c = c >> 1

print(c)
