#! python3

import logging

from pathlib import Path

logging.basicConfig(level=logging.DEBUG, format="%(levelname)s: %(message)s")
logging.disable(logging.CRITICAL)

def load_puzzle_input(filepath):
    """Loads puzzle input file."""
    path = Path(filepath)
    puzzle_input = path.read_text()
    logging.info(f"puzzle_input: {puzzle_input}")
    return puzzle_input

def make_puzzle_vars(puzzle_input):
    """Converts puzzle_input into list."""
    puzzle_input_list = puzzle_input.splitlines()
    for i, line in enumerate(puzzle_input_list):
        #logging.info(f"line: {line}")
        puzzle_input_list[i] = [int(x) for x in line.split()]
    logging.info(f"puzzle_input_list: {puzzle_input_list}")
    return puzzle_input_list

def extrapolate(sequences):
    """Builds up sequences until extrapolated value then returns it."""
    seq_len = len(sequences)
    for i in range(seq_len - 1, 0, -1):
        i_len = len(sequences[i])
        sequences[i - 1] += [sequences[i - 1][-1] + sequences[i][i_len - 1]] 

    logging.info(f"sequences: {sequences}")
    next_val = sequences[0][-1]
    return next_val

def build_down(line):
    """Builds out lines until deltas are equal."""
    sequences = [line]
    while True:
        if ave_check(sequences[-1]):
            break
        else:
            new_seq = []
            last_len = len(sequences[-1])
            for i in range(1, last_len):
                new_seq.append(sequences[-1][i] - sequences[-1][i - 1])
            
            sequences.append(new_seq)
        
    logging.info(f"sequences: {sequences}")
    return sequences

def ave_check(line):
    """Checks whether deltas are equal."""
    if line[0] == sum(line) / len(line):
        return True
    else:
        return False

def part1(filepath):
    """Main loop for part one."""
    puzzle_input = load_puzzle_input(filepath)
    puzzle_input_list = make_puzzle_vars(puzzle_input)
    tally = 0

    for line in puzzle_input_list:
        logging.info(f"line: {line}")
        sequences = build_down(line)
        next_val = extrapolate(sequences)
        tally += next_val
        print(f"next_val: {next_val}")
    print(f"tally: {tally}")

part1('day09input.txt')