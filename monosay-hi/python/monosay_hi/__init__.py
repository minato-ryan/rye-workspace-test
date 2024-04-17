from monosay_hi._lowlevel import hello

def say_hi():
    print(hello("hi"))
    return hello("hi")

__all__ = ["hello"]