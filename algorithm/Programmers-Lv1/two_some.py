def solution(numbers):

    res_list = []
    for i in range(len(numbers)):
        for j in range(len(numbers) - i):

            res_list.append(numbers[i] + numbers[i + j])
    
    return list(set(res_list))


print(solution([2, 1, 3, 4, 1]))
# print(solution([5, 0, 2, 7]))
