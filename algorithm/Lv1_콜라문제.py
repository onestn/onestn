def solution(a, b, n):
    
    new_coke_cnt = 0

    while n >= a: # THINK: while을 선택한 것이 나은 방법일까?
        # 새로 받은 콜라병 갯수 구하기
        get_new_coke_cnt = n // a * b

        # 새로 받은 콜라병을 누적하기
        new_coke_cnt += get_new_coke_cnt

        remain_bottle_cnt = n % a

        n = remain_bottle_cnt + get_new_coke_cnt

    return new_coke_cnt


print(solution(3, 1, 20))
