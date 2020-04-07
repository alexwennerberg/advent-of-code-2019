import itertools


def chunked_iterable(iterable, size):
    it = iter(iterable)
    while True:
        chunk = tuple(itertools.islice(it, size))
        if not chunk:
            break
        yield chunk


with open("input/input.txt") as f:
    smallest_layer = 1
    minim = 25 * 6
    tmp = None
    a = chunked_iterable(f.read(), minim)
    for i in a:
        if i.count("0") < minim and i.count("0") != 0:
            minim = i.count("0")
            tmp = i
    print(tmp)
    print(tmp.count("1") * tmp.count("2"))
