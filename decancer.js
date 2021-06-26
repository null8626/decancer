const constants = require('./constants');
const numbers = [...'0123456789'];
const alphabet = [...'abcdefghijklmnopqrstuvwxyz'];

/**
 * @param {string} text The text to decancer.
 * @returns {string} The cleaned string. Will ALWAYS be in lowercase.
 */
module.exports = (text) => {
    if (typeof text !== 'string' || !text.length)
        throw new TypeError("'text' must be a string and it must contain at least a character.");

    else if (!/[^\u0000-\u007F]/.test(text))
        return text.toLowerCase(); // sorry
    
    for (const [k, v] of Object.entries(constants.emojis))
        text = text.replace(new RegExp(v, 'g'), k);

    for (const [k, v] of Object.entries(constants.miscOthers))
        text = text.replace(new RegExp(`[${v}]`), k);
    
    text = text
      .toLowerCase()
      .replace(constants.startRegex, '');

    for (let i = 0; i < 10; i++)
        text = text.replace(new RegExp(`[${constants.numericalStyles.map(x => String.fromCodePoint(x + i)).join('')}]`, 'gi'), numbers[i]);
  
    for (let i = 0; i < 26; i++)
        text = text.replace(new RegExp(`[${constants.alphabeticalStyles.map(x => String.fromCodePoint(x + i)).join('')}${constants.miscAlphabetical[i]}]`, 'gi'), alphabet[i]);

    return text;
};