def simulate():
    # python has no hoisting, so we simulate by capturing the function reference
    # before it's "used" by deferring the call
    
    foo = None  # forward declaration
    
    z = foo     # z captures None, like capturing before hoisting resolves
    
    def foo():
        pass
    
    print(z)    # prints None

simulate()