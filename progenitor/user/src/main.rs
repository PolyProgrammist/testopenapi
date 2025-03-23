use keeper::Client;
use serde_json::json;
use std::error::Error;

// const NEAR_RPC_URL: &str = "https://archival-rpc.mainnet.near.org";
const NEAR_RPC_URL: &str = "http://localhost:3030/";

async fn print_transaction() -> Result<(), Box<dyn Error>> {
    // 
    // let transaction_hash = "7LGhv7GUTUX3th4Dm2AmEHCWV3Z9C2G1SDJUZXGwkJPv"; // Replace with your TX hash

    let transaction_hash = "6zpAGJMsPAcTRm7vuD4V6tgevx7suVwXTQ56DRgfZ5ft";

    let transaction_hash = "9FtHUFBQsZ2MG77K3x3MJ9wjX3UT8zE1TczCrhZEcG8U"; // Replace with your TX hash
    let sender_account_id = "miraclx.near"; // Replace with sender's account

    // let client = Client::new(NEAR_RPC_URL);

    let payloadTx = keeper::types::JsonRpcRequestForTxMethodNameHelperEnum {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: keeper::types::TxMethodNameHelperEnum::Tx,
        params: keeper::types::RpcTransactionStatusRequest::Variant1 {
            tx_hash: transaction_hash.parse().unwrap(),
            sender_account_id: sender_account_id.parse().unwrap(),
            wait_until: keeper::types::TxExecutionStatus::None,
        }
    };

    let payloadBlock = keeper::types::JsonRpcRequestForBlockMethodNameHelperEnum {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: keeper::types::BlockMethodNameHelperEnum::Block,
        params: keeper::types::RpcBlockRequest::BlockId({
            keeper::types::BlockId::Variant1("Dxhrj21NWZYKi3DpCtQNtmhLj5sg6FwVVQCRn3EyLZLF".parse().unwrap())
        })
    };

    let payloadChunk = keeper::types::JsonRpcRequestForChunkMethodNameHelperEnum {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: keeper::types::ChunkMethodNameHelperEnum::Chunk,
        params: keeper::types::RpcChunkRequest::Variant0{
            block_id: keeper::types::BlockId::Variant1("Dxhrj21NWZYKi3DpCtQNtmhLj5sg6FwVVQCRn3EyLZLF".parse().unwrap()),
            shard_id: keeper::types::ShardId(0)
        }
    };

    let payloadGasPrice = keeper::types::JsonRpcRequestForGasPriceMethodNameHelperEnum {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: keeper::types::GasPriceMethodNameHelperEnum::GasPrice,
        params: keeper::types::RpcGasPriceRequest {
            block_id: None
        }
    };

    // let block: keeper::types::JsonRpcResponseForRpcBlockResponseAndRpcError = client.block(&payloadBlock).await?.into_inner();
    // // println!("block: {:#?}", block);

    // let tx: keeper::types::JsonRpcResponseForRpcTransactionResponseAndRpcError = client.tx(&payloadTx).await?.into_inner();
    // // println!("tx: {:#?}", tx);

    // let chunk: keeper::types::JsonRpcResponseForRpcChunkResponseAndRpcError = client.chunk(&payloadChunk).await?.into_inner();
    // // println!("chunk: {:#?}", chunk);

    // let gas_price: keeper::types::JsonRpcResponseForRpcGasPriceResponseAndRpcError = client.gas_price(&payloadGasPrice).await?.into_inner();
    // // println!("gas_price: {:#?}", gas_price);

    // Construct the correct JSON-RPC request
    let request_body = json!({
        "jsonrpc": "2.0",
        "id": "dontcare",
        "method": "gas_price",
        "params": [null]  // Pass block hash inside an array
    });

    let client_my = reqwest::Client::new();
    // Send the request to the NEAR RPC endpoint
    let response = client_my
        .post("https://127.0.0.1:3030/")  // Make sure URL is correct
        .header("Content-Type", "application/json") // Explicitly setting the JSON header
        .json(&request_body)
        .send()
        .await?;

    // Parse the response as JSON
    let response_json: serde_json::Value = response.json().await?;
    
    // Print the actual response
    println!("{:#?}", response_json);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    
    let txprinted = print_transaction().await;
    match txprinted {
        Ok(..) => {
            println!("hooray")
        }
        Err(err) => {
            println!("error {:#?}", err);
        }
    }

    Ok(())
}


// use reqwest::Client;
// use serde_json::json;

// #[tokio::main]
// async fn main() -> Result<(), reqwest::Error> {
//     let client = Client::new();

//     // JSON-RPC тело запроса
//     let request_body = json!({
//         "jsonrpc": "2.0",
//         "id": "dontcare",
//         "method": "gas_price",
//         "params": [null]  // Последний блок
//     });

//     // Отправка запроса на локальный RPC
//     let response = client
//         .post("http://127.0.0.1:3030")
//         .header("Content-Type", "application/json")
//         .json(&request_body)
//         .send()
//         .await?;

//     // Парсим ответ в JSON
//     let response_json: serde_json::Value = response.json().await?;

//     // Выводим ответ сервера
//     println!("{:#?}", response_json);

//     Ok(())
// }