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
pub struct HostErrorOneOf4IteratorWasInvalidated {
    #[serde(rename = "iterator_index")]
    pub iterator_index: i32,
}

impl HostErrorOneOf4IteratorWasInvalidated {
    pub fn new(iterator_index: i32) -> HostErrorOneOf4IteratorWasInvalidated {
        HostErrorOneOf4IteratorWasInvalidated {
            iterator_index,
        }
    }
}

