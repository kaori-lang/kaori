import dis

class Foo:
    def __init__(self, a, b, c):
        self.a = a
        self.b = b
        self.c = c

def example():
    d = Foo(5, 6, 7)
    print(d)




dis.dis(example)