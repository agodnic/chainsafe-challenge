use anyhow::{anyhow, Context, Result};
use ipfs_api::IpfsClient;
use std::env;
use std::fs::File;
use std::str::FromStr;
use web3::types::{H160, H256};

/// Config is a POD struct that stores general configuration settings for this program.
struct Config {
    path_to_file: String,
    ganache_url: String,
    contract_address: H160,
    contract_call_address: H160,
}

impl Config {
    /// from_env initializes program settings from environment variables.
    fn from_env() -> Result<Config> {
        Ok(Config {
            path_to_file: env::var("PATH_TO_FILE")?,
            ganache_url: env::var("GANACHE_URL")?,
            contract_address: H160::from_str(&env::var("CONTRACT_ADDRESS")?)?,
            contract_call_address: H160::from_str(&env::var("CONTRACT_CALL_ADDRESS")?)?,
        })
    }
}

/// store_file stores a given file on IPFS.
///
/// When successful, returns a string that corresponds to the file's CID on ipfs.
async fn store_file(path: &str) -> Result<String> {
    let file = File::open(path).with_context(|| format!("failed to read file {}", path))?;

    let client = IpfsClient::default();

    client
        .add(file)
        .await
        .map_err(|e| anyhow!("failed to store file on ipfs: {}", e))
        .map(|res| res.hash)
}

/// store_cid saves an ipfs CID on an ethereum smart contract.
///
/// When successful, returns the transaction hash associated to the smart contract call.
async fn store_cid(
    ganache_url: &str,
    contract_address: &H160,
    contract_call_address: &H160,
    cid: &str,
) -> Result<H256> {
    let http = web3::transports::Http::new(ganache_url).unwrap();
    let web3 = web3::Web3::new(http);

    let contract = web3::contract::Contract::from_json(
        web3.eth(),
        *contract_address,
        include_bytes!("../contracts/abi"),
    )?;

    let tx = contract
        .call(
            "setCid",
            (cid.to_string(),),
            *contract_call_address,
            web3::contract::Options::default(),
        )
        .await?;

    Ok(tx)
}

#[tokio::main]
async fn main() -> Result<()> {
    // populate configuration settings from environment variables
    ::dotenv::dotenv()?;
    let config = Config::from_env()?;

    // upload the file to IPFS, get the CID
    println!("storing file {} on ipfs", &config.path_to_file);
    let cid = store_file(&config.path_to_file).await?;
    println!("uploaded file with cid={}", &cid);

    // save the CID into the smart contract
    println!("saving cid on a smart contract");
    let tx_hash = store_cid(
        &config.ganache_url,
        &config.contract_address,
        &config.contract_call_address,
        &cid,
    )
    .await?;
    println!("saved cid on a smart contract (tx hash: {})", tx_hash);

    Ok(())
}
