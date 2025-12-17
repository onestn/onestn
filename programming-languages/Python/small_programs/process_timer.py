import time


start_time = time.perf_counter()

# change to real logic
time.sleep(3)

end_time = time.perf_counter()

duration = end_time - start_time
hours, remainder = divmod(duration, 3600)
minutes, seconds = divmod(remainder, 60)

print(f"Total Time: {int(hours):02}:{int(minutes):02}:{int(seconds):02}")
