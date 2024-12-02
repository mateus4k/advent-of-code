import { readFileSync } from 'fs';

const getFile = () => readFileSync('./02-red-nosed-reports/test.txt', 'utf-8');

const input = getFile();
const lines = input.split('\n');
let count = 0;

for (const line of lines) {
  const items = line.split(' ').map(Number);

  function isValidSequence(numbers) {
    const isGenerallyIncreasing =
      numbers.reduce((accum, curr, index) => {
        items[index] < items[index + 1] && ++accum;
        return accum;
      }, 0) >
        numbers.length / 2
        ? true
        : false;

    for (let i = 0; i < numbers.length - 1; i++) {
      const diff = isGenerallyIncreasing
        ? numbers[i + 1] - numbers[i]
        : numbers[i] - numbers[i + 1];

      if (diff < 1 || diff > 3) return i + 1;
    }
    return -1;
  }

  let isValid = isValidSequence(items);
  if (isValid >= 0) {
    const newList = [...items.slice(0, isValid), ...items.slice(isValid + 1)];
    const newList2 = [...items.slice(0, isValid - 1), ...items.slice(isValid)];

    const isValid1 = isValidSequence(newList);
    const isValid2 = isValidSequence(newList2);

    isValid = isValid1 === -1 ? isValid1 : isValid2;
  }

  if (isValid == -1) ++count;
}

console.log(count);