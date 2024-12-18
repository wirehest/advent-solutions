#! python3

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
    """Converts puzzle_input into variables."""
    logging.info(f"puzzle_input: {puzzle_input}")
    puzzle_input_list = puzzle_input.splitlines()
    nav = puzzle_input_list[0]
    nodes = {}
    nodes_str = puzzle_input_list[2:]
    
    for node in nodes_str:
        a, b = node.split(' = ')
        b = b.replace('(', '("').replace(')', '")').replace(', ', '", "')
        nodes[a] = eval(b)

    return nav, nodes

def part1(filepath):
    """Main loop for part one."""
    curr_node = 'AAA'
    puzzle_input = load_puzzle_input(filepath)
    nav, nodes = make_puzzle_vars(puzzle_input)
    steps = 0
    active = True

    logging.info(f"nav: {nav}")
    logging.info(f"nodes: {nodes}")

    while active:
        for n in nav:
            logging.info(f"steps: {steps} curr_node: {curr_node} turn: {n}")
            turn = 0 if n == 'L' else 1
            
            if curr_node != 'ZZZ':
                steps += 1
                curr_node = nodes[curr_node][turn]
            else:
                active = False
                break

    print(f"steps: {steps}")

part1('day08input.txt')