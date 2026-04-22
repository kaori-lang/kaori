def Cat(age):
    state = {"age": age}
    

    def foo():
        bar = 5

        def get_age():
            return state["age"] + bar
        
        return get_age()
    
    def set_age(new_age):
        state["age"] = new_age

    return {"get_age": foo, "set_age": set_age}



cat = Cat(5)

print(cat["get_age"]())