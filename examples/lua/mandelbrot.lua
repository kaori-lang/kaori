local width = 800
local height = 800
local max_iteration = 1000
local x_min = -2.5
local x_max = 1.0
local y_min = -1.25
local y_max = 1.25

for y = 0, height - 1 do
    for x = 0, width - 1 do
        local cx = x_min + (x * (x_max - x_min) / width)
        local cy = y_min + (y * (y_max - y_min) / height)
        local zx = 0.0
        local zy = 0.0
        local iteration = 0
        while (zx * zx + zy * zy < 4.0) and (iteration < max_iteration) do
            local xtemp = zx * zx - zy * zy + cx
            zy = 2.0 * zx * zy + cy
            zx = xtemp
            iteration = iteration + 1
        end
    end
end
