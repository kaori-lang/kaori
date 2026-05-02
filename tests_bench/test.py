

def foo(): 
    return 10

def foo():
    def foo(): 
        return 10
    
    def foo(): 
        return 11
    

    foo()
    return 1

print(foo())