# list[start:end[:step]]

nums = [1, 2, 3, 4, 5, 6, 7, 8, 9]
ex_list = [10, 20, 30, 40, 50]

# 1. List를 역순으로 슬라이싱하기
reversed_arr = ex_list[::-1]
print(reversed_arr)

# 2. List의 마지막 세 요소 가져오기
last_three = ex_list[-3:]
print(last_three)

exclude_last_three = nums[:-3]
print(nums[:-3])  # 마지막 세 요소를 제외한 나머지를 가져오기

desc_sort_last_three = nums[-3:-1:-1] # start가 end보다 작기 때문에 []을 반환
print(desc_sort_last_three)

desc_sort_last_three = nums[-1:-4:-1]  # 원하는 대로 출력되었다.
print(desc_sort_last_three)

print('ABCD'[::2])
