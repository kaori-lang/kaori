import dis


a = 5
b = 6.0

print(type(a), type(b),  type(a + b))


d = { 1: 'bar'}

print(d[1.0])

d = { 1.0: 'bar'}

print(d[1])