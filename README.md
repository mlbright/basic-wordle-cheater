# basic-wordle-cheater

Use regular expressions to cheat at [Wordle][wordle].

## Usage

Defaults to consulting the `/usr/share/dict/words` dictionary on MacOS.

```bash
wordle-assistant "[^kd]r.ng"
areng
bring
orang
prong
wrang
wring
wrong
wrung
```

To specify another dictionary file,

```bash
wordle-assistant "s[^plt]ung" --dictionary "$HOME/Desktop/dictionary"
```

## Build

```bash
cargo build --release
cp target/release/wordle-assistant ~/.bin # ... or somewhere in your PATH
```

[wordle]: https://www.powerlanguage.co.uk/wordle/