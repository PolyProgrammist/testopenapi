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
struct JsonRpcResponse<T, E> {
    jsonrpc: String,
    id: String,
    #[serde(flatten)]
    reserr: ResponseEither<T, E>,
}

trait MethodNameTrait {
    type S: JsonSchema;
}

#[derive(JsonSchema)]
struct JsonRpcRequest<T: MethodNameTrait> {
    jsonrpc: String,
    id: String,
    params: T,
    method: T::S
}

fn schema_map<T: JsonSchema>() -> okapi::Map<String, okapi::openapi3::SchemaObject> {
    let settings = schemars::gen::SchemaSettings::openapi3();
    let mut generator = schemars::gen::SchemaGenerator::new(settings);

    let root_schema = generator.into_root_schema_for::<T>();

    let mut theMap: okapi::Map<String, okapi::openapi3::SchemaObject> = root_schema.definitions.into_iter().map(|(k, v)| (k, v.into())).collect();
    theMap.insert(T::schema_name(), root_schema.schema);
    theMap
}

fn get_paths(request_schema_name: String, response_schema_name: String, method_name: String) -> okapi::Map::<String, okapi::openapi3::PathItem> {
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

    let mut paths = okapi::Map::<String, okapi::openapi3::PathItem>::new();
    paths.insert(
        format!("/{}", method_name),
        okapi::openapi3::PathItem {
            post: Some(operation),
            ..Default::default()
        },
    );

    paths
}

fn path_spec_internal<RequestType: JsonSchema, ResponseType: JsonSchema>(method_name: String) -> OpenApi {
    let mut requestMap = schema_map::<RequestType>();
    let responseMap = schema_map::<ResponseType>();

    let mut allMap = requestMap;
    allMap.extend(responseMap);

    let paths = get_paths(
        format!("#/components/schemas/{}", RequestType::schema_name()), 
        format!("#/components/schemas/{}", ResponseType::schema_name()),
        method_name
    );

    OpenApi {
        openapi: "3.0.0".to_string(),
        info: okapi::openapi3::Info {
            title: "My API".to_string(),
            version: "1.0.0".to_string(),
            ..Default::default()
        },
        paths: paths,
        components: Some(okapi::openapi3::Components {
            schemas: allMap,
            ..Default::default()
        }),
        ..Default::default()
    }
}

fn path_spec<Request: JsonSchema + MethodNameTrait, Response: JsonSchema>(method_name: String) -> OpenApi {
    path_spec_internal::<JsonRpcRequest<Request>, JsonRpcResponse<Response, RpcError>>(method_name)
}


use near_jsonrpc_primitives::types::{
    transactions::{
        RpcTransactionResponse, RpcTransactionStatusRequest
    },
    blocks::{
        RpcBlockRequest, RpcBlockResponse
    }
    
};

#[derive(Serialize, Deserialize, JsonSchema)]
pub enum Tx_enum {
    #[serde(rename = "tx")]
    VALUE
}

impl MethodNameTrait for RpcTransactionStatusRequest {
    type S = Tx_enum;
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub enum Block_enum {
    #[serde(rename = "block")]
    VALUE
}

impl MethodNameTrait for RpcBlockRequest {
    type S = Block_enum;
}



fn main() {
    let path_schema = path_spec::<RpcTransactionStatusRequest, RpcTransactionResponse>("tx".to_string());
    let path_schema = path_spec::<RpcBlockRequest, RpcBlockResponse>("block".to_string());
    
    let spec_json = serde_json::to_string_pretty(&path_schema).unwrap();
    println!("{}", spec_json);
}