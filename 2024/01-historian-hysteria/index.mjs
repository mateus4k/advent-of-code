import { readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';

const currentDir = dirname(new URL(import.meta.url).pathname);

function main() {
    const input = readFileSync(join(currentDir, './input.txt')).toString();
    const [left, right] = parse(input);

    console.log('Part 1: ', part1(left, right));
    console.log('Part 2: ', part2(left, right));
}

function parse(input) {
    const leftColumn = [];
    const rightColumn = [];

    for (const line of input.split('\n')) {
        const [left, right] = line.split(/\s+/);
        leftColumn.push(Number(left));
        rightColumn.push(Number(right));
    }

    return [leftColumn, rightColumn];
}

function part1(left, right) {
    const leftColumn = left.sort();
    const rightColumn = right.sort();
    let distance = 0;

    for (let i = 0; i < leftColumn.length; i++) {
        const leftElement = leftColumn[i];
        const rightElement = rightColumn[i];
        distance += rightElement > leftElement ? rightElement - leftElement : leftElement - rightElement;
    }

    return distance;
}

function part2(left, right) {
    const occurrencesMap = new Map();

    for (const element of right) {
        if (occurrencesMap.has(element)) {
            const currentValue = occurrencesMap.get(element);
            occurrencesMap.set(element, currentValue + 1);
        } else {
            occurrencesMap.set(element, 1);
        }
    }

    let similarityScore = 0;

    for (const element of left) {
        const occurrences = occurrencesMap.get(element) || 0;
        similarityScore += element * occurrences;
    }

    return similarityScore;
}

main();