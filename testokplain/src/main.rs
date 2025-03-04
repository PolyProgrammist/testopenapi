use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use near_time;

#[derive(Serialize, Deserialize, JsonSchema)]
struct MyResponse {
    message: String,
    code: u16,
}

use near_jsonrpc_primitives::types::transactions::{
    RpcSendTransactionRequest, RpcTransactionResponse
};

use okapi::openapi3::{OpenApi, SchemaObject};
use schemars::gen::SchemaGenerator;
use schemars::schema::RootSchema;

fn main() {
    // Generate schema
    let settings = schemars::gen::SchemaSettings::openapi3();
    let generator = schemars::gen::SchemaGenerator::new(settings);
    let root_schema = generator.into_root_schema_for::<RpcTransactionResponse>();

    // Create OpenAPI spec
    let openapi = OpenApi {
        openapi: "3.0.0".to_string(),
        info: okapi::openapi3::Info {
            title: "My API".to_string(),
            version: "1.0.0".to_string(),
            ..Default::default()
        },
        components: Some(okapi::openapi3::Components {
            schemas: root_schema.definitions.into_iter().map(|(k, v)| (k, v.into())).collect(),
            ..Default::default()
        }),
        ..Default::default()
    };

    let spec_yaml = serde_yaml::to_string(&openapi).unwrap();
    println!("{}", spec_yaml);
}