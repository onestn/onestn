function fact (n)
  if n == 0 then
    return 1
  else
    return n * fact(n - 1)
  end
end

print("Enter a number:")
a = io.read("*n")

if a < 0 then
  print("Number must be positive")
  return
end

print(fact(a))
