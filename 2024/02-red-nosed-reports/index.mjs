import { readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';

const currentDir = dirname(new URL(import.meta.url).pathname);

function main() {
    const input = readFileSync(join(currentDir, './input.txt')).toString();

    console.log('Part 1: ', part1(input));
    console.log('Part 2: ', part2(input));
}

function isSafe(levels) {
    const diffs = levels.slice(1).map((level, index) => level - levels[index]);
    return diffs.every((diff) => diff >= 1 && diff <= 3) || diffs.every((diff) => diff <= -1 && diff >= -3);
}

function part1(input) {
    let count = 0;

    for (const report of input.split('\n')) {
        const levels = report.split(' ').map(Number);
        if (isSafe(levels)) {
            count++;
        }
    }

    return count;
}

function part2(input) {
    let count = 0;

    for (const report of input.split('\n')) {
        const levels = report.split(' ').map(Number);

        for (const [index] of levels.entries()) {
            const newLevels = [...levels.slice(0, index), ...levels.slice(index + 1)];

            if (isSafe(newLevels)) {
                count++;
                break;
            }
        }
    }

    return count;
}

main();