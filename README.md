# Lift

A calculator for strength training.  Calculates warmup sets and barbell racking.

## Example

Say you want to Squat 145lbs with 4 warmup sets and a 45lb bar.

```
$ lift --bar 45 --sets 4 145
 45x5x2 []
 80x4x1 [10.0, 5.0, 2.5]
115x3x1 [35.0]
145x5x3 [45.0, 5.0]
```

## Usage

```
$ lift --help
A calculator for strength training.

USAGE:
    lift [OPTIONS] <work-set>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b, --bar <bar>      The bar weight. [default: 45]
    -s, --sets <sets>    The number of sets. [default: 5]

ARGS:
    <work-set>    Sets the weight of the work set.  Must be great than or equal to the bar weight.
```
