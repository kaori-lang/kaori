local iterations = 1000000
local max_fib = 40

for i = 0, iterations - 1 do
    local a, b = 0, 1

    for j = 0, max_fib - 1 do
        local temp = a + b
        a = b
        b = temp
    end
end
