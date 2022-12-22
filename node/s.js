const { readFileSync, writeFileSync } = require('node:fs')
const pene = JSON.parse(readFileSync('../output.json'))

for (const [codepoint, translation] of readFileSync('add.txt').toString().trim().split(/\r?\n/).map(x => {
  const [a, b] = x.split(' ');
  return [parseInt(a, 16), b]
})) {
  pene.confusables.push({
	codepoint,
	translation
  })
}

writeFileSync('../output.json', JSON.stringify(pene, null, 2))