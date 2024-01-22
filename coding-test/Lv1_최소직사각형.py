def solution(sizes):

    min_size, max_size = [], []

    for size in sizes:
        sorted_size = sorted(size)

        min_size.append(sorted_size[0])
        max_size.append(sorted_size[1])

    max_min_size = max(min_size)
    max_max_size = max(max_size)

    return max_min_size * max_max_size


print(solution([[60, 50], [30, 70], [60, 30], [80, 40]]))
print(solution([[10, 7], [12, 3], [8, 15], [14, 7], [5, 15]]))
print(solution([[14, 4], [19, 6], [6, 16], [18, 7], [7, 11]]))
