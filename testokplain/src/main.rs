use schemars::JsonSchema;
use serde::{self, Deserialize, Serialize};
use near_jsonrpc_primitives::errors::{RpcRequestValidationErrorKind};
use okapi::openapi3::{OpenApi, SchemaObject};

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, schemars::JsonSchema)]
#[serde(untagged)]
pub enum CauseRpcErrorKind {
    RequestValidationError(RpcRequestValidationErrorKind),
    HandlerError(serde_json::Value),
    InternalError(serde_json::Value),
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, schemars::JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NameRpcErrorKind {
    RequestValidationError,
    HandlerError,
    InternalError,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, schemars::JsonSchema)]
pub struct RpcError {
    pub name: Option<NameRpcErrorKind>,
    pub cause: Option<CauseRpcErrorKind>,
    pub code: i64,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

#[derive(JsonSchema)]
#[serde(untagged)]
pub enum ResponseEither<T, E> {
    Success { result: T },
    Error { tmp: E },
    RpcError { error: RpcError }
}

#[derive(JsonSchema)]
#[allow(dead_code)] // Suppress fields never read
struct JsonRpcResponse<T, E> {
    jsonrpc: String,
    id: String,
    #[serde(flatten)]
    response_or_error: ResponseEither<T, E>,
}

trait MethodNameTrait {
    type S: JsonSchema;
    type T: JsonSchema;
}

#[derive(JsonSchema)]
#[allow(dead_code)] // Suppress fields never read
struct JsonRpcRequest<S: MethodNameTrait> {
    jsonrpc: String,
    id: String,
    params: S::T,
    method: S::S,
}

type SchemasMap = okapi::Map::<String, okapi::openapi3::SchemaObject>;
type PathsMap = okapi::Map::<String, okapi::openapi3::PathItem>;

fn schemas_map<T: JsonSchema>() -> SchemasMap {
    let settings = schemars::gen::SchemaSettings::openapi3();
    let generator = schemars::gen::SchemaGenerator::new(settings);

    let root_schema = generator.into_root_schema_for::<T>();

    let mut result: SchemasMap = root_schema.definitions.into_iter().map(|(k, v)| (k, v.into())).collect();
    let mut root_schema_copy = root_schema.schema;

    result.insert(T::schema_name(), root_schema_copy);
    result
}

fn paths_map(request_schema_name: String, response_schema_name: String, method_name: String) -> PathsMap {
    let request_body = okapi::openapi3::RequestBody {
        required: true,
        content: {
            let mut map = okapi::Map::new();
            map.insert(
                "application/json".to_string(),
                okapi::openapi3::MediaType {
                    schema: Some(SchemaObject{reference: Some(request_schema_name), ..Default::default()}),
                    ..Default::default()
                },
            );
            map
        },
        ..Default::default()
    };

    let mut responses = okapi::openapi3::Responses::default();
    responses.responses.insert(
        "200".to_string(),
        okapi::openapi3::Response {
            content: {
                let mut map = okapi::Map::new();
                map.insert(
                    "application/json".to_string(),
                    okapi::openapi3::MediaType {
                        schema: Some(SchemaObject{reference: Some(response_schema_name), ..Default::default()}),
                        ..Default::default()
                    },
                );
                map
            },
            ..Default::default()
        }.into(),
    );

    let operation = okapi::openapi3::Operation {
        operation_id: Some(method_name.clone()),
        request_body: Some(request_body.into()),
        responses,
        ..Default::default()
    };

    let mut paths = PathsMap::new();
    paths.insert(
        format!("/{}", method_name),
        okapi::openapi3::PathItem {
            post: Some(operation),
            ..Default::default()
        },
    );

    paths
}

fn add_spec_for_path_internal<RequestType: JsonSchema, ResponseType: JsonSchema>(all_schemas: &mut SchemasMap, all_paths: &mut PathsMap, method_name: String) {
    let request_map = schemas_map::<RequestType>();
    let response_map = schemas_map::<ResponseType>();

    let mut schemas = request_map;
    schemas.extend(response_map);

    let paths = paths_map(
        format!("#/components/schemas/{}", RequestType::schema_name()), 
        format!("#/components/schemas/{}", ResponseType::schema_name()),
        method_name
    );

    all_schemas.extend(schemas.clone());
    all_paths.extend(paths.clone());
}

fn add_spec_for_path<Request: JsonSchema + MethodNameTrait, Response: JsonSchema>(all_schemas: &mut SchemasMap, all_paths: &mut PathsMap, method_name: String) {
    add_spec_for_path_internal::<JsonRpcRequest<Request>, JsonRpcResponse<Response, RpcError>>(all_schemas, all_paths, method_name)
}

fn whole_spec(all_schemas: SchemasMap, all_paths: PathsMap) -> OpenApi {
    OpenApi {
        openapi: "3.0.0".to_string(),
        info: okapi::openapi3::Info {
            title: "My API".to_string(),
            version: "1.0.0".to_string(),
            ..Default::default()
        },
        paths: all_paths,
        components: Some(okapi::openapi3::Components {
            schemas: all_schemas,
            ..Default::default()
        }),
        ..Default::default()
    }
}

//// ---- implement for all requests ----- 


use near_jsonrpc_primitives::types::{
    transactions::{
        RpcTransactionResponse, RpcTransactionStatusRequest, RpcSendTransactionRequest
    },
    blocks::{
        RpcBlockRequest, RpcBlockResponse
    },
    chunks::{
        RpcChunkRequest, RpcChunkResponse
    },
    gas_price::{
        RpcGasPriceRequest, RpcGasPriceResponse
    },
    status::{
        RpcHealthResponse, RpcStatusResponse
    },
    validator::{
        RpcValidatorRequest, RpcValidatorResponse, RpcValidatorsOrderedResponse
    },
    light_client::{
        RpcLightClientExecutionProofRequest, RpcLightClientNextBlockRequest, RpcLightClientExecutionProofResponse, RpcLightClientNextBlockResponse, RpcLightClientBlockProofResponse
    },
    network_info::{
        RpcNetworkInfoResponse
    },
    client_config::{
        RpcClientConfigResponse
    },
    changes::{
        RpcStateChangesInBlockByTypeRequest, RpcStateChangesInBlockResponse, RpcStateChangesInBlockByTypeResponse
    },
    congestion::{
        RpcCongestionLevelResponse
    },
    config::{
        RpcProtocolConfigResponse
    },
    receipts::{
        RpcReceiptResponse
    },
    maintenance::{
        RpcMaintenanceWindowsResponse
    },
    split_storage::{
        RpcSplitStorageInfoResponse
    }
    
};

macro_rules! generate_method_name_helper {
    ($enum_name:ident, $trait_impl:ty, $method_name:expr) => {
        #[derive(Serialize, Deserialize, JsonSchema)]
        pub enum $enum_name {
            #[serde(rename = $method_name)]
            VALUE,
        }

        impl MethodNameTrait for $enum_name {
            type S = $enum_name;
            type T = $trait_impl;
        }
    };
}

#[derive(JsonSchema)]
struct RpcHealthRequest;

#[derive(JsonSchema)]
struct RpcStatusRequest;

#[derive(JsonSchema)]
struct RpcNetworkInfoRequest;

#[derive(JsonSchema)]
struct RpcClientConfigRequest;

type RpcSendTransactionRequest2 = RpcSendTransactionRequest;

generate_method_name_helper!(BlockMethodNameHelperEnum, RpcBlockRequest, "block");
generate_method_name_helper!(BroadCastTxAsyncMethodNameHelperEnum, RpcSendTransactionRequest, "broadcast_tx_async");
generate_method_name_helper!(BroadCastTxCommitMethodNameHelperEnum, RpcSendTransactionRequest, "broadcast_tx_commit");
generate_method_name_helper!(ChunkMethodNameHelperEnum, RpcChunkRequest, "chunk");
generate_method_name_helper!(GasPriceMethodNameHelperEnum, RpcGasPriceRequest, "gas_price");
generate_method_name_helper!(HealthMethodNameHelperEnum, RpcHealthRequest, "health");
generate_method_name_helper!(LightClientProofMethodNameHelperEnum, RpcLightClientExecutionProofRequest, "light_client_proof");
generate_method_name_helper!(NextLightClientBlockMethodNameHelperEnum, RpcLightClientNextBlockRequest, "next_light_client_block");
generate_method_name_helper!(NetworkInfoMethodNameHelperEnum, RpcNetworkInfoRequest, "network_info");
generate_method_name_helper!(SendTxMethodNameHelperEnum, RpcSendTransactionRequest, "send_tx");
generate_method_name_helper!(StatusMethodNameHelperEnum, RpcStatusRequest, "status");
generate_method_name_helper!(TxMethodNameHelperEnum, RpcTransactionStatusRequest, "tx");
generate_method_name_helper!(ValidatorsMethodNameHelperEnum, RpcValidatorRequest, "validators");
generate_method_name_helper!(ClientConfigMethodNameHelperEnum, RpcClientConfigRequest, "client_config");

generate_method_name_helper!(ExpChangeMethodNameHelperEnum, RpcClientConfigRequest, "EXPERIMENTAL_changes");
generate_method_name_helper!(ExpChangesBlockMethodNameHelperEnum, RpcClientConfigRequest, "EXPERIMENTAL_changes_in_block");
generate_method_name_helper!(ExpGongestionMethodNameHelperEnum, RpcClientConfigRequest, "EXPERIMENTAL_congestion_level");
generate_method_name_helper!(ExpGenesisMethodNameHelperEnum, RpcClientConfigRequest, "EXPERIMENTAL_genesis_config");
generate_method_name_helper!(ExpLightClientProofMethodNameHelperEnum, RpcClientConfigRequest, "EXPERIMENTAL_light_client_proof");
generate_method_name_helper!(ExpLightClientBlockProofMethodNameHelperEnum, RpcClientConfigRequest, "EXPERIMENTAL_light_client_block_proof");
generate_method_name_helper!(ExpProtocolConfigMethodNameHelperEnum, RpcClientConfigRequest, "EXPERIMENTAL_protocol_config");
generate_method_name_helper!(ExpReceiptMethodNameHelperEnum, RpcClientConfigRequest, "EXPERIMENTAL_receipt");
generate_method_name_helper!(ExpTxStatusMethodNameHelperEnum, RpcClientConfigRequest, "EXPERIMENTAL_tx_status");
generate_method_name_helper!(ExpValidatorsMethodNameHelperEnum, RpcClientConfigRequest, "EXPERIMENTAL_validators_ordered");
generate_method_name_helper!(ExpMaintenanceWindoesMethodNameHelperEnum, RpcClientConfigRequest, "EXPERIMENTAL_maintenance_windows");
generate_method_name_helper!(ExpSplitStorageInfoMethodNameHelperEnum, RpcClientConfigRequest, "EXPERIMENTAL_split_storage_info");


fn main() {

    // let x = X<"block"> {
    // }

    let mut all_schemas = SchemasMap::new();
    let mut all_paths = PathsMap::new();

    add_spec_for_path::<BlockMethodNameHelperEnum, RpcBlockResponse>(&mut all_schemas, &mut all_paths, "block".to_string());
    add_spec_for_path::<BroadCastTxAsyncMethodNameHelperEnum, RpcBlockResponse>(&mut all_schemas, &mut all_paths, "broadcast_tx_async".to_string());
    add_spec_for_path::<BroadCastTxCommitMethodNameHelperEnum, RpcTransactionResponse>(&mut all_schemas, &mut all_paths, "broadcast_tx_commit".to_string());
    add_spec_for_path::<BlockMethodNameHelperEnum, RpcBlockResponse>(&mut all_schemas, &mut all_paths, "chunk".to_string());
    add_spec_for_path::<GasPriceMethodNameHelperEnum, RpcGasPriceResponse>(&mut all_schemas, &mut all_paths, "gas_price".to_string());
    add_spec_for_path::<TxMethodNameHelperEnum, RpcTransactionResponse>(&mut all_schemas, &mut all_paths, "tx".to_string());

    add_spec_for_path::<ChunkMethodNameHelperEnum, RpcChunkResponse>(&mut all_schemas, &mut all_paths, "chunk".to_string());
    add_spec_for_path::<GasPriceMethodNameHelperEnum, RpcGasPriceResponse>(&mut all_schemas, &mut all_paths, "gas_price".to_string());
    add_spec_for_path::<HealthMethodNameHelperEnum, RpcHealthResponse>(&mut all_schemas, &mut all_paths, "health".to_string());
    add_spec_for_path::<LightClientProofMethodNameHelperEnum, RpcLightClientExecutionProofResponse>(&mut all_schemas, &mut all_paths, "light_client_proof".to_string());
    add_spec_for_path::<NextLightClientBlockMethodNameHelperEnum, RpcLightClientNextBlockResponse>(&mut all_schemas, &mut all_paths, "next_light_client_block".to_string());
    add_spec_for_path::<NetworkInfoMethodNameHelperEnum, RpcNetworkInfoResponse>(&mut all_schemas, &mut all_paths, "network_info".to_string());
    add_spec_for_path::<SendTxMethodNameHelperEnum, RpcTransactionResponse>(&mut all_schemas, &mut all_paths, "send_tx".to_string());
    add_spec_for_path::<StatusMethodNameHelperEnum, RpcStatusResponse>(&mut all_schemas, &mut all_paths, "status".to_string());
    add_spec_for_path::<ValidatorsMethodNameHelperEnum, RpcValidatorResponse>(&mut all_schemas, &mut all_paths, "validators".to_string());
    add_spec_for_path::<ClientConfigMethodNameHelperEnum, RpcClientConfigResponse>(&mut all_schemas, &mut all_paths, "client_config".to_string());

    add_spec_for_path::<ExpChangeMethodNameHelperEnum, RpcStateChangesInBlockResponse>(&mut all_schemas, &mut all_paths, "EXPERIMENTAL_changes".to_string());
    add_spec_for_path::<ExpChangesBlockMethodNameHelperEnum, RpcStateChangesInBlockByTypeResponse>(&mut all_schemas, &mut all_paths, "EXPERIMENTAL_changes_in_block".to_string());
    add_spec_for_path::<ExpGongestionMethodNameHelperEnum, RpcCongestionLevelResponse>(&mut all_schemas, &mut all_paths, "EXPERIMENTAL_congestion_level".to_string());
    // add_spec_for_path::<ExpGenesisMethodNameHelperEnum, RpcExperimentalGenesisConfigResponse>(&mut all_schemas, &mut all_paths, "EXPERIMENTAL_genesis_config".to_string());
    add_spec_for_path::<ExpLightClientProofMethodNameHelperEnum, RpcLightClientExecutionProofResponse>(&mut all_schemas, &mut all_paths, "EXPERIMENTAL_light_client_proof".to_string());
    add_spec_for_path::<ExpLightClientBlockProofMethodNameHelperEnum, RpcLightClientBlockProofResponse>(&mut all_schemas, &mut all_paths, "EXPERIMENTAL_light_client_block_proof".to_string());
    add_spec_for_path::<ExpProtocolConfigMethodNameHelperEnum, RpcProtocolConfigResponse>(&mut all_schemas, &mut all_paths, "EXPERIMENTAL_protocol_config".to_string());
    add_spec_for_path::<ExpReceiptMethodNameHelperEnum, RpcReceiptResponse>(&mut all_schemas, &mut all_paths, "EXPERIMENTAL_receipt".to_string());
    add_spec_for_path::<ExpTxStatusMethodNameHelperEnum, RpcTransactionResponse>(&mut all_schemas, &mut all_paths, "EXPERIMENTAL_tx_status".to_string());
    add_spec_for_path::<ExpValidatorsMethodNameHelperEnum, RpcValidatorsOrderedResponse>(&mut all_schemas, &mut all_paths, "EXPERIMENTAL_validators_ordered".to_string());
    add_spec_for_path::<ExpMaintenanceWindoesMethodNameHelperEnum, RpcMaintenanceWindowsResponse>(&mut all_schemas, &mut all_paths, "EXPERIMENTAL_maintenance_windows".to_string());
    add_spec_for_path::<ExpSplitStorageInfoMethodNameHelperEnum, RpcSplitStorageInfoResponse>(&mut all_schemas, &mut all_paths, "EXPERIMENTAL_split_storage_info".to_string());


    let path_schema = whole_spec(all_schemas, all_paths);
    
    let spec_json = serde_json::to_string_pretty(&path_schema).unwrap();
    println!("{}", spec_json);
}