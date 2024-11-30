import { readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';

const currentDir = dirname(new URL(import.meta.url).pathname);

function main() {
    const input = readFileSync(join(currentDir, './test.txt')).toString();

    console.log("Part 1: ", part1(input));
    console.log("Part 2: ", part2(input));
}

function part1(input) {
    return;
}

function part2(input) {
    return;
}

main();