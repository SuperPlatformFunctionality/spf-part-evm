
**SPF EVM Compatible Part built with [Substrate](https://substrate.io).**


### Prefunded Development Addresses

Running SPF in development mode will pre-fund several well-known addresses that (mostly) contain the letters "th" in their names to remind you that they are for ethereum-compatible usage. These addresses are derived from
Substrate's canonical mnemonic: `bottom drive obey lake curtain smoke basket hold race lonely fit walk`

```
# Alith:
- Address: 0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac
- PrivKey: 0x5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133

# Baltathar:
- Address: 0x3Cd0A705a2DC65e5b1E1205896BaA2be8A07c6e0
- PrivKey: 0x8075991ce870b93a8870eca0c0f91913d12f47948ca0fd25b49c6fa7cdbeee8b

# Charleth:
- Address: 0x798d4Ba9baf0064Ec19eB4F0a1a45785ae9D6DFc
- PrivKey: 0x0b6e18cafb6ed99687ec547bd28139cafdd2bffe70e6b688025de6b445aa5c5b

# Dorothy:
- Address: 0x773539d4Ac0e786233D90A233654ccEE26a613D9
- PrivKey: 0x39539ab1876910bbf3a223d84a29e28f1cb4e2e456503e7e91ed39b2e7223d68

# Ethan:
- Address: 0xFf64d3F6efE2317EE2807d223a0Bdc4c0c49dfDB
- PrivKey: 0x7dce9bc8babb68fec1409be38c8e1a52650206a7ed90ff956ae8a6d15eeaaef4

# Faith:
- Address: 0xC0F0f4ab324C46e55D02D0033343B4Be8A55532d
- PrivKey: 0xb9d2ea9a615f3165812e8d44de0d24da9bbd164b65c4f0573e1ce2c8dbd9c8df

# Goliath:
- Address: 0x7BF369283338E12C90514468aa3868A551AB2929
- PrivKey: 0x96b8a38e12e1a31dee1eab2fffdf9d9990045f5b37e44d8cc27766ef294acf18

# Heath:
- Address: 0x931f3600a299fd9B24cEfB3BfF79388D19804BeA
- PrivKey: 0x0d6dcaaef49272a5411896be8ad16c01c35d6f8c18873387b71fbc734759b0ab

# Ida:
- Address: 0xC41C5F1123ECCd5ce233578B2e7ebd5693869d73
- PrivKey: 0x4c42532034540267bf568198ccec4cb822a025da542861fcb146a5fab6433ff8

# Judith:
- Address: 0x2898FE7a42Be376C8BC7AF536A940F7Fd5aDd423
- PrivKey: 0x94c49300a58d576011096bcb006aa06f5a91b34b4383891e8029c21dc39fbb8b
```

Also, the prefunded default account for testing purposes is:

```
# Gerald:
- Address: 0x6Be02d1d3665660d22FF9624b7BE0551ee1Ac91b
- PrivKey: 0x99b3c12287537e38c90a9219d4cb074a89a16e9cdb20bf85728ebd97c343e342
```

## Build the SPF Node

To build SPF, you will need a proper Substrate development environment. 

If you need a refresher setting up your Substrate environment, see [Substrate's Getting Started Guide](https://substrate.io/docs/en/knowledgebase/getting-started/).

```bash
cd spf-part-evm

# Build the node (The first build will be long (~30min))
cargo build --release
```

## Chain IDs

The Ethereum specification described a numeric Chain Id. The SPF mainnet Chain Id will be 1280


## Runtime Architecture

The SPF Runtime is built using FRAME and consists of pallets from substrate, frontier, cumulus, and `pallets/`.

From substrate:

- _Utility_: Allows users to use derivative accounts, and batch calls
- _Balances_: Tracks GLMR token balances
- _Sudo_: Allows a privileged account to make arbitrary runtime changes - will be removed before
  launch
- _Timestamp_: On-Chain notion of time
- _Transaction Payment_: Transaction payment (fee) management
- _Randomness Collective Flip_: A (mock) onchain randomness beacon. Will be replaced by parachain
  randomness by mainnet.

From frontier:

- _EVM_: Encapsulates execution logic for an Ethereum Virtual Machine
- _Ethereum_: Ethereum-style data encoding and access for the EVM.

From cumulus:

- _ParachainUpgrade_: A helper to perform runtime upgrades on parachains
- _ParachainInfo_: A place to store parachain-relevant constants like parachain id

The following pallets are stored in `pallets/`. They are designed for SPF's specific requirements:

- _Ethereum Chain Id_: A place to store the chain id for each SPF network
- _Author Inherent_: Allows block authors to include their identity in a block via an inherent
- _Parachain Staking_: Minimal staking pallet that selects collators by total amount at stake

When modifying the git repository for these dependencies, a tool called [diener](https://github.com/bkchr/diener) can be used to replace the git URL and branch for each reference in all `Cargo.toml` files with a single command. This alleviates a lot of the repetitive modifications necessary when changing dependency versions.

