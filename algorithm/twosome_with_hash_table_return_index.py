def solution(numbers, target):

    # make hash table using dict
    table = {}
    for idx, number in enumerate(numbers):
        table[number] = idx
    
    for idx, number in enumerate(numbers):
        matching_num = target - number

        print(idx, number)
        print("matching_num:", matching_num)
        
        print((matching_num in table), "and", (idx != table.get(matching_num)))

        if (matching_num in table) and (idx != table.get(matching_num)):
            return [idx, table.get(matching_num)]


print(solution([2, 7, 11, 15], target=9))  # [0, 1]
print(solution([3, 2, 4], target=6))  # [1, 2]
print(solution([3, 3], target=6))  # [0, 1]



# Example 1:
#
# Input: nums = [2,7,11,15], target = 9
# Output: [0,1]
# Output: Because nums[0] + nums[1] == 9, we return [0, 1].
# Example 2:
#
# Input: nums = [3,2,4], target = 6
# Output: [1,2]
# Example 3:
#
# Input: nums = [3,3], target = 6
# Output: [0,1]
