use std::env; //Used to read env variables
use std::str::FromStr; //Imported to use Address:from_str

use web3::contract::{Contract, Options};
use web3::types::{Address, H160, U256};

fn wei_to_eth(wei_val: U256) -> f64 {
    let res = wei_val.as_u128() as f64;
    res / 1_000_000_000_000_000_000.0
}

//Denoted so it can be run in an async runtime
//Return value: web3
#[tokio::main]
async fn main() -> web3::Result<()> {
    //load variables
    dotenv::dotenv().ok();

    //create transport instance used to establish the connection to the Ethereum network.
    let websocket = web3::transports::WebSocket::new(&env::var("ALCHEMY_RINKEBY").unwrap()).await?;
    //web3 instance
    let web3s = web3::Web3::new(websocket);
    //retrieve list of acounts type is vec of H160
    let mut accounts = web3s.eth().accounts().await?;
    accounts.push(H160::from_str(&env::var("ACCOUNT_ADDRESS").unwrap()).unwrap());
    println!("Accounts: {:?}", accounts);
    //Loop Through list of accounts and call .balance() to get balance in wei
    //Then pass to get balance in ETH
    for account in accounts {
        let balance = web3s.eth().balance(account, None).await?;
        println!("Eth balance of {:?}: {}", account, wei_to_eth(balance));
    }

    let aave_addr = Address::from_str("0x42447d5f59d5bf78a82c34663474922bdf278162").unwrap();
    let token_contract =
        Contract::from_json(web3s.eth(), aave_addr, include_bytes!("aave_erc20_abi.json")).unwrap();
    //Query token name, output type is String
    let token_name: String = token_contract
        .query("name", (), None, Options::default(), None)
        .await
        .unwrap();
    let total_supply: U256 = token_contract
        .query("totalSupply", (), None, Options::default(), None)
        .await
        .unwrap();
    println!("Token name: {}, total supply: {}", token_name, total_supply);

    Ok(())
}
