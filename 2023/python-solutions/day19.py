#! python3

"""Advent of Code 2023: Day 19."""

import logging
import re

from pathlib import Path


logging.basicConfig(level=logging.DEBUG, format="%(levelname)s: %(message)s")
#logging.disable(logging.CRITICAL)


def load_puzzle_input(filepath):
    """Loads puzzle input file."""
    path = Path(filepath)
    puzzle_input = path.read_text()
    return puzzle_input


def make_puzzle_vars(puzzle_input):
    """Makes workable puzzle variable(s)."""
    flows_raw, parts_raw = puzzle_input.split('\n\n')
    flows_raw, parts_raw = flows_raw.splitlines(), parts_raw.splitlines()
    flows, parts = {}, {}

    for f in flows_raw:
        name, rules = f[:-1].split('{')
        flows[name] = rules.split(',')

    for i, r in enumerate(parts_raw):
        parts[i] = r[1:-1].split(',')
    
    logging.info(f"flows: {flows}")
    logging.info(f"parts: {parts}")
    return flows, parts


def walk_flows(flows, key_name):
    """ """
    pass


def walk_ratings(ratings):
    """"""
    for k, parts in ratings.items():
        for part in parts:
            logging.info(part)
            exec(part)


def main_loop(filepath):
    """Main loop."""
    puzzle_input = load_puzzle_input(filepath)
    flows, parts = make_puzzle_vars(puzzle_input)
    logging.info(f"\nflows:\n{flows}\nparts:\n{parts}")

    results = {}
    #x, m, a, s = 0, 0, 0, 0

    for k, ratings in parts.items():
        p_dict = {'x': 0, 'm': 0, 'a': 0, 's': 0}
        
        for rating in ratings:
            #logging.info(f"rating:{rating}")
            exec(rating, None, p_dict)
        
            
        logging.info(f"p_dict: {p_dict}")


main_loop('day19input.txt')