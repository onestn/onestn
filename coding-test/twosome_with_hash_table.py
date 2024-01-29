def solution(numbers, target):
    # Make set using numbers
    number_set = set(numbers)

    for number in numbers:
        diff_target = target - number

        if diff_target in number_set and diff_target != number:
            return True

    return False


# print(solution([4, 1, 9, 7, 5, 3, 16], 14))
print(solution([2, 1, 5, 7], 4))
