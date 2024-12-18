#! python3

"""Advent of Code 2023: Day 3."""

import logging
import math
import re

from pathlib import Path

logging.basicConfig(level=logging.DEBUG, format="%(levelname)s: %(message)s")
logging.disable(logging.CRITICAL)


def make_puzzle_input(filepath):
    """Makes workable puzzle input."""
    p_input = Path(filepath).read_text().splitlines()
    logging.info(f"p_input:\n\n{Path(filepath).read_text()}\n")

    return p_input, len(p_input[0]), len(p_input)    


def make_sym_dict(p_input, width, height, part_two):
    """Makes dict. with symbol positions only."""
    symbols = '#-%=*$+/&@' if part_two == False else '*'

    return {(r, c): [] for r in range(height) for c in range(width) 
        if p_input[r][c] in symbols
        }


def find_parts(p_input, sym):
    """Finds parts by: 
    1) making dict. of numbers and edges; 
    2) finding the intersection of the number dict. with the sym. dict.;
    3) where there's an intersection, all coordinates from the numbers
    dict. are appended to the symbols dict.
    """
    for r, row in enumerate(p_input):
            # Matches numbers in puzzle input.
            for num in re.finditer(r'\d+', row):
                logging.info(f"sym: {sym}")
                logging.info(f"matches: {num}")
                
                # Dict. of numbers and edges.
                edge = {(r, c) for r in (r - 1, r, r + 1) 
                    for c in range(num.start() - 1, num.end() + 1)
                    }
                
                logging.info(f"edge:\n{edge}\n")
                logging.info(f"type(edge): {type(edge)} "
                    f" type(sym.keys()): {type(sym.keys())}")

                # "&" returns intersection (not bitwise &) between
                # edge+numbers and symbols.
                for o in edge & sym.keys():
                    logging.info(f"num.group(): {num.group()}")
                    
                    # group() method appends matched numbers to symbol dict.
                    sym[o].append(int(num.group()))


def main_loop(filepath, part_two):
    """Calls sub-functions."""
    p_input, width, height = make_puzzle_input(filepath)
    sym = make_sym_dict(p_input, width, height, part_two)
    find_parts(p_input, sym)
    
    game_part = '1' if part_two == False else '2'
    tally = 0

    if part_two == False:
        tally = sum(sum(n) for n in sym.values())
    else:
        tally = sum(math.prod(n) for n in sym.values() if len(n) == 2)

    print(f"Part {game_part}: {tally}")


main_loop('day03input.txt', part_two=False)  # Part 1
main_loop('day03input.txt', part_two=True)   # Part 2