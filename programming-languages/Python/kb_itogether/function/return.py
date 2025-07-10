# [return 키워드가 *없는* 함수]
#  - return 키워드가 없는 함수는 None을 반환해요.
def function_without_return():
    some_text = "Hello?"

# [return 키워드가 *있는* 함수]
# - some_text 변수에 담긴 값을 반환해요.
def function_with_return():
    some_text = "Hello?"
    return some_text


value_with_return = function_with_return()
print("Value with RETURN:")
print(value_with_return)

print()

value_without_return = function_without_return()
print("Value without RETURN:")
print(value_without_return)



