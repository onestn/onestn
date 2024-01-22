def solution(absolutes, signs):
    # 1. absolutes와 signs를 사용하여 각 수를 음, 양 기호로 판단될 수 있도록 만들자.
    # 2. 간단하게 만드는 것은, if문으로 분기시켜 값을 누적시키는 것이다.
    answer = 0
    for num, sign in zip(absolutes, signs):
        if sign == False:
            answer -= num
        else:
            answer += num

    return answer



solution([4, 7, 12], [True, False, True])
print()
solution([1, 2, 3], [False, False, True])
