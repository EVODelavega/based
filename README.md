# based

Small utility to convert values between bases 2, 8, 10, and 16. A task that is trivial to perform in any programming language, but still useful to have a utility for when debugging.

## Usage

```
based [OPTIONS] -v <VALUE>

Options:
  -v <VALUE>         
  -m, --mode <MODE>  [default: decimal] [possible values: binary, octal, decimal, hex]
  -t, --to <TO>      [possible values: binary, octal, decimal, hex]
  -h, --help         Print help
```

To print the value of a decimal numer in binary, octal, decimal, and hex:

```sh
$ based -v 123

BIN: 1111011
OCT: 173
DEC: 123
HEX: 0x00007B
```

To print a decimal value using a specific base/radix (for example hex):

```sh
$ based -v 123 -t hex

Formatted: 0x00007B
```

To convert a value other than decimal (for example decimal):

```sh
$ based -v 101010 -m binary

BIN: 101010
OCT: 52
DEC: 42
HEX: 0x00002A
```

Or, of course, specifically convert a non-decimal input to base 10:

```sh
$ based -v 101010 -m binary --to decimal

Formatted: 42
```
