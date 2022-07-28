import inspect
from time import time
from picalc import calc

N = 100_000_000

def pypitest(n_terms: int) -> tuple:
    start_time = time()
    numerator = 4.0
    denominator = 1.0
    operation = 1.0
    pi = 0.0
    for _ in range(n_terms):
        pi += operation * (numerator / denominator)
        denominator += 2.0
        operation *= -1.0
    end_time = time()
    return (inspect.stack()[0][3], (pi, round((end_time - start_time), 2)))

def rustpitest(n_terms: int) -> tuple:
    start_time = time()
    pi = calc(N)
    end_time = time()
    return (inspect.stack()[0][3], (pi, round((end_time - start_time), 2)))

def print_results(results: tuple) -> None:
    print(f"Runtime: {results[0]}")
    results = results[1]
    print(f"pi = {results[0]}")
    print(f"Time elapsed: {results[1]} seconds")

if __name__ == "__main__":
    print_results(rustpitest(N))
    print_results(pypitest(N))

