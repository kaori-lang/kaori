local result = 0
local i = 0

while i < 10 do
    local j = 0
    while j < 10 do
        local k = 0
        while k < 10 do
            if k == 3 then
                k = k + 1
                goto continue_k
            end
            if j == 7 and k == 8 then
                break
            end
            if i == 5 and j == 5 and k == 5 then
                break
            end
            result = result + 1
            k = k + 1
            ::continue_k::
        end
        if i == 5 and j == 5 then
            break
        end
        j = j + 1
    end
    i = i + 1
end

print(result)