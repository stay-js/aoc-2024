import { file } from 'bun';

const demoInput = await file('demo-input.txt').text();
const input = await file('input.txt').text();

function getRulesAndChanges(input: string): [number[][], number[][]] {
  const parts = input.split('\n\n');

  const rules = parts[0].split('\n').map((line) => line.split('|').map((x) => parseInt(x)));
  const changes = parts[1].split('\n').map((line) => line.split(',').map((x) => parseInt(x)));

  return [rules, changes];
}

function checkValidity(rules: number[][], change: number[]): boolean {
  let valid = true;

  for (let i = 0; i < change.length; i++) {
    const rulesForField = rules.filter((rule) => rule[0] === change[i]);

    rulesForField.forEach((rule) => {
      const index = change.indexOf(rule[1]);
      if (index !== -1 && index < i) valid = false;
    });
  }

  return valid;
}

function firstPart(input: string): number {
  const [rules, changes] = getRulesAndChanges(input);

  const valid = changes.filter((change) => checkValidity(rules, change));

  return valid.reduce((acc, curr) => acc + curr[Math.floor(curr.length / 2)], 0);
}

function secondPart(input: string): number {
  const [rules, changes] = getRulesAndChanges(input);

  const inValid = changes.filter((change) => !checkValidity(rules, change));

  const fixed = inValid.map((change) => {
    while (!checkValidity(rules, change)) {
      for (let i = 0; i < change.length; i++) {
        const rulesForField = rules.filter((rule) => rule[0] === change[i]);

        rulesForField.forEach((rule) => {
          const index = change.indexOf(rule[1]);
          if (index !== -1 && index < i) [change[i], change[index]] = [change[index], change[i]];
        });
      }
    }

    return change;
  });

  return fixed.reduce((acc, curr) => acc + curr[Math.floor(curr.length / 2)], 0);
}

console.log('demo-input:');
console.log(firstPart(demoInput));
console.log(secondPart(demoInput));

console.log('\ninput:');
console.log(firstPart(input));
console.log(secondPart(input));
