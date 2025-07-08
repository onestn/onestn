def solution(nums): # 초기 최대 부분배열 합과 현재 부분배열 합 설정
    max_sum = current_sum = nums[0]
    
    print('init:', max_sum)
    print()

    # 배열의 첫 번째 요소를 제외하고 순회
    for num in nums[1:]:
        print('num:', num)
        # 현재 부분배열 합에 현재 요소를 더하고,
        # 만약 현재 요소 자체가 더 크다면 현재 요소로 현재 합을 재설정
        current_sum = max(num, current_sum + num)
        print('current_sum:', current_sum)

        # 최대 부분배열 합 업데이트
        max_sum = max(max_sum, current_sum)
        print('max_sum:', max_sum)
        print()

    return max_sum


print(solution([-2, 1, -3, 4]))

# 입력: [-2,1,-3,4,-1,2,1,-5,4]
# 출력: 6
# 설명: 연속된 요소 [4,-1,2,1]의 합이 6으로 최대입니다.
# print(solution([-2,1,-3,4,-1,2,1,-5,4]))

# 입력: [1]
# 출력: 1
# 설명: 단일 요소 배열의 최대 합은 요소 자체입니다.
# print(solution([1]))
#
# 입력: [5,4,-1,7,8]
# 출력: 23
# 설명: 전체 배열의 합이 최대입니다.
# print(solution([5, 4, -1, 7, 8]))
