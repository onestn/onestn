def solution(n, a, b):
    return ((a - 1) ^ (b - 1)).bit_length()


# a, b에 -1을 하는 이유.
# : Computer Science에서의 제일 처음 수는 0이지만 제시된 문제에서는 1부터 시작하므로 이를 맞추기 위해 0으로 맞춰줍니다.

# XOR(^)을 사용하는 이유  

# bit_length()


print(solution(8, 4, 7))
