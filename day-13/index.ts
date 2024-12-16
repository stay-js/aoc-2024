import { file } from 'bun';

const demoInput = await file('demo-input.txt').text();
const input = await file('input.txt').text();

type Position = { x: number; y: number };

type ClawMachine = {
  a: Position;
  b: Position;
  prize: Position;
};

function parsePosition(line: string, delimiter: string) {
  const [, coords] = line.split(':');
  const [x, y] = coords.split(',').map((part) => Number(part.split(delimiter)[1]));

  return { x, y };
}

function solveClawMachine({ a, b, prize }: ClawMachine, maxPresses: number) {
  let minTokens: number | null = null;

  for (let i = 0; i <= maxPresses; i++) {
    for (let j = 0; j <= maxPresses; j++) {
      const x = i * a.x + j * b.x;
      const y = i * a.y + j * b.y;

      if (x === prize.x && y === prize.y) {
        const tokens = 3 * i + j;

        if (minTokens === null || tokens < minTokens) minTokens = tokens;
      }
    }
  }

  return minTokens;
}

function firstPart(input: string) {
  const MAX_PRESSES = 100;

  return input
    .split('\n\n')
    .map((block) => {
      const [a, b, prize] = block.split('\n');

      return {
        a: parsePosition(a, '+'),
        b: parsePosition(b, '+'),
        prize: parsePosition(prize, '='),
      } satisfies ClawMachine;
    })
    .reduce((acc, curr) => acc + (solveClawMachine(curr, MAX_PRESSES) ?? 0), 0);
}

console.log('demo-input:');
console.log(firstPart(demoInput));
// console.log(secondPart(demoInput));

console.log('\ninput:');
console.log(firstPart(input));
// console.log(secondPart(input));
