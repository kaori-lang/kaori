local start = os.clock()
local function fib(n)
    if n < 2 then
        return n
    end
    return fib(n - 1) + fib(n - 2)
end


fib(30)


local elapsed = os.clock() - start
print(elapsed * 1000) 