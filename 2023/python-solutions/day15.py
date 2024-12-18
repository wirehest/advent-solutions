#! python3

"""Advent of Code 2023: Day 15."""

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
    init_list = puzzle_input.split(',')
    return init_list


def hash_algo(char, value):
    """Converts character according to Day 15's hash algorithm."""
    value = (17 * (value + ord(char))) % 256
    return value

def main_loop(filepath):
    """Main loop."""
    puzzle_input = load_puzzle_input(filepath)
    init_list = make_puzzle_vars(puzzle_input)
    hashed_list = []

    for step in init_list:
        value = 0

        for char in step:
            value = hash_algo(char, value)

        hashed_list.append(value)

    logging.info(f"init_list: {init_list}")
    logging.info(f"hashed_list: {hashed_list}")

    tally = sum(hashed_list)
    print(f"part 1, tally: {tally}")


main_loop('day15input.txt')