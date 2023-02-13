# japanese-words-generator

Generator of fake Japanese words built with hiragana, katakana, and rōmaji.
As words are not real, it is only useful in kana reading and writing training.
Long vowels were skipped because these can be indeterminate when transforming to kana.
Extended katakana is divided into two parts, first one consists of the most common characters.
Some rōmaji have a `*` appended to them, to differentiate between duplicate transcriptions.

### Running

To run the app you will need to install rust on your machine, and then you can simply execute the `cargo run` command at the project's root.
Alternatively, you may want to build with `cargo build`, and use the executable.

### Config

Config is loaded from the `config.yaml` located in the current directory. If no config is present the default one will be generated.

Default config:

```
---
kana_types:
  - gojūon
  - dakuten
  - handakuten
  - yōon
  - extended1
  - extended2
lines_count: 10
words_per_line: 5
word_chances:
  - chance: 50
    length: 4
  - chance: 50
    length: 5
no_spaces_in_kana: false
```

- `kana_types` - types of kana to be included in the generation, example above lists all available types, these are not validated, so generated text will consist only of the correct types
- `lines_count` - how many lines of text will be generated
- `words_per_line` - how many words will be placed in each line
- `word_chances` - a word with given `length` will be generated with `chance`, where chance is a value in percent, chances have to sum up to 100
- `no_spaces_in_kana` - if true, then there will be no whitespace in between the words in each line, but only in kana output, rōmaji will have spaces regardless

### Output

`kana.txt` and `romaji.txt` will be saved in the current directory.
