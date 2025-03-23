<!-- markdownlint-disable no-inline-html first-line-heading -->

<div align="center">

![evar.webp](./images/evar.webp)

</div>

# evar

Welcome to evar, a feature-rich CLI calculator REPL!
My goal was to create a modern, ergonomic, and simple calculator.

## Features ✨

- **Basic Math Functions**: Perform essential mathematical operations.

  Supported operators: `+`, `-`, `*`, `/`, `%`, `^`, `!`

  Supported functions:
  - **Trigonometric Functions**: `sin`, `cos`, `tan`, `sec`, `csc`, `cot`
  - **Inverse Trigonometric Functions**: `asin`, `acos`, `atan`
  - **Hyperbolic Functions**: `sinh`, `cosh`, `tanh`
  - **Exponential and Logarithmic Functions**: `sqrt`, `exp`, `exp2`, `ln`, `log2`, `log10`, `log`
  - **Angle Conversion Functions**: `rad`, `deg`
  - **Rounding Functions**: `floor`, `ceil`, `round`
  - **Miscellaneous Functions**: `abs`, `log`, `ntroot`

- **Mathematical Notation**: Write expressions like `2 sin(x)` instead of `2 * sin(x)`.

  ```bash
  > 2sin(2pi/3)
  1.7320508075688774
  > 2 * sin(2 * pi / 3)
  1.7320508075688774
  > sqrt(3)
  1.7320508075688772
  ```

- **Variable & Function Definition**: Define your own variables and functions.

  ```bash
  > let x = 3
  3
  > let y = 10
  10
  > let f(z, w) = z + w
  > f(x, y)
  13
  ```

- **Neat Error Handling**: Easily understand errors.

  ```bash
  > let f(x) = 
  error: found end of input expected expression
    ┌─ <repl>:1:12
    │
  1 │ let f(x) = 
    │ ---------- ^ found end of input expected expression
    │ │          
    │ while parsing this function definition
  ```

- **Persistent History**: Keep a record of your calculations.

- **Realtime Input Highlight**: See your input highlighted as you type.

- **Radian & Degrees Mode**: Switch between radian and degrees mode.

- **And More...**

## Inspiration 💡

evar is inspired by [`eva`](https://github.com/oppiliappan/eva), another CLI calculator written in Rust.

## Installation 🛠️

- crates.io

```bash
cargo install evar
```

- manual

```bash
git clone https://github.com/enklht/evar.git
cd evar
cargo run
```

## Usage 🚀

```
Modern ergonomic math calculator inspired by eva

Usage: evar [OPTIONS]

Options:
  -a, --angle-unit <ANGLE_UNIT>  Angle Unit [default: radian] [possible values: radian,
 degree]
      --no-color                 Enable colored output
  -f, --fix <FIX>                Number of decimal places in output (0-63) [default: No
ne]
  -d, --debug                    Print parsed expression for debug purpose
  -h, --help                     Print help
  -V, --version                  Print version
```

## Contributing 🤝

Contributions are welcome via GitHub.

## License 📜

This project is licensed under the MIT License.
