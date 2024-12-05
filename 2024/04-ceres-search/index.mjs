import { readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';

const currentDir = dirname(new URL(import.meta.url).pathname);

function main() {
    const input = readFileSync(join(currentDir, './input.txt')).toString().split('\n');

    console.log('Part 1: ', part1(input));
    console.log('Part 2: ', part2(input));
}

function part1(grid) {
    const delta = [-1, 0, 1];
    let count = 0;

    for (let row = 0; row < grid.length; row++) {
        for (let col = 0; col < grid[0].length; col++) {
            if (grid[row][col] !== 'X') continue;

            for (const rowDelta of delta) {
                for (const colDelta of delta) {
                    if (rowDelta === 0 && colDelta === 0) continue;

                    const nextRow = row + rowDelta;
                    const nextCol = col + colDelta;
                    const secondNextRow = row + 2 * rowDelta;
                    const secondNextCol = col + 2 * colDelta;
                    const thirdNextRow = row + 3 * rowDelta;
                    const thirdNextCol = col + 3 * colDelta;

                    if (
                        thirdNextRow >= 0 && thirdNextRow < grid.length &&
                        thirdNextCol >= 0 && thirdNextCol < grid[0].length &&
                        grid[nextRow][nextCol] === 'M' &&
                        grid[secondNextRow][secondNextCol] === 'A' &&
                        grid[thirdNextRow][thirdNextCol] === 'S'
                    )
                        count++;
                }
            }
        }
    }

    return count;
}

function part2(grid) {
    const patterns = ['MMSS', 'MSSM', 'SSMM', 'SMMS'];
    let count = 0;

    for (let row = 1; row < grid.length - 1; row++) {
        for (let col = 1; col < grid[0].length - 1; col++) {
            if (grid[row][col] !== 'A') continue;

            const corners = [
                grid[row - 1][col - 1],
                grid[row - 1][col + 1],
                grid[row + 1][col + 1],
                grid[row + 1][col - 1],
            ];

            if (patterns.includes(corners.join('')))
                count++;
        }
    }

    return count;
}

main();