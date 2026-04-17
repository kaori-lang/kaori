local function fib(n)
    if n < 2 then
        return n
    end
    return fib(n - 1) + fib(n - 2)
end


fib(30)

main <tests_bench/recursive_fib.lua:0,0> (6 instructions at 0000000000ae8cb0)
0+ params, 3 slots, 1 upvalue, 1 local, 0 constants, 1 function
        1       [1]     VARARGPREP      0
        2       [6]     CLOSURE         0 0     ; 0000000000ae8ef0
        3       [9]     MOVE            1 0
        4       [9]     LOADI           2 30
        5       [9]     CALL            1 2 1   ; 1 in 0 out
        6       [9]     RETURN          1 1 1k  ; 0 out

function <tests_bench/recursive_fib.lua:1,6> (15 instructions at 0000000000ae8ef0)
1 param, 4 slots, 1 upvalue, 1 local, 0 constants, 0 functions
        1       [2]     LTI             0 2 0
        2       [2]     JMP             1       ; to 4
        3       [3]     RETURN1         0
        4       [5]     GETUPVAL        1 0     ; fib
        5       [5]     ADDI            2 0 -1
        6       [5]     MMBINI          0 1 7 0 ; __sub
        7       [5]     CALL            1 2 2   ; 1 in 1 out
        8       [5]     GETUPVAL        2 0     ; fib
        9       [5]     ADDI            3 0 -2
        10      [5]     MMBINI          0 2 7 0 ; __sub
        11      [5]     CALL            2 2 2   ; 1 in 1 out
        12      [5]     ADD             1 1 2
        13      [5]     MMBIN           1 2 6   ; __add
        14      [5]     RETURN1         1
        15      [6]     RETURN0