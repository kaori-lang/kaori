local function objects()
    local data = {
        a = 0,
        i = 5000000,
    }

    while data.i > 0 do
        data.a = data.a + data.i
        data.i = data.i - 1
    end

    return data
end

local x = objects()