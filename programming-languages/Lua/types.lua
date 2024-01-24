-- Lua는 동적 타입 언어이다. Lua에서는 모든 값마다 타입이 있으므로, 변수를 선언할 때 타입을 지정하지 않는다.
type(nil) -- type() 함수는 인자로 받은 값의 타입을 문자열로 반환한다.

-- Lua에서는 8개의 기본 타입이 있다.
-- nil, boolean, number, string, userdata, function, thread, table

a = "a string!!"
print(type(a)) -- string

-- Lua에서는 함수를 1급 값으로 취급하기 때문에 아래와 같이 값처럼 다룰 수 있다.
a = print
print(type(a)) -- function

-- Lua에서 string은 immutable이다. 즉, string을 수정할 수 없다.
a = "one string"
b = string.gsub(a, "one", "another")
print(a) -- one string
print(b) -- another string

-- Lua에서는 길이 연산자(#)를 사용하여 string의 길이를 구할 수 있다.
print(#a) -- 10
print(#"good")
