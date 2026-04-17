
local iterations = 1000000
local max_fib = 30


for i = 1, iterations do
    local a, b = 0, 1
    for j = 1, max_fib do
        local temp = a + b
        a = b
        b = temp
    end
end

        1       [1]     VARARGPREP      0
        2       [2]     LOADK           0 0     ; 1000000
        3       [3]     LOADI           1 30
        4       [6]     LOADI           2 1
        5       [6]     MOVE            3 0
        6       [6]     LOADI           4 1
        7       [6]     FORPREP         2 11    ; exit to 20
        8       [7]     LOADI           5 0
        9       [7]     LOADI           6 1
        10      [8]     LOADI           7 1
        11      [8]     MOVE            8 1
        12      [8]     LOADI           9 1
        13      [8]     FORPREP         7 4     ; exit to 19
        14      [9]     ADD             10 5 6
        15      [9]     MMBIN           5 6 6   ; __add
        16      [10]    MOVE            5 6
        17      [11]    MOVE            6 10
        18      [8]     FORLOOP         7 5     ; to 14
        19      [6]     FORLOOP         2 12    ; to 8
        20      [13]    RETURN          2 1 1   ; 0 out



