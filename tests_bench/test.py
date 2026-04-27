import time

def foo():

    for i in range(10):
        yield i



print(foo().__next__())
print(foo().__next__())