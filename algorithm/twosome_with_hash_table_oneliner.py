def solution(numbers, target):
    return any(target - num in set(numbers) and target - num != num for num in numbers)


print(solution([4, 1, 9, 7, 5, 3, 16], 14))
