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
pub struct MerklePathItem {
    #[serde(rename = "hash")]
    pub hash: String,
    #[serde(rename = "direction")]
    pub direction: models::Direction,
}

impl MerklePathItem {
    pub fn new(hash: String, direction: models::Direction) -> MerklePathItem {
        MerklePathItem {
            hash,
            direction,
        }
    }
}

