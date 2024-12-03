import { readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';

const currentDir = dirname(new URL(import.meta.url).pathname);

function main() {
    const input = readFileSync(join(currentDir, './input.txt')).toString();

    console.log('Part 1: ', part1(input));
    console.log('Part 2: ', part2(input));
}

/**
 * @param {string} input 
 * @returns {number} 
 */
function part1(input) {
    const regex = /mul\(([0-9]{1,3}),([0-9]{1,3})\)/g;
    let sum = 0;

    for (const match of input.match(regex)) {
        const [x, y] = match.slice(4, -1).split(',');
        sum += Number(x) * Number(y);
    }

    return sum;
}

/**
 * @param {string} input 
 * @returns {number} 
 */
function part2(input) {
    const regex = /do\(\)|don\'t\(\)|mul\(([0-9]{1,3}),([0-9]{1,3})\)/g;
    let sum = 0;
    let activate = true;

    for (const match of input.match(regex)) {
        if (match === 'do()')
            activate = true;
        else if (match === 'don\'t()')
            activate = false;
        else if (activate) {
            const [x, y] = match.slice(4, -1).split(',');
            sum += Number(x) * Number(y);
        }
    }

    return sum;
}

main();