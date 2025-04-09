#[allow(unused_imports)]
use progenitor_client::{encode_path, RequestBuilderExt};
#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, Error, ResponseValue};
#[allow(unused_imports)]
use reqwest::header::{HeaderMap, HeaderValue};
/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
    /// Error types.
    pub mod error {
        /// Error from a TryFrom or FromStr implementation.
        pub struct ConversionError(::std::borrow::Cow<'static, str>);
        impl ::std::error::Error for ConversionError {}
        impl ::std::fmt::Display for ConversionError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Display::fmt(&self.0, f)
            }
        }

        impl ::std::fmt::Debug for ConversionError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Debug::fmt(&self.0, f)
            }
        }

        impl From<&'static str> for ConversionError {
            fn from(value: &'static str) -> Self {
                Self(value.into())
            }
        }

        impl From<String> for ConversionError {
            fn from(value: String) -> Self {
                Self(value.into())
            }
        }
    }

    ///NEAR Account Identifier.
    ///
    /// This is a unique, syntactically valid, human-readable account identifier
    /// on the NEAR network.
    ///
    /// [See the crate-level docs for information about
    /// validation.](index.html#account-id-rules)
    ///
    /// Also see [Error kind precedence](AccountId#error-kind-precedence).
    ///
    /// ## Examples
    ///
    /// ```
    /// use near_account_id::AccountId;
    ///
    /// let alice: AccountId = "alice.near".parse().unwrap();
    ///
    /// assert!("ƒelicia.near".parse::<AccountId>().is_err()); // (ƒ is not f)
    /// ```
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "NEAR Account Identifier.\n\n This is a unique,
    /// syntactically valid, human-readable account identifier on the NEAR
    /// network.\n\n [See the crate-level docs for information about
    /// validation.](index.html#account-id-rules)\n\n Also see [Error kind
    /// precedence](AccountId#error-kind-precedence).\n\n ## Examples\n\n ```\n
    /// use near_account_id::AccountId;\n\n let alice: AccountId =
    /// \"alice.near\".parse().unwrap();\n\n
    /// assert!(\"ƒelicia.near\".parse::<AccountId>().is_err()); // (ƒ is not
    /// f)\n ```",
    ///  "type": "string"
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    #[serde(transparent)]
    pub struct AccountId(pub ::std::string::String);
    impl ::std::ops::Deref for AccountId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<AccountId> for ::std::string::String {
        fn from(value: AccountId) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&AccountId> for AccountId {
        fn from(value: &AccountId) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<::std::string::String> for AccountId {
        fn from(value: ::std::string::String) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for AccountId {
        type Err = ::std::convert::Infallible;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::fmt::Display for AccountId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    ///CauseRpcErrorKind
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "anyOf": [
    ///    {
    ///      "$ref": "#/components/schemas/RpcRequestValidationErrorKind"
    ///    },
    ///    {},
    ///    {}
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CauseRpcErrorKind {
        #[serde(
            flatten,
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subtype_0: ::std::option::Option<RpcRequestValidationErrorKind>,
        #[serde(
            flatten,
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subtype_1: ::std::option::Option<::serde_json::Value>,
        #[serde(
            flatten,
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subtype_2: ::std::option::Option<::serde_json::Value>,
    }

    impl ::std::convert::From<&CauseRpcErrorKind> for CauseRpcErrorKind {
        fn from(value: &CauseRpcErrorKind) -> Self {
            value.clone()
        }
    }

    impl ::std::default::Default for CauseRpcErrorKind {
        fn default() -> Self {
            Self {
                subtype_0: Default::default(),
                subtype_1: Default::default(),
                subtype_2: Default::default(),
            }
        }
    }

    ///JsonRpcRequestForRpcMaintenanceWindowsRequest
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "JsonRpcRequest_for_RpcMaintenanceWindowsRequest",
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "jsonrpc",
    ///    "method",
    ///    "params"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "jsonrpc": {
    ///      "type": "string"
    ///    },
    ///    "method": {
    ///      "type": "string",
    ///      "enum": [
    ///        "EXPERIMENTAL_maintenance_windows"
    ///      ]
    ///    },
    ///    "params": {
    ///      "$ref": "#/components/schemas/RpcMaintenanceWindowsRequest"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct JsonRpcRequestForRpcMaintenanceWindowsRequest {
        pub id: ::std::string::String,
        pub jsonrpc: ::std::string::String,
        pub method: JsonRpcRequestForRpcMaintenanceWindowsRequestMethod,
        pub params: RpcMaintenanceWindowsRequest,
    }

    impl ::std::convert::From<&JsonRpcRequestForRpcMaintenanceWindowsRequest>
        for JsonRpcRequestForRpcMaintenanceWindowsRequest
    {
        fn from(value: &JsonRpcRequestForRpcMaintenanceWindowsRequest) -> Self {
            value.clone()
        }
    }

    ///JsonRpcRequestForRpcMaintenanceWindowsRequestMethod
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "EXPERIMENTAL_maintenance_windows"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum JsonRpcRequestForRpcMaintenanceWindowsRequestMethod {
        #[serde(rename = "EXPERIMENTAL_maintenance_windows")]
        ExperimentalMaintenanceWindows,
    }

    impl ::std::convert::From<&Self> for JsonRpcRequestForRpcMaintenanceWindowsRequestMethod {
        fn from(value: &JsonRpcRequestForRpcMaintenanceWindowsRequestMethod) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for JsonRpcRequestForRpcMaintenanceWindowsRequestMethod {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::ExperimentalMaintenanceWindows => {
                    write!(f, "EXPERIMENTAL_maintenance_windows")
                }
            }
        }
    }

    impl ::std::str::FromStr for JsonRpcRequestForRpcMaintenanceWindowsRequestMethod {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "EXPERIMENTAL_maintenance_windows" => Ok(Self::ExperimentalMaintenanceWindows),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for JsonRpcRequestForRpcMaintenanceWindowsRequestMethod {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for JsonRpcRequestForRpcMaintenanceWindowsRequestMethod
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for JsonRpcRequestForRpcMaintenanceWindowsRequestMethod
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///JsonRpcRequestForRpcSplitStorageInfoRequest
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "JsonRpcRequest_for_RpcSplitStorageInfoRequest",
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "jsonrpc",
    ///    "method",
    ///    "params"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "jsonrpc": {
    ///      "type": "string"
    ///    },
    ///    "method": {
    ///      "type": "string",
    ///      "enum": [
    ///        "EXPERIMENTAL_split_storage_info"
    ///      ]
    ///    },
    ///    "params": {
    ///      "$ref": "#/components/schemas/RpcSplitStorageInfoRequest"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct JsonRpcRequestForRpcSplitStorageInfoRequest {
        pub id: ::std::string::String,
        pub jsonrpc: ::std::string::String,
        pub method: JsonRpcRequestForRpcSplitStorageInfoRequestMethod,
        pub params: RpcSplitStorageInfoRequest,
    }

    impl ::std::convert::From<&JsonRpcRequestForRpcSplitStorageInfoRequest>
        for JsonRpcRequestForRpcSplitStorageInfoRequest
    {
        fn from(value: &JsonRpcRequestForRpcSplitStorageInfoRequest) -> Self {
            value.clone()
        }
    }

    ///JsonRpcRequestForRpcSplitStorageInfoRequestMethod
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "EXPERIMENTAL_split_storage_info"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum JsonRpcRequestForRpcSplitStorageInfoRequestMethod {
        #[serde(rename = "EXPERIMENTAL_split_storage_info")]
        ExperimentalSplitStorageInfo,
    }

    impl ::std::convert::From<&Self> for JsonRpcRequestForRpcSplitStorageInfoRequestMethod {
        fn from(value: &JsonRpcRequestForRpcSplitStorageInfoRequestMethod) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for JsonRpcRequestForRpcSplitStorageInfoRequestMethod {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::ExperimentalSplitStorageInfo => write!(f, "EXPERIMENTAL_split_storage_info"),
            }
        }
    }

    impl ::std::str::FromStr for JsonRpcRequestForRpcSplitStorageInfoRequestMethod {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "EXPERIMENTAL_split_storage_info" => Ok(Self::ExperimentalSplitStorageInfo),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for JsonRpcRequestForRpcSplitStorageInfoRequestMethod {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for JsonRpcRequestForRpcSplitStorageInfoRequestMethod
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for JsonRpcRequestForRpcSplitStorageInfoRequestMethod
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///JsonRpcResponseForArrayOfMaintenanceWindowAndRpcError
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "JsonRpcResponse_for_Array_of_MaintenanceWindow_and_RpcError",
    ///  "type": "object",
    ///  "anyOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "result"
    ///      ],
    ///      "properties": {
    ///        "result": {
    ///          "type": "array",
    ///          "items": {
    ///            "$ref": "#/components/schemas/MaintenanceWindow"
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "tmp"
    ///      ],
    ///      "properties": {
    ///        "tmp": {
    ///          "$ref": "#/components/schemas/RpcError"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "error"
    ///      ],
    ///      "properties": {
    ///        "error": {
    ///          "$ref": "#/components/schemas/RpcError"
    ///        }
    ///      }
    ///    }
    ///  ],
    ///  "required": [
    ///    "id",
    ///    "jsonrpc"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "jsonrpc": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum JsonRpcResponseForArrayOfMaintenanceWindowAndRpcError {
        Variant0 {
            id: ::std::string::String,
            jsonrpc: ::std::string::String,
            result: ::std::vec::Vec<MaintenanceWindow>,
        },
        Variant1 {
            id: ::std::string::String,
            jsonrpc: ::std::string::String,
            tmp: RpcError,
        },
        Variant2 {
            error: RpcError,
            id: ::std::string::String,
            jsonrpc: ::std::string::String,
        },
    }

    impl ::std::convert::From<&Self> for JsonRpcResponseForArrayOfMaintenanceWindowAndRpcError {
        fn from(value: &JsonRpcResponseForArrayOfMaintenanceWindowAndRpcError) -> Self {
            value.clone()
        }
    }

    ///JsonRpcResponseForRpcSplitStorageInfoResponseAndRpcError
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "JsonRpcResponse_for_RpcSplitStorageInfoResponse_and_RpcError"
    /// ,
    ///  "type": "object",
    ///  "anyOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "result"
    ///      ],
    ///      "properties": {
    ///        "result": {
    ///          "$ref": "#/components/schemas/RpcSplitStorageInfoResponse"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "tmp"
    ///      ],
    ///      "properties": {
    ///        "tmp": {
    ///          "$ref": "#/components/schemas/RpcError"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "error"
    ///      ],
    ///      "properties": {
    ///        "error": {
    ///          "$ref": "#/components/schemas/RpcError"
    ///        }
    ///      }
    ///    }
    ///  ],
    ///  "required": [
    ///    "id",
    ///    "jsonrpc"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "jsonrpc": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum JsonRpcResponseForRpcSplitStorageInfoResponseAndRpcError {
        Variant0 {
            id: ::std::string::String,
            jsonrpc: ::std::string::String,
            result: RpcSplitStorageInfoResponse,
        },
        Variant1 {
            id: ::std::string::String,
            jsonrpc: ::std::string::String,
            tmp: RpcError,
        },
        Variant2 {
            error: RpcError,
            id: ::std::string::String,
            jsonrpc: ::std::string::String,
        },
    }

    impl ::std::convert::From<&Self> for JsonRpcResponseForRpcSplitStorageInfoResponseAndRpcError {
        fn from(value: &JsonRpcResponseForRpcSplitStorageInfoResponseAndRpcError) -> Self {
            value.clone()
        }
    }

    ///MaintenanceWindow
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "finish",
    ///    "start"
    ///  ],
    ///  "properties": {
    ///    "finish": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "start": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct MaintenanceWindow {
        pub finish: u64,
        pub start: u64,
    }

    impl ::std::convert::From<&MaintenanceWindow> for MaintenanceWindow {
        fn from(value: &MaintenanceWindow) -> Self {
            value.clone()
        }
    }

    ///NameRpcErrorKind
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "REQUEST_VALIDATION_ERROR",
    ///    "HANDLER_ERROR",
    ///    "INTERNAL_ERROR"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum NameRpcErrorKind {
        #[serde(rename = "REQUEST_VALIDATION_ERROR")]
        RequestValidationError,
        #[serde(rename = "HANDLER_ERROR")]
        HandlerError,
        #[serde(rename = "INTERNAL_ERROR")]
        InternalError,
    }

    impl ::std::convert::From<&Self> for NameRpcErrorKind {
        fn from(value: &NameRpcErrorKind) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for NameRpcErrorKind {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::RequestValidationError => write!(f, "REQUEST_VALIDATION_ERROR"),
                Self::HandlerError => write!(f, "HANDLER_ERROR"),
                Self::InternalError => write!(f, "INTERNAL_ERROR"),
            }
        }
    }

    impl ::std::str::FromStr for NameRpcErrorKind {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "REQUEST_VALIDATION_ERROR" => Ok(Self::RequestValidationError),
                "HANDLER_ERROR" => Ok(Self::HandlerError),
                "INTERNAL_ERROR" => Ok(Self::InternalError),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for NameRpcErrorKind {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for NameRpcErrorKind {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for NameRpcErrorKind {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///RpcError
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "code",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "cause": {
    ///      "$ref": "#/components/schemas/CauseRpcErrorKind"
    ///    },
    ///    "code": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "data": {},
    ///    "message": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "$ref": "#/components/schemas/NameRpcErrorKind"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RpcError {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cause: ::std::option::Option<CauseRpcErrorKind>,
        pub code: i64,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub data: ::std::option::Option<::serde_json::Value>,
        pub message: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<NameRpcErrorKind>,
    }

    impl ::std::convert::From<&RpcError> for RpcError {
        fn from(value: &RpcError) -> Self {
            value.clone()
        }
    }

    ///RpcMaintenanceWindowsRequest
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "RpcMaintenanceWindowsRequest",
    ///  "type": "object",
    ///  "required": [
    ///    "account_id"
    ///  ],
    ///  "properties": {
    ///    "account_id": {
    ///      "$ref": "#/components/schemas/AccountId"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RpcMaintenanceWindowsRequest {
        pub account_id: AccountId,
    }

    impl ::std::convert::From<&RpcMaintenanceWindowsRequest> for RpcMaintenanceWindowsRequest {
        fn from(value: &RpcMaintenanceWindowsRequest) -> Self {
            value.clone()
        }
    }

    ///RpcRequestValidationErrorKind
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "info",
    ///        "name"
    ///      ],
    ///      "properties": {
    ///        "info": {
    ///          "type": "object",
    ///          "required": [
    ///            "method_name"
    ///          ],
    ///          "properties": {
    ///            "method_name": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        },
    ///        "name": {
    ///          "type": "string",
    ///          "enum": [
    ///            "METHOD_NOT_FOUND"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "info",
    ///        "name"
    ///      ],
    ///      "properties": {
    ///        "info": {
    ///          "type": "object",
    ///          "required": [
    ///            "error_message"
    ///          ],
    ///          "properties": {
    ///            "error_message": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        },
    ///        "name": {
    ///          "type": "string",
    ///          "enum": [
    ///            "PARSE_ERROR"
    ///          ]
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(tag = "name", content = "info")]
    pub enum RpcRequestValidationErrorKind {
        #[serde(rename = "METHOD_NOT_FOUND")]
        MethodNotFound { method_name: ::std::string::String },
        #[serde(rename = "PARSE_ERROR")]
        ParseError {
            error_message: ::std::string::String,
        },
    }

    impl ::std::convert::From<&Self> for RpcRequestValidationErrorKind {
        fn from(value: &RpcRequestValidationErrorKind) -> Self {
            value.clone()
        }
    }

    ///RpcSplitStorageInfoRequest
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "RpcSplitStorageInfoRequest",
    ///  "type": "object"
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct RpcSplitStorageInfoRequest(
        pub ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    );
    impl ::std::ops::Deref for RpcSplitStorageInfoRequest {
        type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
        fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
            &self.0
        }
    }

    impl ::std::convert::From<RpcSplitStorageInfoRequest>
        for ::serde_json::Map<::std::string::String, ::serde_json::Value>
    {
        fn from(value: RpcSplitStorageInfoRequest) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&RpcSplitStorageInfoRequest> for RpcSplitStorageInfoRequest {
        fn from(value: &RpcSplitStorageInfoRequest) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
        for RpcSplitStorageInfoRequest
    {
        fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
            Self(value)
        }
    }

    ///Contains the split storage information.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Contains the split storage information.",
    ///  "type": "object",
    ///  "properties": {
    ///    "cold_head_height": {
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "final_head_height": {
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "head_height": {
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "hot_db_kind": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RpcSplitStorageInfoResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cold_head_height: ::std::option::Option<u64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub final_head_height: ::std::option::Option<u64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub head_height: ::std::option::Option<u64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub hot_db_kind: ::std::option::Option<::std::string::String>,
    }

    impl ::std::convert::From<&RpcSplitStorageInfoResponse> for RpcSplitStorageInfoResponse {
        fn from(value: &RpcSplitStorageInfoResponse) -> Self {
            value.clone()
        }
    }

    impl ::std::default::Default for RpcSplitStorageInfoResponse {
        fn default() -> Self {
            Self {
                cold_head_height: Default::default(),
                final_head_height: Default::default(),
                head_height: Default::default(),
                hot_db_kind: Default::default(),
            }
        }
    }
}

#[derive(Clone, Debug)]
///Client for My API
///
///Version: 1.0.0
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
}

impl Client {
    /// Create a new client.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new(baseurl: &str) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let client = {
            let dur = std::time::Duration::from_secs(15);
            reqwest::ClientBuilder::new()
                .connect_timeout(dur)
                .timeout(dur)
        };
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::ClientBuilder::new();
        Self::new_with_client(baseurl, client.build().unwrap())
    }

    /// Construct a new client with an existing `reqwest::Client`,
    /// allowing more control over its configuration.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
        }
    }

    /// Get the base URL to which requests are made.
    pub fn baseurl(&self) -> &String {
        &self.baseurl
    }

    /// Get the internal `reqwest::Client` used to make requests.
    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }

    /// Get the version of this API.
    ///
    /// This string is pulled directly from the source OpenAPI
    /// document and may be in any format the API selects.
    pub fn api_version(&self) -> &'static str {
        "1.0.0"
    }
}

#[allow(clippy::all)]
#[allow(elided_named_lifetimes)]
impl Client {
    ///Sends a `POST` request to `/EXPERIMENTAL_maintenance_windows`
    pub async fn experimental_maintenance_windows<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForRpcMaintenanceWindowsRequest,
    ) -> Result<
        ResponseValue<types::JsonRpcResponseForArrayOfMaintenanceWindowAndRpcError>,
        Error<()>,
    > {
        let url = format!("{}/", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/EXPERIMENTAL_split_storage_info`
    pub async fn experimental_split_storage_info<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForRpcSplitStorageInfoRequest,
    ) -> Result<
        ResponseValue<types::JsonRpcResponseForRpcSplitStorageInfoResponseAndRpcError>,
        Error<()>,
    > {
        let url = format!("{}/", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

/// Items consumers will typically use such as the Client.
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
}
