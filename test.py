result = 0
i = 0
k = 0

while i < 10:
    j = 0
    while j < 10:
        if i == 5 and j == 5:
            j += 1
        else:
            result += 1
            j += 1
        k += 1
    i += 1

print(result)