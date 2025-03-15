use schemars::JsonSchema;
use serde::{self, Deserialize, Serialize};
use near_time;

#[derive(Serialize, Deserialize, JsonSchema)]
struct MyResponse {
    message: String,
    code: u16,
}

use near_jsonrpc_primitives::types::transactions::{
    RpcSendTransactionRequest, RpcTransactionResponse, RpcTransactionError
};
use near_jsonrpc_primitives::errors::{RpcRequestValidationErrorKind};
use okapi::openapi3::{OpenApi, SchemaObject};
use schemars::gen::SchemaGenerator;
use schemars::schema::RootSchema;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, schemars::JsonSchema)]
#[serde(tag = "name", content = "cause", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RpcErrorKind {
    RequestValidationError(RpcRequestValidationErrorKind),
    HandlerError(serde_json::Value),
    InternalError(serde_json::Value),
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, schemars::JsonSchema)]
#[serde(untagged)]
pub enum CauseRpcErrorKind {
    A(RpcRequestValidationErrorKind),
    B(serde_json::Value),
    C(serde_json::Value),
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
    // #[serde(flatten)]
    // pub error_struct: Option<RpcErrorKind>,
    /// Deprecated please use the `error_struct` instead
    pub code: i64,
    /// Deprecated please use the `error_struct` instead
    pub message: String,
    /// Deprecated please use the `error_struct` instead
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    pub name: Option<NameRpcErrorKind>,
    pub cause: Option<CauseRpcErrorKind>,
}

#[derive(JsonSchema)]
#[serde(untagged)]
pub enum ResponseEither<T, E> {
    Success { result: T },
    Error { tmp: E },
    RpcError { error: RpcError }
}


#[derive(JsonSchema)]
struct JsonRpcResponse<T, E> {
    jsonrpc: String,
    id: String,
    #[serde(flatten)]
    reserr: ResponseEither<T, E>,
}


#[derive(Serialize, Deserialize, JsonSchema)]
pub enum Tx_enum {
    #[serde(rename = "tx")]
    VALUE
}

#[derive(JsonSchema)]
struct RpcParams {
    request_data: near_jsonrpc_primitives::types::transactions::RpcTransactionStatusRequest,
    fetch_receipt: bool,
}

trait HasS {
    type S: JsonSchema;
}

impl HasS for RpcParams {
    type S = Tx_enum;
}


#[derive(JsonSchema)]
struct JsonRpcRequest<T: HasS> {
    jsonrpc: String,
    id: String,
    params: T,
    method: T::S
}


fn generate_schema<T: JsonSchema>() -> OpenApi {
    let settings = schemars::gen::SchemaSettings::openapi3();
    let mut generator = schemars::gen::SchemaGenerator::new(settings);

    let root_schema = generator.into_root_schema_for::<T>();

    let mut theMap: okapi::Map<String, okapi::openapi3::SchemaObject> = root_schema.definitions.into_iter().map(|(k, v)| (k, v.into())).collect();
    theMap.insert(T::schema_name(), root_schema.schema);
    // println!("{:?}", T::schema_name());

    OpenApi {
        openapi: "3.0.0".to_string(),
        info: okapi::openapi3::Info {
            title: "My API".to_string(),
            version: "1.0.0".to_string(),
            ..Default::default()
        },
        components: Some(okapi::openapi3::Components {
            schemas: theMap,
            ..Default::default()
        }),
        ..Default::default()
    }
}

fn main() {
    let response_schema = generate_schema::<JsonRpcResponse<RpcTransactionResponse, RpcError>>();
    let request_schema = generate_schema::<JsonRpcRequest<RpcParams>>();
    
    let spec_json = serde_json::to_string_pretty(&response_schema).unwrap();
    println!("{}", spec_json);

    let spec_yaml = serde_yaml::to_string(&response_schema).unwrap();
    // println!("{}", spec_yaml);
}