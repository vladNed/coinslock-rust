# RegTest Bitcoin

This is a helper file to note down all the commands that I use to run a regtest bitcoin network.


## Loading a wallet

After ssh-ing into the container usually a wallet is not loaded. To load a wallet, run the following command:

```bash
# To create
bitcoin-cli createwallet adminos

# To load
bitcoin-cli loadwallet adminos
```

Check if the wallet is loaded by running the following command:

```bash
bitcoin-cli getwalletinfo
```

