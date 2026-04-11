local start = os.clock()

local cat = {
    name = "meow",
    age = 5,
    color = "black"
}

local sum = 0
local N = 100000000


for i = 1, N do
    sum = sum + cat.age
end

local elapsed = os.clock() - start

print("sum:", sum)
print("time:", elapsed)