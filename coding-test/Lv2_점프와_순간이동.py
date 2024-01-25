def solution(n):
    # 내 생각에는, 한 칸만 이동하고 그 이후는 모두 순간이동으로 처리하면 그 것이 가장 최소의 배터리를 사용하는 로직이 아닌가 싶다. -> 한번 검증해보자.

    # ! 순간이동: (현재까지 온 거리) * 2
    # ! 현재위치: 0, N: 도착지
    # ! 점프로 이동하는 것을 최소화하는 것이 이번 문제의 관건이다.
    # ! 연속 순간이동 가능

    # !!! 주어진 도착지 N을 무언가로 처리하는 것이 관건인 것으로 보인다.

    # 최소 배터리 사용량은 1이다. 왜냐하면 처음에 1칸을 이동해야 뭘 할 수 있기 때문이다.

    battery_usage = 1
    current_distance = 2 # 1칸 이동 후 무조건 순간이동 한 번 필요

    current_distance = current_distance * 2

    while True:
        previous_distance = current_distance
        current_distance = current_distance * 2

        if current_distance == n:
            return battery_usage
        elif current_distance > n:
            previous_distance += 1
            battery_usage += 1
            current_distance = previous_distance
            break


    print(f"n: {n}, current_distance: {current_distance}")

    return battery_usage


print(solution(5000))
