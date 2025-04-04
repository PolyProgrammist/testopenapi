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
pub struct HostErrorOneOf2InvalidPromiseResultIndex {
    #[serde(rename = "result_idx")]
    pub result_idx: i32,
}

impl HostErrorOneOf2InvalidPromiseResultIndex {
    pub fn new(result_idx: i32) -> HostErrorOneOf2InvalidPromiseResultIndex {
        HostErrorOneOf2InvalidPromiseResultIndex {
            result_idx,
        }
    }
}

