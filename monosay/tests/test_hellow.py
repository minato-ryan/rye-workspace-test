from monosay import hello


def test_hello():
    assert hello("1") == "hello 1"


if __name__ == "__main__":
    test_hello()
