#! python3

"""Advent of Code 2023: Day 10."""

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
    puzzle_temp = puzzle_input.splitlines()
    puzzle_map = []

    for a in range(len(puzzle_temp)):
        puzzle_map.append([])
        for b in range(len(puzzle_temp[0])):
            puzzle_map[a].extend(puzzle_temp[b][a])

    return puzzle_map


def find_starts(puzzle_map):
    """Finds and returns origin and first two valid pipes."""
    origin = []
    starts = []
    pmap = puzzle_map
    directions_start = {
        'up': '7|F',
        'down': 'J|L',
        'left': 'L-F',
        'right': 'J-7',
        }    

    # Finds position of 'S.'
    for x in range(len(puzzle_map)):
        try:
            y = puzzle_map[x].index('S')
        except ValueError:
            continue
        origin = [x, y]

    # Finds direction of first two valid pipes.
    for d in directions_start:
        shifted = shift(origin, d)
        x, y = shifted[0], shifted[1]
        
        if pmap[x][y] in directions_start[d]:
            starts.append(d)
        else:
            continue
    
    starts.append(origin)
    return starts


def next_pipe(puzzle_map, coord, previous=None):
    """Finds the next pipe connections."""
    directions = {
        'up': ['7|F', 'J|L'],
        'down': ['J|L', '7|F'],
        'left': ['L-F', '7-J'],
        'right': ['J-7', 'L-F'],
        }
    pmap = puzzle_map
    next_pipe = ''

    for d in ('up', 'down', 'left', 'right'):
        shifted = shift(coord, d)
        curr_char = pmap[x1][y1]
        x1, y1 = coord[0], coord[1]
        x2, y2 = shifted[0], shifted[1]
        
        try:
            pmap[x2][y2]
        except IndexError:
            continue

        next_char = pmap[x2][y2]
        logging.info(f"check {d}: {coord}->{shifted} prev: {previous}")
        
        if (shifted != previous
                and curr_char in directions[d][1]
                and next_char in directions[d][0]):
            next_pipe = d

    return next_pipe    # Returns a direction, 'up', 'down', etc.


def shift(coord, direction):
    """Shifts given coordinate in given direction."""
    shifts = {'up': [0, -1], 'down': [0, 1], 'left': [-1, 0], 'right': [1, 0]}
    return [coord[0] + shifts[direction][0], coord[1] + shifts[direction][1]]


def count_steps(puzzle_map, starts):
    """Counts steps from coords in both directions."""
    active = True
    steps = 0
    pos1 = shift(starts[2], starts[0])   # Set first position.    
    pos2 = shift(starts[2], starts[1])   # Set second position.
    prev1 = starts[2]   # Initial previous position set to origin.
    prev2 = starts[2]   # Initial previous position set to origin.
    logging.info(f"initial pos: {pos1} {pos2} initial prev: {prev1} {prev2}")

    while active:
        active = False if pos1 == pos2 else True
        direction1 = next_pipe(puzzle_map, pos1, prev1)
        direction2 = next_pipe(puzzle_map, pos2, prev2)
        logging.info(f"directions: {direction1} {direction2}")

        prev1 = pos1
        prev2 = pos2
        logging.info(f"updating previous coords: {prev1} {prev2}")

        pos1 = shift(pos1, direction1)
        pos2 = shift(pos2, direction2)
        logging.info(f"updating position coords: {pos1} {pos2}")

        steps += 1
        
    return steps, pos2

def main_loop(filepath):
    """Main loop to call sub-functions."""
    
    # Initializes puzzle map.
    puzzle_input = load_puzzle_input(filepath)
    puzzle_map = make_puzzle_vars(puzzle_input)
    
    # Gets start positions (initial pipes connected to the origin.)
    starts = find_starts(puzzle_map)
    logging.info(f"starts: {starts}\n")

    # Starts step-counting function.
    steps, final_pos = count_steps(puzzle_map, starts)
    print(f"steps: {steps} final position: {final_pos}")


main_loop('day10input.txt')