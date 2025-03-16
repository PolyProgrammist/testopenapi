use schemars::JsonSchema;
use serde::{self, Deserialize, Serialize};

use near_jsonrpc_primitives::types::transactions::{
    RpcTransactionResponse, RpcTransactionStatusRequest
};
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


#[derive(Serialize, Deserialize, JsonSchema)]
pub enum Tx_enum {
    #[serde(rename = "tx")]
    VALUE
}

trait MethodNameTrait {
    type S: JsonSchema;
}

impl MethodNameTrait for near_jsonrpc_primitives::types::transactions::RpcTransactionStatusRequest {
    type S = Tx_enum;
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

fn get_paths(request_schema_name: String, response_schema_name: String) -> okapi::Map::<String, okapi::openapi3::PathItem> {
    let request_body = okapi::openapi3::RequestBody {
        description: Some("User registration data".to_string()),
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

    // Define response
    let mut responses = okapi::openapi3::Responses::default();
    responses.responses.insert(
        "200".to_string(),
        okapi::openapi3::Response {
            description: "User created successfully".to_string(),
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

    // Define operation
    let operation = okapi::openapi3::Operation {
        summary: Some("Create a new user".to_string()),
        operation_id: Some("createUser".to_string()),
        request_body: Some(request_body.into()),
        responses,
        ..Default::default()
    };

    // Define paths
    let mut paths = okapi::Map::<String, okapi::openapi3::PathItem>::new();
    paths.insert(
        "/users".to_string(),
        okapi::openapi3::PathItem {
            post: Some(operation),
            ..Default::default()
        },
    );

    paths
}

fn path_spec_internal<RequestType: JsonSchema, ResponseType: JsonSchema>() -> OpenApi {
    let mut requestMap = schema_map::<RequestType>();
    let responseMap = schema_map::<ResponseType>();

    let mut allMap = requestMap;
    allMap.extend(responseMap);

    let paths = get_paths(
        format!("#/components/schemas/{}", RequestType::schema_name()), 
        format!("#/components/schemas/{}", ResponseType::schema_name())
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

fn path_spec<Request: JsonSchema + MethodNameTrait, Response: JsonSchema>() -> OpenApi {
    path_spec_internal::<JsonRpcRequest<Request>, JsonRpcResponse<Response, RpcError>>()
}

fn main() {
    let path_schema = path_spec::<RpcTransactionStatusRequest, RpcTransactionResponse>();
    
    let spec_json = serde_json::to_string_pretty(&path_schema).unwrap();
    println!("{}", spec_json);
}