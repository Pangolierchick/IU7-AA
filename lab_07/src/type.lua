Game = {}

Game.__index = Game

function Game:New(n, c)
    assert(n ~= nil and c ~= nil, 'name and company must be provided')
    
    local self = setmetatable({}, Game)
    
    self.game = n
    self.company = c

    return self
end

function Game:__tostring()
    return self.game .. " : " .. self.company 
end
    
return Game
