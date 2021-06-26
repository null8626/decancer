# decancer
Node.js module that decancers a string. Cleans everything back to lowercased, clean, alphanumeric state. Covers a large range of unicode groups, including:

- Zalgos
- Fancy characters & unicode "fonts"
- Emojified characters
- Foreign punctuations
- Cyrillic characters
- Greek characters
- Japanese characters
- Arabic characters
- And so much more.

# installation
Install with npm:
```bash
$ npm install decancer
```

# examples
```js
const decancer = require('decancer');

const noCancer = decancer('vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣');
console.log(noCancer); // 'very funny text'

const deEmojized = decancer('🇻🇪🇷🇾 🇫🇺🇳🇳🇾 🇹🇪🇽🇹');
console.log(deEmojized); // 'very funny text'

const banishedZalgos = decancer('v̸̨̢̥̜̦͇̹̰̳̯̳͍̭̐̊͂̎͋̋͒͘͜͝ͅe̵̢̛͔̜̦̙̖̭̜͍̜̰̦̺͐̐̎̌̇̓̚͝͠r̸̨̥̩̍̈́̿͛̑̀̿͆͋̀͠͝ÿ̴̧̢̧̱̙͚͕̬̻̞͈͔̘̪́ͅ ̴̡̬͎͚̼̪͇̗͗̽̀̽̋̾͂̃͗̈̾͊̕f̷̢͚͍̝͇̠͈͗̎̏́͑̄̀͗̀̿͛ȕ̷̡̡͉̯͙͙̔̐̀̾̈̓̉̐̓̋̓͘͜͜n̴̢̢̟̫̺͇̞͈̯̮̣̰̅̋̊̕̕͜n̴̢̺̗̖̼̙̙̝̼̳̋̽̏̎ỳ̶̲̺͈̖̣̘̙͕̰͜ͅ ̶̡͈̟̺͎̯̯̲͉͙̫̾̆͛̅̑͐̇̊̌͊̑̚͘ͅẗ̷̪̬̰̝͉̼̙͙̗̥̭̥͕̪͗̐̇́̂͘e̶̙̿̌̈́̇͗͐͌̃̈́͊͒͝x̵̣͈͚͆̇̅͊̈̕t̸̳̫͇̭̻͙̤͖̠̔͂̒̀̄̽̋̆̅̇͒̏̕͠͝');
console.log(banishedZalgos); // 'very funny text'
```
> **NOTE:** all output will be in lowercase.

# contributions
All contributions are welcome. If you want to, you can [make a fork here at GitHub.](https://github.com/vierofernando/decancer) Thanks! &lt;3