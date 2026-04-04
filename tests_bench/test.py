import dis

def example():
    a = "a" + "".join("b" for ch in range(1000))




dis.dis(example)