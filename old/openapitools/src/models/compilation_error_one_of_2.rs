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

/// CompilationErrorOneOf2 : This is for defense in depth. We expect our runtime-independent preparation code to fully catch all invalid wasms, but, if it ever misses something we’ll emit this error
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CompilationErrorOneOf2 {
    #[serde(rename = "WasmerCompileError")]
    pub wasmer_compile_error: Box<models::FunctionCallErrorOneOf1LinkError>,
}

impl CompilationErrorOneOf2 {
    /// This is for defense in depth. We expect our runtime-independent preparation code to fully catch all invalid wasms, but, if it ever misses something we’ll emit this error
    pub fn new(wasmer_compile_error: models::FunctionCallErrorOneOf1LinkError) -> CompilationErrorOneOf2 {
        CompilationErrorOneOf2 {
            wasmer_compile_error: Box::new(wasmer_compile_error),
        }
    }
}

