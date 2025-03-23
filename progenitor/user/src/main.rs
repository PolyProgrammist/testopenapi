use keeper::Client;
use serde_json::json;
use std::error::Error;

const NEAR_RPC_URL: &str = "https://archival-rpc.mainnet.near.org";
// const NEAR_RPC_URL: &str = "http://localhost:3030/";

async fn print_transaction() -> Result<(), Box<dyn Error>> {
    // 
    // let transaction_hash = "7LGhv7GUTUX3th4Dm2AmEHCWV3Z9C2G1SDJUZXGwkJPv"; // Replace with your TX hash

    let transaction_hash = "6zpAGJMsPAcTRm7vuD4V6tgevx7suVwXTQ56DRgfZ5ft";

    let transaction_hash = "9FtHUFBQsZ2MG77K3x3MJ9wjX3UT8zE1TczCrhZEcG8U"; // Replace with your TX hash
    let sender_account_id = "miraclx.near"; // Replace with sender's account

    let client = Client::new(NEAR_RPC_URL);

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

    let block: keeper::types::JsonRpcResponseForRpcBlockResponseAndRpcError = client.block(&payloadBlock).await?.into_inner();
    println!("block: {:#?}", block);

    let tx: keeper::types::JsonRpcResponseForRpcTransactionResponseAndRpcError = client.tx(&payloadTx).await?.into_inner();
    println!("tx: {:#?}", tx);

    let chunk: keeper::types::JsonRpcResponseForRpcChunkResponseAndRpcError = client.chunk(&payloadChunk).await?.into_inner();
    println!("chunk: {:#?}", chunk);

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
