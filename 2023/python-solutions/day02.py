#! python3

import logging

logging.basicConfig(level=logging.DEBUG, format='%(levelname)s: %(message)s')
#logging.disable(logging.CRITICAL)

puzzle_input = open('day02input.txt', 'r')
dice_load = {'red': 12, 'green': 13, 'blue': 14}
game_validity = {}
game_dict = {}
#tally = 0
#tally_power = 0

test_game = ('Game 1: 4 red, 5 blue, 4 green; 7 red, 8 blue, 2 green;'
    ' 9 blue, 6 red; 1 green, 3 red, 7 blue; 3 green, 7 red')

def build_dict(game_line):
    """Builds-out game_dict."""
    game_data = game_line.split(':')
    num = int(game_data[0].split(' ')[1])
    logging.info(f"num: {num}")
    game_dict[num] = {}

    game_sets = game_data[1].split(';') 
    logging.info(f"game_sets: {game_sets}")

    for i, game_set in enumerate(game_sets):
        game_sets[i] = game_set.strip().replace('\n', '')
    
    game_dict[num]['sets'] = game_sets

def verify_game(game_set):
    """Verifies validity of dice set against dice_load."""
    game_list = []
    validity = True

    for game in game_set:
        game_list = game.split(',')

        for dice in game_list:
            dice_count = int(dice.strip().split(' ')[0])
            dice_color = dice.strip().split(' ')[1]

            if dice_load[dice_color] >= dice_count:
                continue
            
            else:
                logging.info(f"game invalid: {dice_color} {dice_count}"
                    f" > {dice_load[dice_color]}")
                validity = False
                break
    
    return validity

def find_power(game_set):
    """Finds product of minimum required red, green, blue dice per game."""
    min_dice = {'red': 0, 'green': 0, 'blue': 0}
    power = 1

    for game in game_set:
        game_list = game.split(',')

        for dice in game_list:
            dice_count = int(dice.strip().split(' ')[0])
            dice_color = dice.strip().split(' ')[1]

            if dice_count > min_dice[dice_color]:
                min_dice[dice_color] = dice_count
            else:
                continue
                
        logging.info(f"min_dice: {min_dice}")

    for value in min_dice.values():
        power *= value

    return power

def part_1():
    tally = 0
    for game_line in puzzle_input:
        build_dict(game_line)
        
    for game in game_dict:
        game_set = game_dict[game]['sets']
        game_dict[game]['validity'] = verify_game(game_set)
        logging.info(f"game_num: {game}"
            f"\ngame_sets: {game_dict[game]['sets']}"
            f"\nvalidity: {game_dict[game]['validity']}")

    logging.info(f"game_dict: {game_dict[1]}")

    for game in game_dict:
        if game_dict[game]['validity']:
            tally += game
            logging.info(f"game #{game} "
                f"validity: {game_dict[game]['validity']}; tally: {tally}")
        else:
            continue

    return tally

def part_2():
    tally_power = 0
    for game_line in puzzle_input:
        build_dict(game_line)
        
    for game in game_dict:
        game_set = game_dict[game]['sets']
        game_dict[game]['power'] = find_power(game_set)

    for game in game_dict:
        tally_power += game_dict[game]['power']

    return tally_power

print(f"Part 1: {part_1()}")
print(f"Part 2: {part_2()}")

puzzle_input.close()