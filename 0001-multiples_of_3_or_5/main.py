#!/usr/bin/env python

def main(limit: int) -> int:
    return sum(
        number for number in range(limit)
        if number % 3 == 0 or number % 5 == 0
    )


if __name__ == "__main__":
    print(main(1000))
