

x = 10
def foo():

    def bar():
        def foo_bar():
            print(x)

        foo_bar()

    bar()

foo()