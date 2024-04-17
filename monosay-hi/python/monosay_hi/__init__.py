from monosay import hello

def say_hi():
    print(hello("hi"))
    return hello("hi")

__all__ = ["hello"]