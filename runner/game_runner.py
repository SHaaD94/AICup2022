# repeat n times
#       start runner
#       connect clients
#       wait for runner finishes
#       read results
# show stats
# fail if won less than threshold

import sys
import subprocess
import json

current_version_path = '../target/aicup2020-jar-with-dependencies.jar'
versions_path = 'versions/'


class GameResult:
    def __init__(self, is_crashed, score, seed, place, was_last_survivor):
        self.is_crashed = is_crashed
        self.score = score
        self.seed = seed
        self.place = place
        self.was_last_survivor = was_last_survivor
        pass

    def __str__(self):
        return f'GameResult(winner {self.place == 1}, place {self.place}, last_survivor {self.was_last_survivor} score {self.score},  crashed {self.is_crashed}, seed {self.seed})'


def parse_game_result(res_file) -> GameResult:
    with open(res_file) as json_file:
        data = json.load(json_file)
        seed = data['seed']
        is_crashed = data['players'][0]['crashed']

        results = data['results']["players"]
        my_score = int(results[0]["score"])
        my_place = int(results[0]["place"])
        was_last = int(results[0]["units_alive"]) > 0

        return GameResult(is_crashed, my_score, seed, my_place, was_last)


def run_games(folder, repeats):
    games = []

    for i in range(repeats):
        print(f'Starting game number {i}')
        runner_process = subprocess.Popen(['./aicup22',
                                           '--batch-mode',
                                           '--config', f'{folder}/config.json',
                                           '--save-results', f'{folder}/res.json'])

        runner_process.wait()

        games.append(parse_game_result(f'{folder}/res.json'))

    return games


def main(args):
    folder = args[0]
    repeats = int(args[1]) if len(args) > 1 else 1
    win_threshold_in_percents = int(args[2]) if len(args) > 2 else 100

    games = run_games(folder, repeats)

    win_percent = len(list(filter(lambda x: bool(x.place < 3), games))) * 1.0 / repeats * 100.0
    crashed_games = len(list(filter(lambda x: bool(x.is_crashed), games)))

    for i, g in enumerate(games):
        print(f'Game {i + 1}: {g}')

    if crashed_games != 0:
        sys.exit(f"Strategy crashed in {crashed_games} out of {repeats}")

    if win_percent < win_threshold_in_percents:
        sys.exit(f"Strategy won in {win_percent} which is less than required {win_threshold_in_percents}")

    print('Run is successful :)')


if __name__ == '__main__':
    main(sys.argv[1:])
