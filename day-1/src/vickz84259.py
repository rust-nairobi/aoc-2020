import time
import itertools
from typing import Set


def get_entries() -> Set[int]:
    with open('input_1.txt', 'r') as f:
        entries_str = f.readlines()

    return set(int(entry.strip()) for entry in entries_str)


def part_1(entries: Set[int]) -> None:
    print('Part 1:')

    for entry in entries:
        entry_2 = 2020 - entry
        if entry_2 in entries:
            print(f'Values: {entry} and {entry_2}')

            answer = entry * entry_2
            print(f'Answer: {answer}')

            break


def part_1_alt(entries: Set[int]) -> None:
    print('Part 1 Alternative:')

    subtraction = ((entry, 2020 - entry) for entry in entries)
    entry, entry_2 = next(filter(lambda x: x[1] in entries, subtraction))

    print(f'Values: {entry} and {entry_2}')
    print(f'Answer: {entry * entry_2}')


def find_entry_3(new_entry: int, entries: Set[int]) -> int:
    answer = -1

    for entry in entries:
        entry_3 = 2020 - new_entry
        if entry_3 in entries:
            answer = entry_3
            break

    return answer


def part_2(entries: Set[int]) -> None:
    print('Part 2:')

    entries_copy = entries.copy()

    for entry in entries:
        for entry_2 in entries_copy:
            new_entry = entry + entry_2
            if new_entry >= 2020:
                continue

            entry_3 = find_entry_3(new_entry, entries_copy)
            if entry_3 != -1:
                print(f'Values: {entry}, {entry_2}, {entry_3}')

                answer = entry * entry_2 * entry_3
                print(f'Answer: {answer}')

                return

        entries_copy.remove(entry)


def part_2_alt(entries: Set[int]) -> None:
    print('Part 2 Alternative:')

    combinations = itertools.combinations(entries, 2)
    addition = map(lambda x: (x[0], x[1], x[0] + x[1]), combinations)
    filtered = filter(lambda x: x[2] < 2020, addition)

    subtraction = ((e_1, e_2, 2020 - value) for e_1, e_2, value in filtered)
    entry_1, entry_2, entry_3 = next(
        filter(lambda x: x[2] in entries, subtraction))

    print(f'Values: {entry_1}, {entry_2}, {entry_3}')

    answer = entry_1 * entry_2 * entry_3
    print(f'Answer: {answer}')


def main() -> None:
    entries = get_entries()

    start_time = time.process_time()
    part_1(entries)
    print(f'Time taken: {time.process_time() - start_time}')
    print('---------------')

    start_time = time.process_time()
    part_1_alt(entries)
    print(f'Time taken: {time.process_time() - start_time}')
    print('---------------')

    start_time = time.process_time()
    part_2(entries)
    print(f'Time taken: {time.process_time() - start_time}')
    print('---------------')

    start_time = time.process_time()
    part_2_alt(entries)
    print(f'Time taken: {time.process_time() - start_time}')


if __name__ == '__main__':
    main()
