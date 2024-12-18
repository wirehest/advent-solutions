#! python3

"""Advent of Code 2023: Day 11."""

import logging

from pathlib import Path
from itertools import combinations

logging.basicConfig(level=logging.DEBUG, format="%(levelname)s: %(message)s")
logging.disable(logging.CRITICAL)


def load_puzzle_input(filepath):
    """Loads puzzle input file."""
    path = Path(filepath)
    puzzle_input = path.read_text()
    return puzzle_input


def make_puzzle_vars(puzzle_input):
    """Makes workable puzzle variable(s)."""
    puzzle_map = puzzle_input.splitlines()
    return puzzle_map


def find_empties(puzzle_map):
    """Finds empty rows and columns."""
    empty = []
    rows = range(len(puzzle_map))
    cols = range(len(puzzle_map[0]))
    empty_rows, empty_cols = set(rows), set(cols)

    for x, row in enumerate(puzzle_map):
        for y, char in enumerate(row):
            if char == '#':
                empty_rows -= {x}
                empty_cols -= {y}
            else:
                continue

    return [(sorted(empty_rows, reverse=True)), 
        (sorted(empty_cols, reverse=True))]


def measure_paths(puzzle_map, empties, part_two=False):
    """Creates galaxy combinations and finds all distances."""
    counter = 1
    galaxies = {}
    paths = []
    factor = 1 if part_two == False else 1_000_000 - 1

    # Builds galaxies dict with each galaxy's coordinates.
    for x, row in enumerate(puzzle_map):
        for y, char in enumerate(row):
            if char == '#':
                galaxies[counter] = (x, y) 
                counter += 1
    
    # Combinations of galaxies.
    combos = combinations(galaxies.keys(), 2)
    logging.info(f"galaxies: {galaxies}")

    for pair in combos:
        a, b = pair[0], pair[1]
        x_adjust, y_adjust = 0, 0   # Path length adjustment vars.

        logging.info(f"pair: {pair}")
        logging.info(f"a: {galaxies[a]} b: {galaxies[b]}")
        logging.info(f"empties: {empties}")
        
        # Counts number of expanding rows between galaxies a and b.
        for empty_row in empties[0]:
            if (galaxies[a][0] < empty_row < galaxies[b][0]
                    or galaxies[b][0] < empty_row < galaxies[a][0]):
                x_adjust += 1

        # Counts number of expanding columns between galaxies a and b.
        for empty_col in empties[1]:
            if (galaxies[a][1] < empty_col < galaxies[b][1]
                    or galaxies[b][1] < empty_col < galaxies[a][1]):
                y_adjust += 1

        path = (
            abs(galaxies[a][0] - galaxies[b][0])
            + abs(galaxies[a][1] - galaxies[b][1])
            + factor * (x_adjust + y_adjust)
            )
        paths.append(path)

    logging.info(paths)
    return(sum(paths))


def main_loop(filepath):
    """Main loop to call sub-functions."""
    puzzle_input = load_puzzle_input(filepath)
    puzzle_map = make_puzzle_vars(puzzle_input)
    empties = find_empties(puzzle_map)

    tally1 = measure_paths(puzzle_map, empties, part_two=False)
    tally2 = measure_paths(puzzle_map, empties, part_two=True)
    print(f"Part 1: {tally1}, Part 2: {tally2}")


main_loop('day11input.txt')