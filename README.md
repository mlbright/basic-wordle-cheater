# basic-wordle-cheater

A really corny way to cheat at Wordle.

## Usage

```bash
# Guesses at this point are:
# SNAKE -> A and E are yellow
# ABOVE -> A and E are yellow (repeated myself uselessly)
# BAGEL -> A and E are yellow (here we know A is the fourth letter, but we make the computer do more work)
wordle-assistant --green ....l --yellow "[a][a][a][e][e]" --grey bgovsnk
ceral
decal
demal
ectal
equal
ethal
fecal
feral
fetal
hemal
ideal
medal
metal
pedal
petal
pheal
queal
tepal
ureal
wheal

# Chose 'IDEAL' and discover that AL are green, DE are yellow, and I is grey

./target/release/wordle-assistant --green ...al --yellow "[a][da][ae][e][e]" --grey ibgovsnk
decal
demal
medal
pedal

# Chose 'MEDAL': solved!
```

## Build

```bash
cargo build --release
cp target/release/wordle-assistant ~/.bin # ... or somewhere in your PATH
```

[wordle]: https://www.powerlanguage.co.uk/wordle/