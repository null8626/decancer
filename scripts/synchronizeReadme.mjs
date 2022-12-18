import { execSync } from 'node:child_process'
import { copyFileSync } from 'node:fs'

const file = execSync('git diff --name-only HEAD HEAD~1').toString().trim().split(/\r?\n/).find(x => x.endsWith('README.md'))
const copiedTo = file === 'README.md' ? './node/README.md' : './README.md'

copyFileSync(`./${file}`, copiedTo)