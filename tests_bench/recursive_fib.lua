local function fib(x)
    if x == 0 then
        return 0
    elseif x == 1 then
        return 1
    else
        return fib(x - 1) + fib(x - 2)
    end
end


fib(40)
