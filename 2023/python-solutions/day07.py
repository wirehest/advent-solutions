#! python3

import logging

from pathlib import Path

logging.basicConfig(level=logging.DEBUG, format="%(levelname)s: %(message)s")
logging.disable(logging.CRITICAL)

def load_puzzle_input(filepath):
    """Loads puzzle input file."""
    path = Path(filepath)
    puzzle_input = path.read_text()

    return puzzle_input

def make_puzzle_vars(puzzle_input):
    """Converts puzzle_input into variables."""
    hands_dict = {}

    for line in puzzle_input.splitlines():
        logging.info(f"line: {line}")
        hand, points = line.split(' ')
        points = int(points)
        hands_dict[hand] = points

    return hands_dict

def order_hand(hand):
    """Use hand composition to apply score for ordering."""
    unique_cards = tuple(set(hand))
    card_counts = []
    hands = {
        '5': 10000000000,
        '14': 20000000000,
        '23': 30000000000,
        '113': 40000000000,
        '122': 50000000000,
        '1112': 60000000000,
        '11111': 70000000000,
        }

    for unique_card in unique_cards:
        count = 0

        for i in hand:
            count += 1 if i == unique_card else 0
        
        card_counts.append(str(count))

    card_counts = ''.join(sorted(card_counts))
    hand_score = hands[card_counts]
    hand_score = order_hand2(hand, hand_score)

    logging.info(f"{hand}: {hand_score}")    
    return hand_score

def order_hand2(hand, hand_score):
    """Second ordering rule by card strength."""
    cards = ('A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2')
    card_ranks = {}
    card_score = 0
    for a, b in enumerate(cards):
        card_ranks[b] = a + 1

    for i, c in enumerate(hand):
        card_score += card_ranks[c] * (100 ** (len(hand) - i - 1))

    hand_score += card_score
    return hand_score

def scorer(hands, hands_dict):
    """Calculates final score."""
    tally = 0
    for i, hand in enumerate(hands):
        tally += hands_dict[hand] * (i + 1)
    
    return tally

def part1(filepath):
    """Part1 main loop. Calls other functions."""
    puzzle_input = load_puzzle_input(filepath)
    hands_dict = make_puzzle_vars(puzzle_input)
    logging.info(f"hands: {hands_dict.keys()}\nscores: {hands_dict.values()}")

    hands = list(hands_dict.keys())

    logging.info(f"hands, pre-sort: {hands}")
    hands.sort(key=order_hand, reverse=True)
    logging.info(f"hands, sorted: {hands}")

    tally = scorer(hands, hands_dict)
    print(f"tally: {tally}")

def part2(filepath):
    """ """
    pass

part1('day07input.txt')