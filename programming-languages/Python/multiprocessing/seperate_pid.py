import os

from multiprocessing import Process

def info(title):
    print(title)
    print(f"Module Name: {__name__}")
    print(f"Parent Process: {os.getppid()}")
    print(f"Process ID: {os.getpid()}")

def f(name):
    info("function f")
    print(f"Hello, {name}")

if __name__ == '__main__':
    info("main line")

    p = Process(target=f, args=('bob',))
    p.start()
    p.join()
