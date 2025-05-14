#!/usr/bin/env python

from typing import Generator

def fib(limit: int) -> Generator[int, None, None]:
    last = 0
    current = 1

    while True:
        new = last + current

        if new > limit:
            break

        yield new

        last = current
        current = new


def main():
    limit = 4_000_000
    result = sum(
        number for number in fib(limit)
        if number % 2 == 0
    )
    print(result)


if __name__ == "__main__":
    main()
