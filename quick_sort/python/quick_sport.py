#! /usr/bin/env python3

def quicksort(array):
    if len(array) < 2:
        return array
    else:
        pivot = array[0]
        less = [i for i in array[1:] if i <= pivot]

        greater = [i for i in array[1:] if i > pivot]
        print(f"{less} | {pivot} | {greater}")
        return quicksort(less) + [pivot] + quicksort(greater)

print (quicksort([10, 8, 1, 9, 3, 27, 46]))
