example = {}

def object1():
    return example

object2 = example

object1()["x"] = 5
object2["y"] = 5

print(example)