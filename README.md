# aave-cmd

This is a simple command-line tool designed to retrieve onchain information from [AAVE](https://aave.com/)'s smart contracts.
It can be integrated into scripts, xbar, and more.

**Work in progress.**

## Authors

- [@cizeon](https://github.com/Cizeon)

## Installation

Install aave with cargo

```bash
  cargo install --path .
```

## Usage/Examples

```bash
Usage: aave [OPTIONS] <COMMAND>

Commands:
  info
  token
  portfolio
  stake
  help       Print this message or the help of the given subcommand(s)

Options:
      --ether
      --polygon
      --gnosis
      --rmm
      --rpc-url <RPC_URL>                              [env: ETH_RPC_URL=]
      --pool-address-provider <POOL_ADDRESS_PROVIDER>  [env: POOL_ADDRESS_PROVIDER=]
      --json
      --no-color
      --verbose
  -h, --help                                           Print help
  -V, --version                                        Print version
```
