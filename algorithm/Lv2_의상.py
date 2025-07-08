def solution(clothes):

    cloth_table = {}
    for item, category in clothes:
        if category not in cloth_table:
            cloth_table[category] = []
        cloth_table[category].append(item)

    print(cloth_table)
    {'headgear': ['yellow_hat', 'green_turban'], 
     'eyewear': ['blue _sunglasses']}




    return 0


print(solution([
    ["yellow_hat", "headgear"], 
    ["blue_sunglasses", "eyewear"], 
    ["green_turban", "headgear"]]))
