with open("image.ppm.txt", "r") as f:
    print(f.readline())
    print(f.readline())
    print(f.readline())
    print(f.readline())
    print(f.readline())
    a, b, c = f.readline().split(" ")
    a, b, c = int(a), int(b), int(c)
    assert 0 <= a <= 255 and 0 <= b <= 255 and 0 <= c <= 255
