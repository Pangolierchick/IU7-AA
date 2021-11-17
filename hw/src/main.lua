function crc32(buf)
    local crc = 0xFFFFFFFF                          -- (1)
    local table = {}                                -- (2)
    local base<const> = 0xEDB88320                  -- (3)
    local rem                                       -- (4)
    local c                                         -- (5)

    -- calculate CRC-table
    for i = 0, 0xFF do                              
        rem = i                                     -- (6)
        for j = 1, 8 do    
            local bit = rem & 1                     -- (7)
            local even = bit == 1                   -- (8)
            
            rem = rem >> 1                          -- (9)

            if (even) then                          -- (10)
                rem = rem ~ base                    -- (11)
            end
        end

        table[i] = rem                              -- (12)
    end
   
    for x = 1, #buf do                              
        c = buf[x]                                  -- (13)
        local crc_shifted = crc >> 8                -- (14)
        local crc_byte = crc & 0xFF                 -- (15)
        crc = (crc_shifted) ~ table[crc_byte ~ c]   -- (16)
    end

    return crc ~ 0xFFFFFFFF                         -- (17)
end

function main()
    local str = "Kir"
    local b = {}

    for i = 1, #str do
        b[i] = str:byte(i)
    end

    print(string.format("CRC 32 of %s: %d", str, crc32(b))) -- d8eec8b3
end

main()
