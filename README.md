
# chainsafe technical challenge
In this repository you'll find my attempt at solving the technical challenge for a Rust engineer role at Chainsafe.

## Prerequisites / initial setup
rustc >= 1.54 (may work on slightly older versions though)

For simplicity, I've chosen to develop this challenge with a setup that consists of a local ipfs node and ganache.

### IPFS setup
1. Install Go-IPFS: https://docs.ipfs.io/install/command-line/
2. Run a local IPFS daemon https://docs.ipfs.io/how-to/command-line-quick-start/

### Ganache + remix setup
1. I just used the ganache GUI: https://github.com/trufflesuite/ganache-ui/releases/download/v2.5.4/ganache-2.5.4-linux-x86_64.AppImage
2. I've used Remix to interact with ganache: https://remix.ethereum.org/. It was particularly useful to compile/deploy the smart contract.

## Project structure
The project consists of a simple binary crate. 

The smart contract's source, abi and bytecode are located on the `contracts/` directory.

## How to run
1. Take a look at the file `.env.sample`. Step 2) needs the variables shown in that file, which can be provided either in the form of a .env file, or just normally via the environment.
2. Running `make upload` will upload the input file to IPFS and store the CID in the deployed smart contract.

## Design decisions / other thoughts
This section outlines some decisions taken while solving the challenge.
* To keep the scope reasonably short, I've only considered Linux as a target OS.
* I've chosen to use Makefiles + environment variables to glue pieces together.
  I like the fact that it's simple, and it is good enough for the purpose of a small exercise.
  In the end I made only one target (`make deploy`), but when starting the project I thought I would have more.
* I've used remix to generate the smart contract's abi / bytecode.
  I could've created a Makefile target (i.e.: `make deploy`) to automate it, but I didn't have a lot of time left.
* Could've added a Makefile target to query the stored CID (i.e.: `make query`). This could have been implemented as a separate binary that issued a smart contract query to get the CID.
  I decided not to do this because it wasn't required in the specification document.
  That said, I've used remix to make sure the CID was being stored properly in the smart contract.