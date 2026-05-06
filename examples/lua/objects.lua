
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

