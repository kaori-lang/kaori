
local start = os.clock()
local iterations = 10
local max_fib = 10


for i = 1, iterations do
    local a, b = 0, 1
    for j = 1, max_fib do
        local temp = a + b
        a = b
        b = temp
    end
end

local elapsed = os.clock() - start
print(elapsed * 1000)

