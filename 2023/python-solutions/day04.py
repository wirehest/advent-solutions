#! python3

import logging
import copy

from pathlib import Path

logging.basicConfig(level=logging.DEBUG, format="%(levelname)s: %(message)s")
#logging.disable(logging.CRITICAL)

filepath = 'day04input.txt'

def load_puzzle_input(filepath):
    """Loads puzzle input file."""
    path = Path(filepath)
    puzzle_input = path.read_text()

    return puzzle_input

def make_puzzle_dict(puzzle_input):
    """Makes dict with puzzle data."""
    puzzle_dict = {}
    puzzle_split = puzzle_input.splitlines()
    logging.info(puzzle_split)

    for i, line in enumerate(puzzle_split):
        game_num = i + 1
        winning_nums, my_nums = line.split('|')
        _, winning_nums = winning_nums.split(':')
        winning_nums = winning_nums.strip().split()
        my_nums = my_nums.strip().split()
        logging.info(f"{game_num}: {winning_nums} | {my_nums}")

        puzzle_dict[i + 1] = [winning_nums, my_nums, 0]
        logging.info(f"puzzle_dict: {puzzle_dict}")
    
    return puzzle_dict

def count_winners(puzzle_dict):
    """Counts winning numbers and adds count to puzzle_dict."""
    for game, numbers in puzzle_dict.items():
        my_nums = numbers[0]
        winning_nums = numbers[1]
        winners = 0

        for my_num in my_nums:
            winners += 1 if my_num in winning_nums else 0
        
        puzzle_dict[game][2] = winners

def scorer(puzzle_dict):
    """Calculates points from winning games."""
    tally = 0

    for numbers in puzzle_dict.values():
        count_winners = int(numbers[2])
        score = 1 * (2 ** (count_winners - 1)) if count_winners > 0 else 0
        tally += score

    return tally

puzzle_input = load_puzzle_input('day04input.txt')
puzzle_dict = make_puzzle_dict(puzzle_input)
count_winners(puzzle_dict)
total = scorer(puzzle_dict)
print(f"Part 1: {total}")
