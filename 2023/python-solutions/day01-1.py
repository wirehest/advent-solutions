#! python3

import re

from pathlib import Path

puzzle_input = open('day01input.txt', 'r')
raw_numbers_list = []
cleaned_numbers_list = []
alpha_strip = re.compile(r'[a-zA-Z\s]')
tally = 0

for line in puzzle_input:
    stripped = alpha_strip.sub('', line)
    raw_numbers_list.append(stripped)

for line in raw_numbers_list:
    cleaned_numbers_list.append(line[0] + line[-1])

for line in cleaned_numbers_list:
    tally += int(line)

print(tally)
puzzle_input.close()