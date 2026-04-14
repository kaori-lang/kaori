local function fib(n)
    if n < 2 then
        return n
    end
    return fib(n - 1) + fib(n - 2)
end

local start = os.clock()
fib(30)

local elapsed = (os.clock() - start) * 1000
print(elapsed)
