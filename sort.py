import time
import random
from python_rust import mirror_string, selection_sort, insertion_sort, shell_sort, quick_sort_py

numbers = random.sample(range(1, 100010000), 20000000)
#start = time.time()
#selection_sort(numbers)
#print("selection_sort:", time.time() - start)
#start = time.time()
#insertion_sort(numbers)
#print("insertion_sort: ",time.time() - start)
shell_sorts = []
for x in range(0, 1):
    start = time.time()
    shell_sort(numbers)
    shell_sorts.append(time.time() - start)
print("shell_sort: ", sum(shell_sorts)/len(shell_sorts))
quick_sorts = []
for x in range(0, 1):
    start = time.time()
    quick_sort_py(numbers)
    quick_sorts.append(time.time() - start)

print("quick_sort: ", sum(quick_sorts)/len(quick_sorts))
numbers.sort()
shell_sorts = []
for x in range(0, 1):
    start = time.time()
    shell_sort(numbers)
    shell_sorts.append(time.time() - start)
print("sorted shell_sort: ", sum(shell_sorts)/len(shell_sorts))
quick_sorts = []
for x in range(0, 1):
    start = time.time()
    quick_sort_py(numbers)
    quick_sorts.append(time.time() - start)

print("sorted quick_sort: ", sum(quick_sorts)/len(quick_sorts))