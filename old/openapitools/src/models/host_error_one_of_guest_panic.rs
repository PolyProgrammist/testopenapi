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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct HostErrorOneOfGuestPanic {
    #[serde(rename = "panic_msg")]
    pub panic_msg: String,
}

impl HostErrorOneOfGuestPanic {
    pub fn new(panic_msg: String) -> HostErrorOneOfGuestPanic {
        HostErrorOneOfGuestPanic {
            panic_msg,
        }
    }
}

