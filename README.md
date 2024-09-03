# A [binary options](https://www.investopedia.com/terms/b/binary-option.asp) implementation for BTC price

### Short description:
Users bet on how will be the price of BTC in the future (higher or lower compared to the price at the time of bet).

### Tech stack:
- Anchor for solana program
- Pyth as oracle to determine the price of BTC at a specific timestamp
- React for client UI
- Nodejs for processing the games finality (win or lose)

### How to run
1. clone the repository
2. run `git submodule update --init --recursive` to fetch the submodules
