function crc32(buf, size)
    local crc = 0xFFFFFFFF                          -- (1)
    local table = {}                                -- (2)
    local rem                                       -- (4)
    local c                                         -- (5)

    -- calculate CRC-table
    for i = 0, 0xFF do                              -- (6)
        rem = i                                     -- (7)
        for j = 1, 8 do                             -- (8) 
            local c = (rem & 1) == 1                -- (9)
            
            rem = rem >> 1                          -- (10)

            if (c) then                             -- (11)
                rem = rem ~ 0xEDB88320              -- (12)
            end
        end

        table[i] = rem                              -- (13)
    end
   
    for x = 1, size do                              --(14)
        c = buf[x]                                  -- (15)
        crc = (crc >> 8) ~ table[(crc & 0xFF) ~ c]  --(16)
    end

    return crc ~ 0xFFFFFFFF                         -- (17)
end

function main()
    local str = "Kirill is doing this homework"
    local b = {}

    for i = 1, #str do
        b[i] = str:byte(i)
    end

    print(string.format("CRC 32: %x", crc32(b, #str))) -- d8eec8b3
end

main()
