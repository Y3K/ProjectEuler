#!/usr/bin/env python


def greatest_common_divisor(a: int, b: int) -> int:
    while b != 0:
        a, b = b, a % b

    return a


def lowest_common_multiplier(a: int, b: int) -> int:
    return abs(a * b) // greatest_common_divisor(a, b)


def main():
    limit = 20
    sequence = list(range(1, limit))
    result = sequence[0]

    for number in sequence[1:]:
        result = lowest_common_multiplier(result, number)

    print(result)


if __name__ == "__main__":
    main()
