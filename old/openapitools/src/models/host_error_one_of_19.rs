/*
 * okapirocket
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// HostErrorOneOf19 : Invalid input to ed25519 signature verification function (e.g. signature cannot be derived from bytes).
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct HostErrorOneOf19 {
    #[serde(rename = "Ed25519VerifyInvalidInput")]
    pub ed25519_verify_invalid_input: Box<models::FunctionCallErrorOneOf1LinkError>,
}

impl HostErrorOneOf19 {
    /// Invalid input to ed25519 signature verification function (e.g. signature cannot be derived from bytes).
    pub fn new(ed25519_verify_invalid_input: models::FunctionCallErrorOneOf1LinkError) -> HostErrorOneOf19 {
        HostErrorOneOf19 {
            ed25519_verify_invalid_input: Box::new(ed25519_verify_invalid_input),
        }
    }
}

