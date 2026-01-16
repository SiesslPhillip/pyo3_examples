import time
import random
from python_rust import mirror_string, selection_sort, insertion_sort, shell_sort, quick_sort_py

numbers = random.sample(range(1, 100010000), 20000000)
start = time.time()
quick_sort_py(numbers)
print(time.time() - start)