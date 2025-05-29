import timeit
import random
from copy import deepcopy

# Reference implementation (with _siftdown)
def _siftup_with_siftdown(heap, pos):
    endpos = len(heap)
    startpos = pos
    newitem = heap[pos]
    childpos = 2 * pos + 1
    while childpos < endpos:
        rightpos = childpos + 1
        if rightpos < endpos and not heap[childpos] < heap[rightpos]:
            childpos = rightpos
        heap[pos] = heap[childpos]
        pos = childpos
        childpos = 2 * pos + 1
    heap[pos] = newitem
    _siftdown(heap, startpos, pos)

def _siftdown(heap, startpos, pos):
    newitem = heap[pos]
    while pos > startpos:
        parentpos = (pos - 1) >> 1
        parent = heap[parentpos]
        if newitem < parent:
            heap[pos] = parent
            pos = parentpos
            continue
        break
    heap[pos] = newitem

# Alternative implementation (early break, no _siftdown)
def _siftup_no_siftdown(heap, pos):
    endpos = len(heap)
    newitem = heap[pos]
    childpos = 2 * pos + 1
    while childpos < endpos:
        rightpos = childpos + 1
        if rightpos < endpos and heap[rightpos] < heap[childpos]:
            childpos = rightpos
        if newitem <= heap[childpos]:
            break
        heap[pos] = heap[childpos]
        pos = childpos
        childpos = 2 * pos + 1
    heap[pos] = newitem

# Heapify functions
def heapify_with_siftdown(lst):
    n = len(lst)
    for i in reversed(range(n // 2)):
        _siftup_with_siftdown(lst, i)

def heapify_no_siftdown(lst):
    n = len(lst)
    for i in reversed(range(n // 2)):
        _siftup_no_siftdown(lst, i)

# Benchmarking
def benchmark():
    setup_code = '''
from __main__ import heapify_with_siftdown, heapify_no_siftdown, deepcopy, random
lst = [random.randint(0, 10000) for _ in range(10000)]
lst1 = deepcopy(lst)
lst2 = deepcopy(lst)
'''

    time_with_siftdown = timeit.timeit('heapify_with_siftdown(lst1)', setup=setup_code, number=10)
    time_no_siftdown = timeit.timeit('heapify_no_siftdown(lst2)', setup=setup_code, number=10)

    print("Heapify Performance Comparison (10 runs on list of 10,000 items):")
    print(f"With _siftdown:     {time_with_siftdown:.6f} seconds")
    print(f"Without _siftdown:  {time_no_siftdown:.6f} seconds")
    if time_with_siftdown < time_no_siftdown:
        print("Result: Reference implementation with _siftdown is faster.")
    else:
        print("Result: Alternative implementation without _siftdown is faster.")

if __name__ == "__main__":
    benchmark()

