# Levensthein Distance In Rust

This is a small educational project for teaching Rust.
It contains a command line tool that filters strings by their Levensthein distance to a search word.

This project is licensed under the terms of the [MIT license](LICENSE).

## Build

```bash
cargo build
```

## Usage

```bash
./target/debug/rust-levensthein house <<EOT 
tree
flower
mouse
EOT
```

[dwyl/english-words](https://github.com/dwyl/english-words) provides a suitable list of English words:

```bash
curl https://raw.githubusercontent.com/dwyl/english-words/master/words.txt --output words.txt
```


## See also

- [Levenshtein Distance](https://en.wikipedia.org/wiki/Levenshtein_distance)

## Status

[![Build Status](https://travis-ci.org/az82/rust-levenshtein.svg?branch=master)](https://travis-ci.org/az82/rust-levenshtein)
