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
}

#[derive(JsonSchema)]
#[allow(dead_code)] // Suppress fields never read
struct JsonRpcRequest<T: MethodNameTrait> {
    jsonrpc: String,
    id: String,
    params: T,
    method: T::S
}

type SchemasMap = okapi::Map::<String, okapi::openapi3::SchemaObject>;
type PathsMap = okapi::Map::<String, okapi::openapi3::PathItem>;

fn schemas_map<T: JsonSchema>() -> SchemasMap {
    let settings = schemars::gen::SchemaSettings::openapi3();
    let generator = schemars::gen::SchemaGenerator::new(settings);

    let root_schema = generator.into_root_schema_for::<T>();

    let mut result: SchemasMap = root_schema.definitions.into_iter().map(|(k, v)| (k, v.into())).collect();
    result.insert(T::schema_name(), root_schema.schema);
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


use near_jsonrpc_primitives::types::{
    transactions::{
        RpcTransactionResponse, RpcTransactionStatusRequest
    },
    blocks::{
        RpcBlockRequest, RpcBlockResponse
    }
    
};


//// ---- implement for all requests ----- 

#[derive(Serialize, Deserialize, JsonSchema)]
pub enum TxMethodNameHelperEnum {
    #[serde(rename = "tx")]
    VALUE
}

impl MethodNameTrait for RpcTransactionStatusRequest {
    type S = TxMethodNameHelperEnum;
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub enum BlockMethodNameHelperEnum {
    #[serde(rename = "block")]
    VALUE
}

impl MethodNameTrait for RpcBlockRequest {
    type S = BlockMethodNameHelperEnum;
}

fn main() {
    let mut all_schemas = SchemasMap::new();
    let mut all_paths = PathsMap::new();

    add_spec_for_path::<RpcTransactionStatusRequest, RpcTransactionResponse>(&mut all_schemas, &mut all_paths, "tx".to_string());
    add_spec_for_path::<RpcBlockRequest, RpcBlockResponse>(&mut all_schemas, &mut all_paths, "block".to_string());

    let path_schema = whole_spec(all_schemas, all_paths);
    
    let spec_json = serde_json::to_string_pretty(&path_schema).unwrap();
    println!("{}", spec_json);
}