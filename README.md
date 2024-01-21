Code to show a simple way to initialize a wallet with its mneomonic phrase that may have been generated from a browser wallet.

There are not succifient resources for initializing and using cosmos wallets.
There is basic documentation and there exist some examples from
https://gist.github.com/larry0x/92399bb2f9ce4b769e112ce5550091b9 or 
https://github.com/cosmos/cosmos-rust/issues/300 
but both do not use the same path that is used for normal keypair generation from a mneomonic phrase as expected from using Keplr/Leap wallets.


In this code we use BIP-0044's registered coin type for cosmos wallets as denoted in 
https://github.com/satoshilabs/slips/blob/master/slip-0044.md
More information on BIP32 derivation paths can be found here:
https://trezor.io/learn/a/what-is-bip32

