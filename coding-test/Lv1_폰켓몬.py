def solution(nums):
    max_select = len(nums) // 2
    unique_nums = set(nums)

    return max_select if max_select < len(unique_nums) else len(unique_nums)


print(solution([3, 1, 2, 3]))
print(solution([3, 3, 3, 2, 2, 4]))
print(solution([3, 3, 3, 2, 2, 2]))
