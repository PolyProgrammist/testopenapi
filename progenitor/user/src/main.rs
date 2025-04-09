use keeper::Client;
use serde_json::json;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
// use keeper::types::CryptoHash;

const NEAR_RPC_URL_REMOTE: &str = "https://archival-rpc.mainnet.near.org";
const NEAR_RPC_URL_LOCAL: &str = "http://localhost:3030";

async fn print_transaction() -> Result<(), Box<dyn Error>> {
    // let transaction_hash: CryptoHash = "9FtHUFBQsZ2MG77K3x3MJ9wjX3UT8zE1TczCrhZEcG8U".parse().unwrap(); // Replace with your TX hash
    // let block_hash: CryptoHash = "Dxhrj21NWZYKi3DpCtQNtmhLj5sg6FwVVQCRn3EyLZLF".parse().unwrap();
    let sender_account_id: keeper::types::AccountId = "miraclx.near".parse().unwrap();
    let signed_tx_base64 = "DgAAAHNlbmRlci50ZXN0bmV0AOrmAai64SZOv9e/naX4W15pJx0GAap35wTT1T/DwcbbDwAAAAAAAAAQAAAAcmVjZWl2ZXIudGVzdG5ldNMnL7URB1cxPOu3G8jTqlEwlcasagIbKlAJlF5ywVFLAQAAAAMAAACh7czOG8LTAAAAAAAAAGQcOG03xVSFQFjoagOb4NBBqWhERnnz45LY4+52JgZhm1iQKz7qAdPByrGFDQhQ2Mfga8RlbysuQ8D8LlA6bQE=".to_string();

    let client_remote = Client::new(NEAR_RPC_URL_REMOTE);
    let client_local = Client::new(NEAR_RPC_URL_LOCAL);

    // let payloadBlock = keeper::types::JsonRpcRequestForBlockMethodNameHelperEnum {
    //     id: String::from("dontcare"),
    //     jsonrpc: String::from("2.0"),
    //     method: keeper::types::BlockMethodNameHelperEnum::Block,
    //     params: keeper::types::RpcBlockRequest::BlockId({
    //         keeper::types::BlockId::Variant1(block_hash.clone())
    //     })
    // };

    // let payloadBroadcastAsync = keeper::types::JsonRpcRequestForBroadCastTxAsyncMethodNameHelperEnum {
    //     id: String::from("dontcare"),
    //     jsonrpc: String::from("2.0"),
    //     method: keeper::types::BroadCastTxAsyncMethodNameHelperEnum::BroadcastTxAsync,
    //     params: keeper::types::RpcSendTransactionRequest {
    //         signed_tx_base64: signed_tx_base64.clone(),
    //         wait_until: Some(keeper::types::TxExecutionStatus::Executed)
    //     }
    // };

    // let payloadBroadcastCommit = keeper::types::JsonRpcRequestForBroadCastTxCommitMethodNameHelperEnum {
    //     id: String::from("dontcare"),
    //     jsonrpc: String::from("2.0"),
    //     method: keeper::types::BroadCastTxCommitMethodNameHelperEnum::BroadcastTxCommit,
    //     params: keeper::types::RpcSendTransactionRequest {
    //         signed_tx_base64: signed_tx_base64.clone(),
    //         wait_until: Some(keeper::types::TxExecutionStatus::Executed)
    //     }
    // };

    // let payloadChunk = keeper::types::JsonRpcRequestForChunkMethodNameHelperEnum {
    //     id: String::from("dontcare"),
    //     jsonrpc: String::from("2.0"),
    //     method: keeper::types::ChunkMethodNameHelperEnum::Chunk,
    //     params: keeper::types::RpcChunkRequest::Variant0{
    //         block_id: keeper::types::BlockId::Variant1(block_hash.clone()),
    //         shard_id: keeper::types::ShardId(0)
    //     }
    // };

    // let payloadGasPriceWithBlock = keeper::types::JsonRpcRequestForGasPriceMethodNameHelperEnum {
    //     id: String::from("dontcare"),
    //     jsonrpc: String::from("2.0"),
    //     method: keeper::types::GasPriceMethodNameHelperEnum::GasPrice,
    //     params: keeper::types::RpcGasPriceRequest {
    //         block_id: Some(keeper::types::BlockId::Variant1(block_hash.clone()))
    //     }
    // };

    // let payloadGasPriceWithoutBlock = keeper::types::JsonRpcRequestForGasPriceMethodNameHelperEnum {
    //     id: String::from("dontcare"),
    //     jsonrpc: String::from("2.0"),
    //     method: keeper::types::GasPriceMethodNameHelperEnum::GasPrice,
    //     params: keeper::types::RpcGasPriceRequest {
    //         block_id: None
    //     }
    // };

    // let payloadHealth = keeper::types::JsonRpcRequestForHealthMethodNameHelperEnum {
    //     id: String::from("dontcare"),
    //     jsonrpc: String::from("2.0"),
    //     method: keeper::types::HealthMethodNameHelperEnum::Health,
    //     params: keeper::types::RpcHealthRequest(serde_json::Map::new())
    // };

    // let payloadLightClientExecutionProof = keeper::types::JsonRpcRequestForLightClientProofMethodNameHelperEnum {
    //     id: String::from("dontcare"),
    //     jsonrpc: String::from("2.0"),
    //     method: keeper::types::LightClientProofMethodNameHelperEnum::LightClientProof,
    //     params: keeper::types::RpcLightClientExecutionProofRequest::Variant0 {
    //         light_client_head: block_hash.clone(),
    //         sender_id: sender_account_id.clone(),
    //         transaction_hash: transaction_hash.clone(),
    //         type_: keeper::types::TypeTransactionOrReceiptId::Transaction,
    //     }
    // };

    // let payloadNextLightClientBlock = keeper::types::JsonRpcRequestForNextLightClientBlockMethodNameHelperEnum {
    //     id: String::from("dontcare"),
    //     jsonrpc: String::from("2.0"),
    //     method: keeper::types::NextLightClientBlockMethodNameHelperEnum::NextLightClientBlock,
    //     params: keeper::types::RpcLightClientNextBlockRequest {
    //         last_block_hash: block_hash.clone(),
    //     }
    // };

    // let payloadNetworkInfo = keeper::types::JsonRpcRequestForNetworkInfoMethodNameHelperEnum {
    //     id: String::from("dontcare"),
    //     jsonrpc: String::from("2.0"),
    //     method: keeper::types::NetworkInfoMethodNameHelperEnum::NetworkInfo,
    //     params: keeper::types::RpcNetworkInfoRequest(serde_json::Map::new())
    // };

    // let payloadSendTx = keeper::types::JsonRpcRequestForSendTxMethodNameHelperEnum {
    //     id: String::from("dontcare"),
    //     jsonrpc: String::from("2.0"),
    //     method: keeper::types::SendTxMethodNameHelperEnum::SendTx,
    //     params: keeper::types::RpcSendTransactionRequest {
    //         signed_tx_base64: signed_tx_base64.clone(),
    //         wait_until: Some(keeper::types::TxExecutionStatus::Executed)
    //     }
    // };

    // let payloadTx = keeper::types::JsonRpcRequestForTxMethodNameHelperEnum {
    //     id: String::from("dontcare"),
    //     jsonrpc: String::from("2.0"),
    //     method: keeper::types::TxMethodNameHelperEnum::Tx,
    //     params: keeper::types::RpcTransactionStatusRequest::Variant1 {
    //         tx_hash: transaction_hash.clone(),
    //         sender_account_id: sender_account_id.clone(),
    //         wait_until: Some(keeper::types::TxExecutionStatus::None),
    //     }
    // };

    // let payloadStatus = keeper::types::JsonRpcRequestForStatusMethodNameHelperEnum {
    //     id: String::from("dontcare"),
    //     jsonrpc: String::from("2.0"),
    //     method: keeper::types::StatusMethodNameHelperEnum::Status,
    //     params: keeper::types::RpcStatusRequest(serde_json::Map::new())
    // };

    // let payloadValidators = keeper::types::JsonRpcRequestForValidatorsMethodNameHelperEnum {
    //     id: String::from("dontcare"),
    //     jsonrpc: String::from("2.0"),
    //     method: keeper::types::ValidatorsMethodNameHelperEnum::Validators,
    //     params: keeper::types::RpcValidatorRequest::Latest
    // };

    // let payloadClientConfig = keeper::types::JsonRpcRequestForClientConfigMethodNameHelperEnum {
    //     id: String::from("dontcare"),
    //     jsonrpc: String::from("2.0"),
    //     method: keeper::types::ClientConfigMethodNameHelperEnum::ClientConfig,
    //     params: keeper::types::RpcClientConfigRequest(serde_json::Map::new())
    // };

    // let payloadStateChanges = keeper::types::JsonRpcRequestForExpChangeMethodNameHelperEnum {
    //     id: String::from("dontcare"),
    //     jsonrpc: String::from("2.0"),
    //     method: keeper::types::ExpChangeMethodNameHelperEnum::ExperimentalChanges,
    //     params: keeper::types::RpcStateChangesInBlockByTypeRequest::Variant0 {
    //         changes_type: keeper::types::RpcStateChangesInBlockByTypeRequestVariant0ChangesType::AccountChanges,
    //         account_ids: vec!["token.sweat".parse().unwrap()],
    //         block_id: keeper::types::BlockId::Variant1(block_hash.clone()),
    //     }
    // };

    // let payloadChangesInBlock = keeper::types::JsonRpcRequestForExpChangesBlockMethodNameHelperEnum {
    //     id: String::from("dontcare"),
    //     jsonrpc: String::from("2.0"),
    //     method: keeper::types::ExpChangesBlockMethodNameHelperEnum::ExperimentalChangesInBlock,
    //     params: keeper::types::RpcStateChangesInBlockRequest::BlockId(keeper::types::BlockId::Variant1(block_hash.clone()))
    // };

    // let payloadCongestionLevel = keeper::types::JsonRpcRequestForExpGongestionMethodNameHelperEnum {
    //     id: String::from("dontcare"),
    //     jsonrpc: String::from("2.0"),
    //     method: keeper::types::ExpGongestionMethodNameHelperEnum::ExperimentalCongestionLevel,
    //     params: keeper::types::RpcCongestionLevelRequest::Variant0 {
    //         block_id: keeper::types::BlockId::Variant1(block_hash.clone()),
    //         shard_id: keeper::types::ShardId(0)
    //     }
    // };

    // let payloadGenesisConfig = keeper::types::JsonRpcRequestForExpGenesisMethodNameHelperEnum {
    //     id: String::from("dontcare"),
    //     jsonrpc: String::from("2.0"),
    //     method: keeper::types::ExpGenesisMethodNameHelperEnum::ExperimentalGenesisConfig,
    //     params: keeper::types::GenesisConfigRequest(serde_json::Map::new())
    // };

    // let payloadExpLightClientExecutionProof = keeper::types::JsonRpcRequestForExpLightClientProofMethodNameHelperEnum {
    //     id: String::from("dontcare"),
    //     jsonrpc: String::from("2.0"),
    //     method: keeper::types::ExpLightClientProofMethodNameHelperEnum::ExperimentalLightClientProof,
    //     params: keeper::types::RpcLightClientExecutionProofRequest::Variant0 {
    //         light_client_head: block_hash.clone(),
    //         sender_id: sender_account_id.clone(),
    //         transaction_hash: transaction_hash.clone(),
    //         type_: keeper::types::TypeTransactionOrReceiptId::Transaction,
    //     }
    // };

    // let payloadExpLightClientBlock = keeper::types::JsonRpcRequestForExpLightClientBlockProofMethodNameHelperEnum {
    //     id: String::from("dontcare"),
    //     jsonrpc: String::from("2.0"),
    //     method: keeper::types::ExpLightClientBlockProofMethodNameHelperEnum::ExperimentalLightClientBlockProof,
    //     params: keeper::types::RpcLightClientBlockProofRequest {
    //         block_hash: block_hash.clone(),
    //         light_client_head: block_hash.clone(),
    //     }
    // };

    // let payloadProtocolConfig = keeper::types::JsonRpcRequestForExpProtocolConfigMethodNameHelperEnum {
    //     id: String::from("dontcare"),
    //     jsonrpc: String::from("2.0"),
    //     method: keeper::types::ExpProtocolConfigMethodNameHelperEnum::ExperimentalProtocolConfig,
    //     params: keeper::types::RpcProtocolConfigRequest::BlockId(keeper::types::BlockId::Variant1(block_hash.clone()))
    // };

    // let payloadReceipt = keeper::types::JsonRpcRequestForExpReceiptMethodNameHelperEnum {
    //     id: String::from("dontcare"),
    //     jsonrpc: String::from("2.0"),
    //     method: keeper::types::ExpReceiptMethodNameHelperEnum::ExperimentalReceipt,
    //     params: keeper::types::RpcReceiptRequest {
    //         receipt_id: "GVpXUxpyo715x7fcvFuzJMJ1zimU1vCJggVwMyGAM6oH".parse().unwrap(),
    //     }
    // };

    // let payloadExpTxStatus = keeper::types::JsonRpcRequestForExpTxStatusMethodNameHelperEnum {
    //     id: String::from("dontcare"),
    //     jsonrpc: String::from("2.0"),
    //     method: keeper::types::ExpTxStatusMethodNameHelperEnum::ExperimentalTxStatus,
    //     params: keeper::types::RpcTransactionStatusRequest::Variant1 {
    //         tx_hash: transaction_hash.clone(),
    //         sender_account_id: sender_account_id.clone(),
    //         wait_until: Some(keeper::types::TxExecutionStatus::None),
    //     }
    // };

    // let payloadExpValidators = keeper::types::JsonRpcRequestForExpValidatorsMethodNameHelperEnum {
    //     id: String::from("dontcare"),
    //     jsonrpc: String::from("2.0"),
    //     method: keeper::types::ExpValidatorsMethodNameHelperEnum::ExperimentalValidatorsOrdered,
    //     params: keeper::types::RpcValidatorsOrderedRequest {
    //         block_id: None
    //     }
    // };

    let payloadMaintenanceWindows = keeper::types::JsonRpcRequestForRpcMaintenanceWindowsRequest {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: keeper::types::JsonRpcRequestForRpcMaintenanceWindowsRequestMethod::ExperimentalMaintenanceWindows,
        params: keeper::types::RpcMaintenanceWindowsRequest {
            account_id: sender_account_id.clone(),
        }
    };

    let payloadSplitStorage = keeper::types::JsonRpcRequestForRpcSplitStorageInfoRequest {
        id: String::from("dontcare"),
        jsonrpc: String::from("2.0"),
        method: keeper::types::JsonRpcRequestForRpcSplitStorageInfoRequestMethod::ExperimentalSplitStorageInfo,
        params: keeper::types::RpcSplitStorageInfoRequest(serde_json::Map::new())
    };

    // let block: keeper::types::JsonRpcResponseForRpcBlockResponseAndRpcError = client_remote.block(&payloadBlock).await?.into_inner();
    // println!("the_response block: {:#?}", block);

    // let broadcast_async: keeper::types::JsonRpcResponseForCryptoHashAndRpcError = client_remote.broadcast_tx_async(&payloadBroadcastAsync).await?.into_inner();
    // println!("the_response broadcast_async: {:#?}", broadcast_async);

    // let broadcast_commit: keeper::types::JsonRpcResponseForRpcTransactionResponseAndRpcError = client_remote.broadcast_tx_commit(&payloadBroadcastCommit).await?.into_inner();
    // println!("the_response broadcast_commit: {:#?}", broadcast_commit);
    
    // let chunk: keeper::types::JsonRpcResponseForRpcChunkResponseAndRpcError = client_remote.chunk(&payloadChunk).await?.into_inner();
    // println!("the_response chunk: {:#?}", chunk);

    // // local as currently accepts only array, fixed in new version
    // let gas_price_with_block: keeper::types::JsonRpcResponseForRpcGasPriceResponseAndRpcError = client_local.gas_price(&payloadGasPriceWithBlock).await?.into_inner();
    // println!("the_response gas_price_with_block: {:#?}", gas_price_with_block);

    // let gas_price_without_block: keeper::types::JsonRpcResponseForRpcGasPriceResponseAndRpcError = client_local.gas_price(&payloadGasPriceWithoutBlock).await?.into_inner();
    // println!("the_response gas_price_without_block: {:#?}", gas_price_without_block);

    // let health: keeper::types::JsonRpcResponseForNullableRpcHealthResponseAndRpcError = client_remote.health(&payloadHealth).await?.into_inner();
    // println!("the_response health: {:#?}", health);

    // let light_client_execution_proof: keeper::types::JsonRpcResponseForRpcLightClientExecutionProofResponseAndRpcError = client_remote.light_client_proof(&payloadLightClientExecutionProof).await?.into_inner();
    // println!("the_response light_client_execution_proof: {:#?}", light_client_execution_proof);

    // let next_light_client_block: keeper::types::JsonRpcResponseForRpcLightClientNextBlockResponseAndRpcError = client_remote.next_light_client_block(&payloadNextLightClientBlock).await?.into_inner();
    // println!("the_response next_light_client_block: {:#?}", next_light_client_block);

    // let network_info: keeper::types::JsonRpcResponseForRpcNetworkInfoResponseAndRpcError = client_remote.network_info(&payloadNetworkInfo).await?.into_inner();
    // println!("the_response network_info: {:#?}", network_info);

    // let send_tx: keeper::types::JsonRpcResponseForRpcTransactionResponseAndRpcError = client_remote.send_tx(&payloadSendTx).await?.into_inner();
    // println!("the_response send_tx: {:#?}", send_tx);

    // let tx: keeper::types::JsonRpcResponseForRpcTransactionResponseAndRpcError = client_remote.tx(&payloadTx).await?.into_inner();
    // println!("the_response tx: {:#?}", tx);

    // // local as ".version.commit" introduced recently: https://github.com/near/nearcore/pull/12722/files
    // let status = client_local.status(&payloadStatus).await?;
    // println!("the_response status: {:#?}", status);

    // let validators: keeper::types::JsonRpcResponseForRpcValidatorResponseAndRpcError = client_remote.validators(&payloadValidators).await?.into_inner();
    // println!("the_response validators: {:#?}", validators);

    // let client_config: keeper::types::JsonRpcResponseForRpcClientConfigResponseAndRpcError = client_local.client_config(&payloadClientConfig).await?.into_inner();
    // println!("the_response client_config: {:#?}", client_config);

    // let experimental_changes: keeper::types::JsonRpcResponseForRpcStateChangesInBlockResponseAndRpcError = client_remote.experimental_changes(&payloadStateChanges).await?.into_inner();
    // println!("the_response experimental_changes: {:#?}", experimental_changes);

    // let experimental_changes_in_block: keeper::types::JsonRpcResponseForRpcStateChangesInBlockByTypeResponseAndRpcError = client_remote.experimental_changes_in_block(&payloadChangesInBlock).await?.into_inner();
    // println!("the_response experimental_changes_in_block: {:#?}", experimental_changes_in_block);

    // let congestion_level: keeper::types::JsonRpcResponseForRpcCongestionLevelResponseAndRpcError = client_remote.experimental_congestion_level(&payloadCongestionLevel).await?.into_inner();
    // println!("the_response congestion_level: {:#?}", congestion_level);

    // let genesis_config: keeper::types::JsonRpcResponseForGenesisConfigAndRpcError = client_remote.experimental_genesis_config(&payloadGenesisConfig).await?.into_inner();
    // println!("the_response genesis_config: {:#?}", genesis_config);

    // let experimental_light_client_execution_proof: keeper::types::JsonRpcResponseForRpcLightClientExecutionProofResponseAndRpcError = client_remote.experimental_light_client_proof(&payloadExpLightClientExecutionProof).await?.into_inner();
    // println!("the_response experimental_light_client_execution_proof: {:#?}", experimental_light_client_execution_proof);

    // let experimental_next_light_client_block: keeper::types::JsonRpcResponseForRpcLightClientBlockProofResponseAndRpcError = client_remote.experimental_light_client_block_proof(&payloadExpLightClientBlock).await?.into_inner();
    // println!("the_response experimental_next_light_client_block: {:#?}", experimental_next_light_client_block);

    // let experimental_protocol_config: keeper::types::JsonRpcResponseForRpcProtocolConfigResponseAndRpcError = client_remote.experimental_protocol_config(&payloadProtocolConfig).await?.into_inner();
    // println!("the_response experimental_protocol_config: {:#?}", experimental_protocol_config);

    // let experimental_receipt: keeper::types::JsonRpcResponseForRpcReceiptResponseAndRpcError = client_remote.experimental_receipt(&payloadReceipt).await?.into_inner();
    // println!("the_response experimental_receipt: {:#?}", experimental_receipt);

    // let experimental_tx_status: keeper::types::JsonRpcResponseForRpcTransactionResponseAndRpcError = client_remote.experimental_tx_status(&payloadExpTxStatus).await?.into_inner();
    // println!("the_response experimental_tx_status: {:#?}", experimental_tx_status);

    // let experimental_validators: keeper::types::JsonRpcResponseForArrayOfValidatorStakeViewAndRpcError = client_remote.experimental_validators_ordered(&payloadExpValidators).await?.into_inner();
    // println!("the_response experimental_validators: {:#?}", experimental_validators);

    // local as changed from tuple to struct
    let experimental_maintenance_windows: keeper::types::JsonRpcResponseForArrayOfMaintenanceWindowAndRpcError = client_remote.experimental_maintenance_windows(&payloadMaintenanceWindows).await?.into_inner();
    println!("the_response experimental_maintenance_windows: {:#?}", experimental_maintenance_windows);

    let experimental_split_storage: keeper::types::JsonRpcResponseForRpcSplitStorageInfoResponseAndRpcError = client_remote.experimental_split_storage_info(&payloadSplitStorage).await?.into_inner();
    println!("the_response experimental_split_storage: {:#?}", experimental_split_storage);

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
