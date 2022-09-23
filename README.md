# rust_blockchain

## Purpose
The goal of this project is to build a simple blockchain in rust to solidify principles and allow for basic understanding of how blockchains work under the hood.

### Current State
This project is implmented from the command line with a Menu based approach to interact with the chain. The menu allows for creating keys, requesting funds, sending funds, and adding blocks. 
The menu allows for spinning up a node module, but this node is not yet functional. See Roadmap for project goals

## Run 
Ensure that all proper Rust components are installed. 
On the command line in the src directory, cargo run

```
cargo run
```
This will enable a menu based approach to interacting with a simple chain.

### Project Layout
The layout is based on two structures.
1. src/models which contain all the blockchain components from blocks to key management
2. src/server this is the implementation of the menu and future server components 

#### Dependencies
This project has basic rust dependencies, one of the most significant is encryption mechanisms for keys. This project relies on the secp256k1 crate that enables ECDSA encryption. See the toml file for other dependencies. 

### Roadmap
As this project is a learning experience, this is not meant to be high performant chain. In spirit of heading that route, below is a rough road map to implement

1. Node Server that accepts transactions and adds them to the blockchain
2. Web Server UI to allow users to interact with node
3. POW or POS mechanism to allow nodes to mine blocks (Currently there are no constraints on block additions)
4. Consensus and message passing with an additional node

#### Stretch goals
5. Robust node architecture and consensus that would allow permissionless entry of nodes to the system over the internet
6. Smart contract implementation


#### Improvements
1. Testing, this program needs significant test functions on each individual funciton to ensure proper configurations
2. An improvement to investigate is stronger error handling amongst function calls. Right now, an error crashed the program rather than proper handling in specific areas.
3. Proper Rust naming convention