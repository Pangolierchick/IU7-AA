from random import randint
import itertools
from copy import deepcopy

def is_equal(l1:list, l2:list) -> bool:
    if len(l1) != len(l2):
        return False

    for a, b in zip(l1, l2):
        if a != b:
            return False
    
    return True

def get_rand_list(size:int) -> list:
    lo = 0
    hi = 10000
    res = []

    for i in range(size):
        res.append(randint(lo, hi))

    return res

def bubble(l:list) -> None:
    for i in range(len(l)):
        for j in range(i, len(l)):
            if l[i] > l[j]:
                l[i], l[j] = l[j], l[i]

def main():
    l1 = get_rand_list(10)
    l2 = deepcopy(l1)

    bubble(l1)
    l2.sort()

    print(*l1)
    print(*l2)

    print(is_equal(l1, l2))

main()
