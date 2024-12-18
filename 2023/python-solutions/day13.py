#! python3

"""Advent of Code 2023: Day 13."""

import logging

from pathlib import Path


logging.basicConfig(level=logging.DEBUG, format="%(levelname)s: %(message)s")
#logging.disable(logging.CRITICAL)


def load_puzzle_input(filepath):
    """Loads puzzle input file."""
    path = Path(filepath)
    puzzle_input = path.read_text()
    return puzzle_input


def make_puzzle_vars(puzzle_input):
    """Makes workable variable(s) from puzzle_input."""
    patterns = [p.split('\n') for p in puzzle_input.split('\n\n')]
    return patterns

def find_reflection(pattern):
    """ """
    # Get pattern dimensions.
    w = len(pattern[0])
    h = len(pattern)
    logging.info(f"dimensions: {w}x{h}")
    mirror = None

    # Check rows.
    for i in range(0, w - 1):
        if pattern[i] == pattern[i + 1]:
            mirror = ['R', i]
            break
        else:
            continue
    
    flipped = []
    for x in range(w):
        string = ''
        for y in range(h):
            string += pattern[y][x]
        flipped.append(string)

    # Check columns.
    for i in range(0, h - 1):
        if pattern[i] == pattern[i + 1]:
            mirror = ['C', i]
            break
        else:
            continue

    if mirror == None:
        raise Exception("mirror: None")
    logging.info(f"mirror: {mirror}")
    return mirror


def summarize(mirror):
    """Adds # of columns, and 100 * # of rows, per pattern."""
    summ = [mirror[1] if mirror[0] == 'C' else 100 * mirror[1]]
    return summ

def main_loop(filepath):
    """Main loop."""
    puzzle_input = load_puzzle_input(filepath)
    patterns = make_puzzle_vars(puzzle_input)
    mirrors = [find_reflection(p) for p in patterns]
    logging.info(f"mirrors: {mirrors}")

    tally = []
    for m in mirrors:
        tally += summarize(m)

    tally = sum(tally)
    print(f"tally: {tally}")

main_loop('day13input.txt')