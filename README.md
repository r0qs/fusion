# Trollup

Trollup is an experimental Ethereum L2, created with the goal of being the
simplest L2 that can be used in production.

## Tool Suite

This repository contains the entire Trollup tool suite:

- `trollup-sequencer`: the Trollup node. Receives L2 transactions via RPC, builds
  blocks, and sends them for verification on L1.
- `l1-verifier`: the Trollup contracts deployed on L1. These contracts provide
  the enter/exit L2 functionalities and block verification for L2 nodes.
- `trollup-wallet`: a simple CLI interface to sign/send Trollup transactions.

## Run it

Requirements: [foundry](https://github.com/foundry-rs/foundry).

1. Set `ETH_PRIVATE_KEY` and `ETH_FROM` to the private and public keys that will deploy and use the L1 contract.
2. Set `ETH_RPC_URL` to the endpoint. Since we are using `anvil` here, this is usually `http://localhost:8545`.
3. Run `source ./scripts/run_anvil_and_deploy_contract` which starts `anvil` and deploys the contract.
4. Run `./scripts/run_node` to start the node.
5. You can run `./scripts/listen_to_node` to check the ongoing output from the node.
6. Now you can also run `./scripts/send_random_tx` to send transactions.
7. To stop everything, run `./scripts/kill_node` and `./scripts/kill_anvil`.

## What

Currently, Trollup is basically a dumb payment channel. Users can enter and
exit the L2 via the L1 contract. Users can also send and receive TrollETH via
L2 transactions.

## State

The state currently consists of two addresses,
0x318A2475f1ba1A1AC4562D1541512d3649eE1131 (A1) and
0x419978a8729ed2c3b1048b5Bba49f8599eD8F7C1 (A2). The state root is
`keccak256(A1, A2)`.

## Execution

There is no VM on the L2 at the moment.

## Roadmap

These are roughly the steps we want to achieve.

### Milestone 1 (done)

The goal is to have a working dumb L2 that only knows two addresses.  This
includes a node, ECDSA, state verification on an L1 contract.

### Milestone 2

The end goal is to have the L1 contract only store the state roots and
receive a SNARK for verification.
