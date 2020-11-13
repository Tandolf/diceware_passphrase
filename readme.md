# Passphrase generator using Diceware methodology

![github action workflow](https://github.com/tandolf/passphrase_diceware/workflows/Rust/badge.svg)
[![codecov](https://codecov.io/gh/Tandolf/passphrase_diceware/branch/master/graph/badge.svg)](https://codecov.io/gh/Tandolf/passphrase_diceware)

This is a rust library generating passphrases using the diceware wordlists.

[Diceware passphrases](https://theworld.com/~reinhold/diceware.html)

TODO:
- Create generator struct
- configure generator with `config` struct
- ~~capitalize first letter in each word~~
- ~~select wordlists using enums~~
- generate special chars
- enable/disable capitalization of first letter in each word
- enable/disable special chars
- set phrase length
- refactor dice rolling to use `iter#fold()`

