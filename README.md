# Tetromino solver

Rules:

- Fit all 10 pieces on the 8x5 board

## Pieces

```
#  #  #  ####
#  ## ##
## #  #  ####

#  ## ## #  #
#  ## ## ## ##
##        #  #
```

## Possible solutions

783 (but this code finds 1337 which can't be a coincidence)

## Usage

Requires [rust](https://www.rust-lang.org/tools/install).

```
cargo run . [--debug]

-d/--debug: turns on logging and a small delay for visual debugging
```
