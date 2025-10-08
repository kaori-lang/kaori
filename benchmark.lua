local iterations = 1000000
local max_fib = 40

local start = os.clock()  -- Lua's high-resolution timer

for i = 1, iterations do
    local a, b = 0, 1

    for j = 1, max_fib do
        local temp = a + b
        a = b
        b = temp
    end
end

local finish = os.clock()
print(string.format("Elapsed time: %.6f seconds", finish - start))
