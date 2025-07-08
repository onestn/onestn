def solution(phone_number):
    # 1. 010 뿐 아니라 02라는 지역번호도 있으므로, 앞에서부터 *로 번호를 가리면 오류가 많을 것이다.
    # phone_number의 뒷 번호 4자리만 남기게.
    return '*' * (len(phone_number) - 4) + phone_number[-4:]


print(solution("01033334443"))
print(solution("027778888"))
