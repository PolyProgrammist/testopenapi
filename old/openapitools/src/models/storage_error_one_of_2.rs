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

/// StorageErrorOneOf2 : Flat storage error, meaning that it doesn't support some block anymore. We guarantee that such block cannot become final, thus block processing must resume normally.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageErrorOneOf2 {
    #[serde(rename = "FlatStorageBlockNotSupported")]
    pub flat_storage_block_not_supported: String,
}

impl StorageErrorOneOf2 {
    /// Flat storage error, meaning that it doesn't support some block anymore. We guarantee that such block cannot become final, thus block processing must resume normally.
    pub fn new(flat_storage_block_not_supported: String) -> StorageErrorOneOf2 {
        StorageErrorOneOf2 {
            flat_storage_block_not_supported,
        }
    }
}

