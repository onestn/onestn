import pyarrow as pa

arr = pa.array(["Arrow", None, "io", "Rust", "100", "1000000000000000000000", "sdsssssssssssssss"])

print("type:", arr.type)
print("length:", len(arr))
print("null count:", arr.null_count)
print("---")

for i, buf in enumerate(arr.buffers()):
    print(f"buffer[{i}]:", buf, "| size:", buf.size if buf else None, "bytes")
