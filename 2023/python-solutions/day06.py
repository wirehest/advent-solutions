#! python3

import logging
import math

logging.basicConfig(level=logging.DEBUG, format="%(levelname)s: %(message)s")
#logging.disable(logging.CRITICAL)

puzzle_input1 = (
    (46, 347),
    (82, 1522),
    (84, 1406),
    (79, 1471),
    )

puzzle_input2 = (
    (46_828_479, 347_152_214_061_471),
    )

def try_hold_time(i, time, record):
    """Calculates distance traveled for a given hold time."""
    hold = i
    speed = i
    time_remaining = time - hold
    distance = speed * time_remaining
    logging.info(f"result: {distance} traveled vs. {record} record")

    if distance > record:
        return True
    else:
        return False 
    
def try_input(puzzle_input):
    """Tries different hold times for a given game time-record pair."""
    winning_holds = []

    for game in puzzle_input:
        counter = 0
        time = game[0]
        record = game[1]
        logging.info(f"game: {game} type: {type(game)}")

        #for time, record in game:
        for i in range(0, time):
            logging.info(f"holding for {i} seconds")
            if try_hold_time(i, time, record):
                counter += 1
                logging.info(f"incrementing counter; {counter}")
                #input()
            else:
                continue

        logging.info(f"appending {counter} to winning_holds")
        #input()
        winning_holds.append(counter)

    return winning_holds
    
def parabolic_solver(puzzle_input):
    """For part 2. Solves puzzle via quadratics."""
    winning_holds = []

    for game in puzzle_input:
        time = game[0]
        record = game[1]

        quad1 = time / 2
        quad2 = math.sqrt((time ** 2) - (4 * record)) / 2
        logging.info(f"solutions: {quad1 + quad2}-{quad1 - quad2}")
        count = math.ceil(quad1 + quad2) - math.ceil(quad1 - quad2)

        winning_holds.append(count)
    
    return winning_holds

winning_holds = try_input(puzzle_input1)
logging.info(f"winning_holds: {winning_holds}")

tally = 1

for count in winning_holds:
    tally *= count

print(f"tally part 1: {tally}")
input()

winning_holds = parabolic_solver(puzzle_input1)
logging.info(f"winning_holds (parabolic): {winning_holds}")

tally = 1

for count in winning_holds:
    tally *= count

print(f"tally part 1: {tally}")
input()

winning_holds = parabolic_solver(puzzle_input2)
logging.info(f"winning_holds (parabolic): {winning_holds}")

tally = 1

for count in winning_holds:
    tally *= count

print(f"tally part 2 (parabolic): {tally}")


