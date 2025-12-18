import time

from contextlib import contextmanager

@contextmanager
def timer(task_name="Task"):
    start = time.perf_counter()

    try:
        yield
    finally:
        elapsed = time.perf_counter() - start

        hours, remainder = divmod(elapsed, 3600)
        minutes, seconds = divmod(remainder, 60)
        
        print(f"Total Time: {int(hours):02}:{int(minutes):02}:{int(seconds):02}")


with timer('example'):
    time.sleep(2)
