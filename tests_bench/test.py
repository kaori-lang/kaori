

def foo(n):
    if n < 0:
        return

    bar(n - 1)

x = 5  # ...

def bar(n):
    if n < 0:
        return

    foo(n - 1)

