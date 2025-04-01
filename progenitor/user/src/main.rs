use keeper::Client;
use serde_json::json;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

const NEAR_RPC_URL_REMOTE: &str = "https://archival-rpc.mainnet.near.org";
const NEAR_RPC_URL_LOCAL: &str = "http://localhost:3030";

async fn print_transaction() -> Result<(), Box<dyn Error>> {
    // 
    // let transaction_hash = "7LGhv7GUTUX3th4Dm2AmEHCWV3Z9C2G1SDJUZXGwkJPv"; // Replace with your TX hash

    let transaction_hash = "6zpAGJMsPAcTRm7vuD4V6tgevx7suVwXTQ56DRgfZ5ft";

    let transaction_hash = "9FtHUFBQsZ2MG77K3x3MJ9wjX3UT8zE1TczCrhZEcG8U"; // Replace with your TX hash
    let block_hash = "8jvfhbzGnCcYjkCrnqnzDr8bXBbFLUXaoVt6jsRNB1pU";
    let sender_account_id = "miraclx.near"; // Replace with sender's account

    let client_remote = Client::new(NEAR_RPC_URL_REMOTE);
    let client_local = Client::new(NEAR_RPC_URL_LOCAL);

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
            block_id: Some(keeper::types::BlockId::Variant1("Dxhrj21NWZYKi3DpCtQNtmhLj5sg6FwVVQCRn3EyLZLF".parse().unwrap()))
        }
    };

    let payloadHealth = keeper::types::JsonRpcRequestForHealthMethodNameHelperEnum {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: keeper::types::HealthMethodNameHelperEnum::Health,
        params: keeper::types::RpcHealthRequest(serde_json::Map::new())
    };

    let payloadBroadcastCommit = keeper::types::JsonRpcRequestForBroadCastTxCommitMethodNameHelperEnum {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: keeper::types::BroadCastTxCommitMethodNameHelperEnum::BroadcastTxCommit,
        params: keeper::types::RpcSendTransactionRequest {
            signed_tx_base64: "DgAAAHNlbmRlci50ZXN0bmV0AOrmAai64SZOv9e/naX4W15pJx0GAap35wTT1T/DwcbbDwAAAAAAAAAQAAAAcmVjZWl2ZXIudGVzdG5ldNMnL7URB1cxPOu3G8jTqlEwlcasagIbKlAJlF5ywVFLAQAAAAMAAACh7czOG8LTAAAAAAAAAGQcOG03xVSFQFjoagOb4NBBqWhERnnz45LY4+52JgZhm1iQKz7qAdPByrGFDQhQ2Mfga8RlbysuQ8D8LlA6bQE=".to_string(),
            wait_until: keeper::types::TxExecutionStatus::Executed
        }
    };

    let payloadBroadcastAsync = keeper::types::JsonRpcRequestForBroadCastTxAsyncMethodNameHelperEnum {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: keeper::types::BroadCastTxAsyncMethodNameHelperEnum::BroadcastTxAsync,
        params: keeper::types::RpcSendTransactionRequest {
            signed_tx_base64: "DgAAAHNlbmRlci50ZXN0bmV0AOrmAai64SZOv9e/naX4W15pJx0GAap35wTT1T/DwcbbDwAAAAAAAAAQAAAAcmVjZWl2ZXIudGVzdG5ldNMnL7URB1cxPOu3G8jTqlEwlcasagIbKlAJlF5ywVFLAQAAAAMAAACh7czOG8LTAAAAAAAAAGQcOG03xVSFQFjoagOb4NBBqWhERnnz45LY4+52JgZhm1iQKz7qAdPByrGFDQhQ2Mfga8RlbysuQ8D8LlA6bQE=".to_string(),
            wait_until: keeper::types::TxExecutionStatus::Executed
        }
    };

    let payloadLightClientExecutionProof = keeper::types::JsonRpcRequestForLightClientProofMethodNameHelperEnum {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: keeper::types::LightClientProofMethodNameHelperEnum::LightClientProof,
        params: keeper::types::RpcLightClientExecutionProofRequest::Variant0 {
            light_client_head: transaction_hash.parse().unwrap(),
            sender_id: sender_account_id.parse().unwrap(),
            transaction_hash: transaction_hash.parse().unwrap(),
            type_: keeper::types::TypeTransactionOrReceiptId::Transaction,
        }
    };

    let payloadNextLightClientBlock = keeper::types::JsonRpcRequestForNextLightClientBlockMethodNameHelperEnum {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: keeper::types::NextLightClientBlockMethodNameHelperEnum::NextLightClientBlock,
        params: keeper::types::RpcLightClientNextBlockRequest {
            last_block_hash: "Dxhrj21NWZYKi3DpCtQNtmhLj5sg6FwVVQCRn3EyLZLF".parse().unwrap(),
        }
    };

    let payloadNetworkInfo = keeper::types::JsonRpcRequestForNetworkInfoMethodNameHelperEnum {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: keeper::types::NetworkInfoMethodNameHelperEnum::NetworkInfo,
        params: keeper::types::RpcNetworkInfoRequest(serde_json::Map::new())
    };

    let payloadSendTx = keeper::types::JsonRpcRequestForSendTxMethodNameHelperEnum {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: keeper::types::SendTxMethodNameHelperEnum::SendTx,
        params: keeper::types::RpcSendTransactionRequest {
            signed_tx_base64: "DgAAAHNlbmRlci50ZXN0bmV0AOrmAai64SZOv9e/naX4W15pJx0GAap35wTT1T/DwcbbDwAAAAAAAAAQAAAAcmVjZWl2ZXIudGVzdG5ldNMnL7URB1cxPOu3G8jTqlEwlcasagIbKlAJlF5ywVFLAQAAAAMAAACh7czOG8LTAAAAAAAAAGQcOG03xVSFQFjoagOb4NBBqWhERnnz45LY4+52JgZhm1iQKz7qAdPByrGFDQhQ2Mfga8RlbysuQ8D8LlA6bQE=".to_string(),
            wait_until: keeper::types::TxExecutionStatus::Executed
        }
    };

    let payloadStatus = keeper::types::JsonRpcRequestForStatusMethodNameHelperEnum {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: keeper::types::StatusMethodNameHelperEnum::Status,
        params: keeper::types::RpcStatusRequest(serde_json::Map::new())
    };

    let payloadValidators = keeper::types::JsonRpcRequestForValidatorsMethodNameHelperEnum {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: keeper::types::ValidatorsMethodNameHelperEnum::Validators,
        params: keeper::types::RpcValidatorRequest::Latest
    };

    let payloadClientConfig = keeper::types::JsonRpcRequestForClientConfigMethodNameHelperEnum {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: keeper::types::ClientConfigMethodNameHelperEnum::ClientConfig,
        params: keeper::types::RpcClientConfigRequest(serde_json::Map::new())
    };

    let block: keeper::types::JsonRpcResponseForRpcBlockResponseAndRpcError = client_remote.block(&payloadBlock).await?.into_inner();
    println!("block: {:#?}", block);

    let tx: keeper::types::JsonRpcResponseForRpcTransactionResponseAndRpcError = client_remote.tx(&payloadTx).await?.into_inner();
    println!("tx: {:#?}", tx);

    let chunk: keeper::types::JsonRpcResponseForRpcChunkResponseAndRpcError = client_remote.chunk(&payloadChunk).await?.into_inner();
    println!("chunk: {:#?}", chunk);

    // local as currently accepts only array, fixed in new version
    let gas_price: keeper::types::JsonRpcResponseForRpcGasPriceResponseAndRpcError = client_local.gas_price(&payloadGasPrice).await?.into_inner();
    println!("gas_price: {:#?}", gas_price);

    let health: keeper::types::JsonRpcResponseForNullableRpcHealthResponseAndRpcError = client_remote.health(&payloadHealth).await?.into_inner();
    println!("health: {:#?}", health);

    let broadcast_commit: keeper::types::JsonRpcResponseForRpcTransactionResponseAndRpcError = client_remote.broadcast_tx_commit(&payloadBroadcastCommit).await?.into_inner();
    println!("broadcast_commit: {:#?}", broadcast_commit);
    
    let broadcast_async: keeper::types::JsonRpcResponseForCryptoHashAndRpcError = client_remote.broadcast_tx_async(&payloadBroadcastAsync).await?.into_inner();
    println!("broadcast_async: {:#?}", broadcast_async);

    let light_client_execution_proof: keeper::types::JsonRpcResponseForRpcLightClientExecutionProofResponseAndRpcError = client_remote.light_client_proof(&payloadLightClientExecutionProof).await?.into_inner();
    println!("light_client_execution_proof: {:#?}", light_client_execution_proof);

    let next_light_client_block: keeper::types::JsonRpcResponseForRpcLightClientNextBlockResponseAndRpcError = client_remote.next_light_client_block(&payloadNextLightClientBlock).await?.into_inner();
    println!("next_light_client_block: {:#?}", next_light_client_block);

    let network_info: keeper::types::JsonRpcResponseForRpcNetworkInfoResponseAndRpcError = client_remote.network_info(&payloadNetworkInfo).await?.into_inner();
    println!("network_info: {:#?}", network_info);

    let send_tx: keeper::types::JsonRpcResponseForRpcTransactionResponseAndRpcError = client_remote.send_tx(&payloadSendTx).await?.into_inner();
    println!("send_tx: {:#?}", send_tx);

    // local as ".version.commit" introduced recently: https://github.com/near/nearcore/pull/12722/files
    let status = client_local.status(&payloadStatus).await?;
    println!("status: {:#?}", status);

    let validators: keeper::types::JsonRpcResponseForRpcValidatorResponseAndRpcError = client_remote.validators(&payloadValidators).await?.into_inner();
    println!("validators: {:#?}", validators);

    let client_config: keeper::types::JsonRpcResponseForRpcClientConfigResponseAndRpcError = client_local.client_config(&payloadClientConfig).await?.into_inner();
    println!("client_config: {:#?}", client_config);

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


    // let path = "tmp.json";
    // let file = File::open(path)?;
    // let reader = BufReader::new(file);
    
    // // Parse the JSON directly into the desired type
    // // let config: keeper::types::JsonRpcResponseForRpcClientConfigResponseAndRpcError = serde_json::from_reader(reader)?;
    // // let onlyresharding: keeper::types::RpcClientConfigResponseOnlyResharding = serde_json::from_reader(reader)?;


