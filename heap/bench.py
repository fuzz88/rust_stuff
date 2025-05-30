import timeit
import random
from copy import deepcopy

# Comparison counter
class ComparisonCounter:
    def __init__(self):
        self.count = 0

    def lt(self, a, b):
        self.count += 1
        return a < b

    def le(self, a, b):
        self.count += 1
        return a <= b

# Optimized siftup with inlined siftdown
def _siftup_with_siftdown_inlined(heap, pos, counter):
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

    # Inlined siftdown
    while pos > startpos:
        parentpos = (pos - 1) >> 1
        if counter.lt(newitem, heap[parentpos]):
            heap[pos] = heap[parentpos]
            pos = parentpos
        else:
            break
    heap[pos] = newitem

# Optimized alternative siftup (no siftdown)
def _siftup_no_siftdown_optimized(heap, pos, counter):
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
    for i in range(n // 2 - 1, -1, -1):
        _siftup_with_siftdown_inlined(lst, i, counter)
    return counter.count

def heapify_no_siftdown(lst):
    counter = ComparisonCounter()
    n = len(lst)
    for i in range(n // 2 - 1, -1, -1):
        _siftup_no_siftdown_optimized(lst, i, counter)
    return counter.count

# Benchmarking
def benchmark():
    lst = [random.randint(0, 10000) for _ in range(1_000_000)]

    def timed_heapify_with():
        heapify_with_siftdown(lst.copy())

    def timed_heapify_no():
        heapify_no_siftdown(lst.copy())

    time_with = timeit.timeit(timed_heapify_with, number=10)
    time_no = timeit.timeit(timed_heapify_no, number=10)

    # Only one run for comparison count
    comparisons_with = heapify_with_siftdown(lst.copy())
    comparisons_no = heapify_no_siftdown(lst.copy())

    # Output
    print(f"With siftdown inlined: {time_with:.6f} seconds, Comparisons: {comparisons_with}")
    print(f"Without siftdown:      {time_no:.6f} seconds, Comparisons: {comparisons_no}")

if __name__ == "__main__":
    benchmark()

