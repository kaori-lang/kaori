def Cat(age):
    def get_age():
        return age
        
        return get_age()
    
    def set_age(new_age):
        age = new_age

    return {"get_age": get_age, "set_age": set_age}



cat = Cat(5)
