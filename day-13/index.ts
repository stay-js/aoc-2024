import { file } from 'bun';

const demoInput = await file('demo-input.txt').text();
const input = await file('input.txt').text();

type Position = { x: number; y: number };

type ClawMachine = {
  a: Position;
  b: Position;
  prize: Position;
};

function parsePosition(line: string, delimiter: string, offset: number = 0) {
  const [, coords] = line.split(':');
  const [x, y] = coords.split(',').map((part) => Number(part.split(delimiter)[1]) + offset);

  return { x, y };
}

function solveClawMachine({ a, b, prize }: ClawMachine) {
  // Cramer's rule + ChatGPT's help
  const px = prize.x;
  const py = prize.y;

  const d = a.x * b.y - a.y * b.x;
  const di = px * b.y - py * b.x;
  const dj = py * a.x - px * a.y;

  if (d === 0 || di % d !== 0 || dj % d !== 0) return null;

  const i = di / d;
  const j = dj / d;

  return 3 * i + j;
}

function calculateTokensSpent(input: string, offset: number) {
  return input
    .split('\n\n')
    .map((block) => {
      const [a, b, prize] = block.split('\n');

      return {
        a: parsePosition(a, '+'),
        b: parsePosition(b, '+'),
        prize: parsePosition(prize, '=', offset),
      } satisfies ClawMachine;
    })
    .reduce((acc, curr) => acc + (solveClawMachine(curr) ?? 0), 0);
}

const firstPart = (input: string) => calculateTokensSpent(input, 0);
const secondPart = (input: string) => calculateTokensSpent(input, 10_000_000_000_000);

console.log('demo-input:');
console.log(firstPart(demoInput));

console.log('\ninput:');
console.log(firstPart(input));
console.log(secondPart(input));
