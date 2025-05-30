import timeit
import random
from copy import deepcopy

# Comparison counter class
class ComparisonCounter:
    def __init__(self):
        self.count = 0

    def lt(self, a, b):
        self.count += 1
        return a < b

    def le(self, a, b):
        self.count += 1
        return a <= b

# Reference implementation (with _siftdown)
def _siftup_with_siftdown(heap, pos, counter):
    endpos = len(heap)
    startpos = pos
    newitem = heap[pos]
    childpos = 2 * pos + 1
    while childpos < endpos:
        rightpos = childpos + 1
        if rightpos < endpos and not counter.lt(heap[childpos], heap[rightpos]):
            childpos = rightpos
        heap[pos] = heap[childpos]
        pos = childpos
        childpos = 2 * pos + 1
    heap[pos] = newitem
    _siftdown(heap, startpos, pos, counter)

def _siftdown(heap, startpos, pos, counter):
    newitem = heap[pos]
    while pos > startpos:
        parentpos = (pos - 1) >> 1
        parent = heap[parentpos]
        if counter.lt(newitem, parent):
            heap[pos] = parent
            pos = parentpos
            continue
        break
    heap[pos] = newitem

# Alternative implementation (early break, no _siftdown)
def _siftup_no_siftdown(heap, pos, counter):
    endpos = len(heap)
    newitem = heap[pos]
    childpos = 2 * pos + 1
    while childpos < endpos:
        rightpos = childpos + 1
        if rightpos < endpos and counter.lt(heap[rightpos], heap[childpos]):
            childpos = rightpos
        if counter.le(newitem, heap[childpos]):
            break
        heap[pos] = heap[childpos]
        pos = childpos
        childpos = 2 * pos + 1
    heap[pos] = newitem

# Heapify functions
def heapify_with_siftdown(lst):
    counter = ComparisonCounter()
    n = len(lst)
    for i in reversed(range(n // 2)):
        _siftup_with_siftdown(lst, i, counter)
    return counter.count

def heapify_no_siftdown(lst):
    counter = ComparisonCounter()
    n = len(lst)
    for i in reversed(range(n // 2)):
        _siftup_no_siftdown(lst, i, counter)
    return counter.count

# Benchmarking
def benchmark():
    lst = [random.randint(0, 10000) for _ in range(1_000_000)]

    # Deepcopy for timing and comparison
    lst1 = deepcopy(lst)
    lst2 = deepcopy(lst)

    # Measure time
    def timed_heapify_with():
        lst_copy = deepcopy(lst1)
        heapify_with_siftdown(lst_copy)

    def timed_heapify_no():
        lst_copy = deepcopy(lst2)
        heapify_no_siftdown(lst_copy)

    time_with = timeit.timeit(timed_heapify_with, number=10)
    time_no = timeit.timeit(timed_heapify_no, number=10)

    # Measure comparisons (not averaged over 10 runs to avoid duplication)
    comparisons_with = heapify_with_siftdown(deepcopy(lst1))
    comparisons_no = heapify_no_siftdown(deepcopy(lst2))

    # Output
    print("Heapify Performance Comparison (10 runs on list of 10,000 items):")
    print(f"With _siftdown:     {time_with:.6f} seconds, Comparisons: {comparisons_with}")
    print(f"Without _siftdown:  {time_no:.6f} seconds, Comparisons: {comparisons_no}")
    if time_with < time_no:
        print("Result: Reference implementation with _siftdown is faster.")
    else:
        print("Result: Alternative implementation without _siftdown is faster.")

if __name__ == "__main__":
    benchmark()

