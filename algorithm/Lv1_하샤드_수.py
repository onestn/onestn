# 하샤드 수
def solution(x):
    # Best Answer with One line.
    # return n % sum(int(x) for x in str(n)) == 0
    x_list = [int(i) for i in str(x)]
    sum_x = sum(x_list)

    if x % sum_x == 0:
        return True

    return False


print(solution(13))
