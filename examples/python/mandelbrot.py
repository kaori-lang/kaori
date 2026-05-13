def mandelbrot():
    width = 800
    height = 800
    max_iteration = 1000
    x_min = -2.5
    x_max = 1.0
    y_min = -1.25
    y_max = 1.25

    for y in range(height):
        x = 0
        for x in range(width):
            cx = x_min + (x * (x_max - x_min) / width)
            cy = y_min + (y * (y_max - y_min) / height)
            zx = 0.0
            zy = 0.0
            iteration = 0
            while (zx * zx + zy * zy < 4.0) and (iteration < max_iteration):
                xtemp = zx * zx - zy * zy + cx
                zy = 2.0 * zx * zy + cy
                zx = xtemp
                iteration = iteration + 1

mandelbrot()