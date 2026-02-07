
a = 5

def foo():
    global a
    a *= 2

foo()


print(a)