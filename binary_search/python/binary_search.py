#! /usr/bin/ python3
import time
import sys

args = sys.argv
db =  list(range(1, int(args[1]) + 1))

def binary_search(data, item):
    """
    O(log n)
    """
    st = time.time()
    low = 0
    high = len(data) - 1
    
    while low <= high:
        mid = (low + high)  // 2
        guess = data[mid]

        if guess == item:
            et = time.time()
            print(f"Result: {mid}")
            return f"Binary Search Exec: {(et - st) * 1000}ms"
        if guess > item:
            high = mid - 1
        else:
            low = mid + 1




def linear_search(data, item):
    """
    O(n)
    """
    st = time.time()
    for i in data:
        if data[i] == item:
            et = time.time()
            print(f"Result: {data[i] - 1}")
            return f"Linear Search Exec: {(et - st) * 1000}ms"
        else:
            i += 1

print(binary_search(db, int(args[1])))
print("--------------------")
print(linear_search(db, int(args[1])))

