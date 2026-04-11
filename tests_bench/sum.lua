
local start = os.clock()
local N = 100000000
local sum = 0


for i = 0, N - 1 do
    sum = sum + i
end

local elapsed = (os.clock() - start) * 1000
print(elapsed)