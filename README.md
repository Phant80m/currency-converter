⚠️ Project is discontinued, check out:
https://github.com/phant80m/Tahwil, updated version ⚠️
# cconvert Documentation

## CConvert:

## introduction

cconvert is a command-line tool for currency conversion. It allows you to convert currency values from one currency to another using real-time exchange rates.

## Installation

To install cconvert, follow these steps:

1. Go to the [GitHub Actions page](https://github.com/phant80m/currency-converter/actions).
2. Locate the latest release of cconvert.
3. Click on the release to view the details.
4. Download the binary file for your operating system.

## Manual Compiling

If you prefer to compile cconvert manually, follow these steps:

1. Clone the cconvert repository from GitHub:
 ```zsh
 git clone https://github.com/phant80m/currency-converter
 ```
2. Navigate to the cloned directory:
```zsh
cd currency-converter
```

3. Compile the source code using cargo.
```zsh
cargo build --release
```
4. Once the compilation is complete, you will find the executable file in the build directory.
```zsh
./target/release/cconvert
```
## Syntax

The syntax for using cconvert is as follows:
- `-i <currency>`: Specifies the input currency symbol or code.
- `-o <currency>`: Specifies the output currency symbol or code.
- `-i <amount>`: Specifies the amount of currency to convert.

## Examples

Here are some examples of how to use cconvert:

- Convert 100 USD to EUR:
`Sure! Here's the raw markdown for the cconvert documentation enclosed in triple backticks:

## Syntax

The syntax for using cconvert is as follows:

cconvert -i <currency, String> -o <currency, String> -i <amount, i32>

markdown


- `-i <currency>`: Specifies the input currency symbol or code.
- `-o <currency>`: Specifies the output currency symbol or code.
- `-i <amount>`: Specifies the amount of currency to convert.

## Examples

Here are some examples of how to use cconvert:

- Convert 100 USD to EUR:
`cconvert -i USD -o EUR -i 100`
- Convert 50 GBP to USD:
`cconvert -i GBP -o USD -i 50`
- Convert 2000 JPY to AUD:
`cconvert -i JPY -o AUD -i 2000`

### That's it! Now you can use cconvert to convert currencies easily from the command line.
---


Nick Samuels - Dev 




