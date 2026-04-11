import dis


class Cat:
    def __init__(self, age):
        self.age = age

def foo():
    d = Cat(1)

    return d.age

a = Cat(2)

foo() = 5