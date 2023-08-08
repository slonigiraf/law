# Recommendation letter module

## Overview

It is a [Substrate](https://github.com/paritytech/substrate) pallet to manage onchain reputation by issuing recommendation letters.
A person who issues a letter (referee) stake some tokens from his account on a recommendation letter about worker. The worker can show this recommendation letter to an employer and enable the employer to slash guarantee tokens in a case of wrong recommendation letter.
This pallet was not audited for bugs. Do not use this pallet as-is in production.

## Interface

### Dispatchable Functions

* `reimburse` - Send a transaction to penalize a referee.

## How to test
```sh
git clone https://github.com/slonigiraf/law.git
cd law
cargo test --features balances
```

## How to use
- An example blockchain node that uses recommendation letters: [law-example-node](https://github.com/slonigiraf/law-example-node)
- An example user interface: [law-example-ui](https://github.com/slonigiraf/law-example-ui)

License: Unlicense
