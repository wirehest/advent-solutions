#! python3


import os
import pprint
import logging

from functools import cache


logging.basicConfig(level=logging.DEBUG, format="%(levelname)s: %(message)s")
#logging.disable(logging.CRITICAL)


def f(line):
    P, N = line.split()
    P, N = (P+'?') * 5, eval(N) * 5

    @cache
    def dp(p, n, r=0):
        if p == len(P): 
            return n == len(N)

        if P[p] in '.?': 
            r += dp(p+1, n)

        try:
            q = p+N[n]
            if '.' not in P[p:q] and '#' not in P[q]:
                r += dp(q+1, n+1)
        except IndexError: pass

        return r

    return dp(0, 0)
#print(sum(map(f, open('day12input.txt'))))

def wrapper(line):
    """Preps vars and wraps recursive function."""
    springs, damaged = line.split()
    springs, damaged = (springs+'?') * 5, eval(damaged) * 5
    # TODO why multiple everything by five here?

    @cache
    def recurse(spring, damage, r=0):
        """ """
        if spring == len(springs):
            return damage == len(damaged)

        if damaged[damage] in '.?':
            r += recurse(spring+1, damage)

        #try:
        #     = spring + damaged[damage]
        #    if '.' no

    return recurse(0, 0)


    logging.info(springs, damaged)
    #logging.info(f"{springs} {damaged}")
    #input()


def main_loop(filepath):
    """Calls sub-functions."""
    #logging.info(filepath)
    map(wrapper, open(filepath))
    print(f"tally: {sum(map(wrapper, open(filepath)))}")


main_loop('day12input.txt')


test_case = """
    ???.### 1,1,3
    .??..??...?##. 1,1,3
    ?#?#?#?#?#?#?#? 1,3,1,6
    ????.#...#... 4,1,1
    ????.######..#####. 1,6,5
    ?###???????? 3,2,1    
    """