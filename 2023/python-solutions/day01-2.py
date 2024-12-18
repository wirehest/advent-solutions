#! python3

import re
import logging

logging.basicConfig(level=logging.DEBUG, format='%(levelname)s: %(message)s')
#logging.disable(logging.CRITICAL)

text_to_numerals = {
    'one': 1, 
    'two': 2, 
    'three': 3,
    'four': 4, 
    'five': 5,
    'six': 6,
    'seven': 7,
    'eight': 8,
    'nine': 9,
    }

puzzle_input = open('day01input.txt', 'r')
raw_numbers_list = []
cleaned_numbers_list = []
numerals = re.compile(r'[0-9]')
alpha = re.compile(r'[a-zA-Z\s]')
tally = 0

for line in puzzle_input:
    stringholder = ''
    logging.info(f"line: {line}")
    
    for n, char in enumerate(line):
        logging.info(f"checking {char} at index {n}")
        
        try:
            int(char)
            stringholder += char 

        except ValueError:
            for key, value in text_to_numerals.items():
                key_len = len(key)
                logging.info(f"line range: [{n}:{n + key_len}]")
                if line[n:(n + key_len)] == key:
                    stringholder += str(value)
        
        logging.info(stringholder)
    raw_numbers_list.append(stringholder)

for line in raw_numbers_list:
    cleaned_numbers_list.append(line[0] + line[-1])

for line in cleaned_numbers_list:
    tally += int(line)

print(tally)
puzzle_input.close()