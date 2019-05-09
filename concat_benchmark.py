import libmyrustlib

iterations = 1000

def concat_basic(iterations):
    output = ""
    for num in range(iterations):
        output += str(num)
    return output

def concat_join(iterations):
    output = []
    for num in range(iterations):
        output.append(str(num))
    return "".join(output)

def concat_comprehensions(iterations):
    return "".join([str(num) for num in range(iterations)])

def test_concat_basic(benchmark):
    benchmark(concat_basic, iterations)

def test_concat_join(benchmark):
    benchmark(concat_join, iterations)

def test_concat_comprehensions(benchmark):
    benchmark(concat_comprehensions, iterations)

def test_concat_rust(benchmark):
    benchmark(libmyrustlib.rust_concat, iterations)