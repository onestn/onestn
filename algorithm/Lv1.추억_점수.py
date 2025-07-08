def solution(name, yearning, photo):

    # Make hash table that align with name, yearning
    yearning_table = {}
    for n, y in zip(name, yearning):
        yearning_table[n] = y
    
    scores = [[] for _ in photo]

    for photo_page, one_photo in enumerate(photo):
        for human in one_photo:
            try:
                scores[photo_page].append(yearning_table[human])
            except KeyError:
                print(f"{human} is not in photo")
                continue
        scores[photo_page] = sum(scores[photo_page])

    return scores


print(solution(
    name=["may", "kein", "kain", "radi"],
    yearning=[5, 10, 1, 3],
    photo=[
        ["may", "kein", "kain", "radi"],
        ["may", "kein", "brin", "deny"], 
        ["kon", "kain", "may", "coni"]
    ]))
