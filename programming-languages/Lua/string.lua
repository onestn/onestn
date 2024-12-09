a = "one string"
b = string.gsub(a, "one", "another")
print(a)
print(b)

-- # Operator
print(#a)
print(#"good\0bye")


-- 구간 문자열
page = [[
<html>
<head>
    <title>An HTML Page</title>
<head>
<body>
    <a href="http://www.lua.org">Lua</a>
</body>
</html>
]]

write(page)
