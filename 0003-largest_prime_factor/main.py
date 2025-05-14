#!/usr/bin/env python

from functools import lru_cache
from math import sqrt
from typing import Generator


@lru_cache
def is_prime(number: int) -> bool:
    if number < 2:
        return False

    for i in range(2, number - 1):
        if number % i == 0:
            return False

    return True


def gen_primes(limit: int) -> Generator[int, None, None]:
    for number in range(limit):
        if is_prime(number):
            yield number


def main():
    number = 600851475143
    limit = int(sqrt(number))

    for prime in gen_primes(limit):
        if number % prime == 0:
            number /= prime

        if number == 1:
            print(prime)
            break


if __name__ == "__main__":
    main()
