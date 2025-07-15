def get_number(a, b: int, c: int = 10) -> int:
    print("[START] get_number")
    print(f"a: {a}")
    print(f"b: {b}")
    print(f"c: {c}")
    
    result_number = a + b + c
    print(f"Result: {result_number}")
    print("[END] get_number")

    return result_number

print("[Result(1)]", get_number(10, 20, 100))

print("[Result(2)]", get_number(a=10, b=20))

print("[Result(3)]", get_number(a=10, b=20, c=100))

