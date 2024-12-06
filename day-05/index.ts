import { file } from 'bun';

const demoInput = await file('demo-input.txt').text();
const input = await file('input.txt').text();

function firstPart(input: string): number {
  const parts = input.split('\n\n');

  const rules = parts[0].split('\n').map((line) => line.split('|').map((x) => parseInt(x)));
  const changes = parts[1].split('\n').map((line) => line.split(',').map((x) => parseInt(x)));

  const valid = changes.filter((change) => {
    let valid = true;

    for (let i = 0; i < change.length; i++) {
      const rulesForField = rules.filter((rule) => rule[0] == change[i]);

      rulesForField.forEach((rule) => {
        const index = change.indexOf(rule[1]);
        if (index != -1 && index < i) valid = false;
      });
    }

    return valid;
  });

  return valid.reduce((acc, curr) => acc + curr[Math.floor(curr.length / 2)], 0);
}

console.log('demo-input.txt');
console.log(firstPart(demoInput));

console.log('\ninput.txt');
console.log(firstPart(input));
