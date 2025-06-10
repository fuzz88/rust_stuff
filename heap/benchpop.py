import timeit
import random
import matplotlib.pyplot as plt

class ComparisonCounter:
    def __init__(self):
        self.count = 0

    def lt(self, a, b):
        self.count += 1
        return a < b

    def le(self, a, b):
        self.count += 1
        return a <= b

# Inlined siftdown version
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
    while pos > startpos:
        parentpos = (pos - 1) >> 1
        if counter.lt(newitem, heap[parentpos]):
            heap[pos] = heap[parentpos]
            pos = parentpos
        else:
            break
    heap[pos] = newitem

# Alternative implementation
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

# Heapify routines
def heapify_with_siftdown(lst):
    counter = ComparisonCounter()
    n = len(lst)
    for i in range(n // 2 - 1, -1, -1):
        _siftup_with_siftdown_inlined(lst, i, counter)
    return counter

def heapify_no_siftdown(lst):
    counter = ComparisonCounter()
    n = len(lst)
    for i in range(n // 2 - 1, -1, -1):
        _siftup_no_siftdown_optimized(lst, i, counter)
    return counter

# Heappop routines
def heappop_with_siftdown(heap, counter):
    lastelt = heap.pop()
    if heap:
        returnitem = heap[0]
        heap[0] = lastelt
        _siftup_with_siftdown_inlined(heap, 0, counter)
        return returnitem
    return lastelt

def heappop_no_siftdown(heap, counter):
    lastelt = heap.pop()
    if heap:
        returnitem = heap[0]
        heap[0] = lastelt
        _siftup_no_siftdown_optimized(heap, 0, counter)
        return returnitem
    return lastelt

# Multi-size benchmark
def benchmark_sizes(sizes):
    times_with = []
    comps_with = []
    times_no = []
    comps_no = []

    for size in sizes:
        data = [random.randint(0, 10000) for _ in range(size)]

        # With siftdown
        heap1 = data.copy()
        counter1 = heapify_with_siftdown(heap1)
        t1 = timeit.timeit(lambda: [heappop_with_siftdown(heap1, counter1) for _ in range(size)], number=1)

        # Without siftdown
        heap2 = data.copy()
        counter2 = heapify_no_siftdown(heap2)
        t2 = timeit.timeit(lambda: [heappop_no_siftdown(heap2, counter2) for _ in range(size)], number=1)

        times_with.append(t1)
        comps_with.append(counter1.count)
        times_no.append(t2)
        comps_no.append(counter2.count)

        print(f"Size: {size} - With: {t1:.4f}s, {counter1.count} comps | Without: {t2:.4f}s, {counter2.count} comps")

    return times_with, comps_with, times_no, comps_no

# Plotting
def plot_results(sizes, times_with, comps_with, times_no, comps_no):
    plt.figure(figsize=(12, 5))

    plt.subplot(1, 2, 1)
    plt.plot(sizes, times_with, 'o-', label='With siftdown')
    plt.plot(sizes, times_no, 's--', label='Without siftdown')
    plt.title('Heap Pop Time vs Heap Size')
    plt.xlabel('Heap Size')
    plt.ylabel('Time (s)')
    plt.legend()

    plt.subplot(1, 2, 2)
    plt.plot(sizes, comps_with, 'o-', label='With siftdown')
    plt.plot(sizes, comps_no, 's--', label='Without siftdown')
    plt.title('Heap Comparisons vs Heap Size')
    plt.xlabel('Heap Size')
    plt.ylabel('Comparisons')
    plt.legend()

    plt.tight_layout()
    plt.show()

# Main
if __name__ == "__main__":
    sizes = [1000, 2000, 5000, 10000, 20000]
    times_with, comps_with, times_no, comps_no = benchmark_sizes(sizes)
    plot_results(sizes, times_with, comps_with, times_no, comps_no)

