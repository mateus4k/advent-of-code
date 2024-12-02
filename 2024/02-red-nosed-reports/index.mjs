import { readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';

const currentDir = dirname(new URL(import.meta.url).pathname);

function main() {
    const input = readFileSync(join(currentDir, './test.txt')).toString();

    console.log('Part 1: ', part1(input));
    console.log('Part 2: ', part2(input));
}

function part1(input) {
    let safeReportsCount = 0;

    for (const report of input.split('\n')) {
        let lastNumber;
        let isIncreasing = false;
        let isDecreasing = false;
        let isSafe = true;

        for (let level of report.split(' ')) {
            level = Number(level);

            if (typeof lastNumber === 'undefined') {
                lastNumber = level;
                continue;
            }

            const diffFromLastNumber = Math.abs(lastNumber - level);
            if (lastNumber === level || diffFromLastNumber < 1 || diffFromLastNumber > 3) {
                isSafe = false;
                continue;
            }

            if (lastNumber > level) {
                isDecreasing = true;
            }

            if (lastNumber < level) {
                isIncreasing = true;
            }

            lastNumber = level;
        }

        // Levels cannot increase and decrease at the same time.
        if (isDecreasing && isIncreasing) {
            isSafe = false;
        }

        if (isSafe) {
            safeReportsCount++;
        }
    }

    return safeReportsCount;
}

function part2(input) {
    return;
}

main();