total = 0.0
i = 1

while i < 1000:
    j = 1
    row = 0.0
    while j < 1000:
        row = row + (i * j) / ((i + j) * (i + j) * (i * i + j * j))
        j += 1
    total = total + row
    i += 1

print(total)