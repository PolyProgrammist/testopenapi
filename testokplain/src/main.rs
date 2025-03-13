use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use near_time;

#[derive(Serialize, Deserialize, JsonSchema)]
struct MyResponse {
    message: String,
    code: u16,
}

use near_jsonrpc_primitives::types::transactions::{
    RpcSendTransactionRequest, RpcTransactionResponse, RpcTransactionError
};

use okapi::openapi3::{OpenApi, SchemaObject};
use schemars::gen::SchemaGenerator;
use schemars::schema::RootSchema;

#[derive(JsonSchema)]
struct JsonRpcResponse<T> {
    jsonrpc: String,
    result: T,
    id: String,
} 


#[derive(JsonSchema)]
struct RpcParams {
    request_data: near_jsonrpc_primitives::types::transactions::RpcTransactionStatusRequest,
    fetch_receipt: bool,
}


#[derive(JsonSchema)]
struct JsonRpcRequest<T> {
    jsonrpc: String,
    id: String,
    params: T,
    const tmp: String = "hello",
}

fn generate_schema<T: JsonSchema>() -> OpenApi {
    let settings = schemars::gen::SchemaSettings::openapi3();
    let mut generator = schemars::gen::SchemaGenerator::new(settings);

    let root_schema = generator.into_root_schema_for::<T>();

    let mut theMap: okapi::Map<String, okapi::openapi3::SchemaObject> = root_schema.definitions.into_iter().map(|(k, v)| (k, v.into())).collect();
    theMap.insert(T::schema_name(), root_schema.schema);
    println!("{:?}", T::schema_name());

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
    let response_schema = generate_schema::<JsonRpcResponse<RpcTransactionResponse>>();
    let request_schema = generate_schema::<JsonRpcRequest<RpcParams>>();
    
    // let spec_json = serde_json::to_string_pretty(&openapi).unwrap();
    // println!("{}", spec_json);

    let spec_yaml = serde_yaml::to_string(&response_schema).unwrap();
    // println!("{}", spec_yaml);
}