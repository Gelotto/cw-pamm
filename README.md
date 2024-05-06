# cw-pamm
A CosmWasm smart contract for parimutuel betting pools in which each pool is an
AMM and sells ownership shares directly from its bonding curve.

## Multi-AMM
PAMM stands for Parimutuel AMM. Each smart contract represents an
interoperable group of two or more traditional AMMs. Each AMM creates a its
own token and initializes it to a non-zero quote price. Users can then 
buy tokens directly from each market's bonding curve. 

Once a user has bought tokens using the configured quote token (most likely, the
blockchain's native coin), they may directly swap their tokens in one AMM for
tokens in another within same the group (within the smart contract).

## Parimutuel Betting 
When developer-defined conditions are met, trading closes, and the markets are
considered resolved. Upon resolution, one AMM is designated the winner. Any
account with a positive balance in the winning market may claim their share of
the smart contract's quote token balance, representing the net buy-in across all
markets and traders. The size of ther claim is proportional to the size of their
buy-in relative to all others.

## Fees
There are multiple places where fees come into play. Each is separately configurable.

- Buy fee - applied when quote token is swapped into an AMM.
- Sell fee - applied when quote token is swapped out the AMM.
- Swap fee - applied when tokens from one AMM are swapped with another.
- Claim fee - applied when quote token winnings are claimed