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

    ///Access key provides limited access to an account. Each access key
    /// belongs to some account and is identified by a unique (within the
    /// account) public key. One account may have large number of access keys.
    /// Access keys allow to act on behalf of the account by restricting
    /// transactions that can be issued. `account_id,public_key` is a key in the
    /// state
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Access key provides limited access to an account. Each
    /// access key belongs to some account and is identified by a unique (within
    /// the account) public key. One account may have large number of access
    /// keys. Access keys allow to act on behalf of the account by restricting
    /// transactions that can be issued. `account_id,public_key` is a key in the
    /// state",
    ///  "type": "object",
    ///  "required": [
    ///    "nonce",
    ///    "permission"
    ///  ],
    ///  "properties": {
    ///    "nonce": {
    ///      "description": "Nonce for this access key, used for tx nonce generation. When access key is created, nonce is set to `(block_height - 1) * 1e6` to avoid tx hash collision on access key re-creation. See <https://github.com/near/nearcore/issues/3779> for more details.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "permission": {
    ///      "description": "Defines permissions for this access key.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/AccessKeyPermission"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AccessKey {
        ///Nonce for this access key, used for tx nonce generation. When access key is created, nonce is set to `(block_height - 1) * 1e6` to avoid tx hash collision on access key re-creation. See <https://github.com/near/nearcore/issues/3779> for more details.
        pub nonce: u64,
        ///Defines permissions for this access key.
        pub permission: AccessKeyPermission,
    }

    impl ::std::convert::From<&AccessKey> for AccessKey {
        fn from(value: &AccessKey) -> Self {
            value.clone()
        }
    }

    ///Describes the cost of creating an access key.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Describes the cost of creating an access key.",
    ///  "type": "object",
    ///  "required": [
    ///    "full_access_cost",
    ///    "function_call_cost",
    ///    "function_call_cost_per_byte"
    ///  ],
    ///  "properties": {
    ///    "full_access_cost": {
    ///      "description": "Base cost of creating a full access access-key.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Fee"
    ///        }
    ///      ]
    ///    },
    ///    "function_call_cost": {
    ///      "description": "Base cost of creating an access-key restricted to
    /// specific functions.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Fee"
    ///        }
    ///      ]
    ///    },
    ///    "function_call_cost_per_byte": {
    ///      "description": "Cost per byte of method_names of creating a
    /// restricted access-key.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Fee"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AccessKeyCreationConfigView {
        ///Base cost of creating a full access access-key.
        pub full_access_cost: Fee,
        ///Base cost of creating an access-key restricted to specific
        /// functions.
        pub function_call_cost: Fee,
        ///Cost per byte of method_names of creating a restricted access-key.
        pub function_call_cost_per_byte: Fee,
    }

    impl ::std::convert::From<&AccessKeyCreationConfigView> for AccessKeyCreationConfigView {
        fn from(value: &AccessKeyCreationConfigView) -> Self {
            value.clone()
        }
    }

    ///Defines permissions for AccessKey
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Defines permissions for AccessKey",
    ///  "oneOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "FunctionCall"
    ///      ],
    ///      "properties": {
    ///        "FunctionCall": {
    ///          "$ref": "#/components/schemas/FunctionCallPermission"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "Grants full access to the account. NOTE: It's used
    /// to replace account-level public keys.",
    ///      "type": "string",
    ///      "enum": [
    ///        "FullAccess"
    ///      ]
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub enum AccessKeyPermission {
        FunctionCall(FunctionCallPermission),
        ///Grants full access to the account. NOTE: It's used to replace
        /// account-level public keys.
        FullAccess,
    }

    impl ::std::convert::From<&Self> for AccessKeyPermission {
        fn from(value: &AccessKeyPermission) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<FunctionCallPermission> for AccessKeyPermission {
        fn from(value: FunctionCallPermission) -> Self {
            Self::FunctionCall(value)
        }
    }

    ///AccessKeyPermissionView
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "string",
    ///      "enum": [
    ///        "FullAccess"
    ///      ]
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "FunctionCall"
    ///      ],
    ///      "properties": {
    ///        "FunctionCall": {
    ///          "type": "object",
    ///          "required": [
    ///            "allowance",
    ///            "method_names",
    ///            "receiver_id"
    ///          ],
    ///          "properties": {
    ///            "allowance": {
    ///              "type": "string"
    ///            },
    ///            "method_names": {
    ///              "type": "array",
    ///              "items": {
    ///                "type": "string"
    ///              }
    ///            },
    ///            "receiver_id": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub enum AccessKeyPermissionView {
        FullAccess,
        FunctionCall {
            allowance: ::std::string::String,
            method_names: ::std::vec::Vec<::std::string::String>,
            receiver_id: ::std::string::String,
        },
    }

    impl ::std::convert::From<&Self> for AccessKeyPermissionView {
        fn from(value: &AccessKeyPermissionView) -> Self {
            value.clone()
        }
    }

    ///AccessKeyView
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "nonce",
    ///    "permission"
    ///  ],
    ///  "properties": {
    ///    "nonce": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "permission": {
    ///      "$ref": "#/components/schemas/AccessKeyPermissionView"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AccessKeyView {
        pub nonce: u64,
        pub permission: AccessKeyPermissionView,
    }

    impl ::std::convert::From<&AccessKeyView> for AccessKeyView {
        fn from(value: &AccessKeyView) -> Self {
            value.clone()
        }
    }

    ///The structure describes configuration for creation of new accounts.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The structure describes configuration for creation of
    /// new accounts.",
    ///  "type": "object",
    ///  "required": [
    ///    "min_allowed_top_level_account_length",
    ///    "registrar_account_id"
    ///  ],
    ///  "properties": {
    ///    "min_allowed_top_level_account_length": {
    ///      "description": "The minimum length of the top-level account ID that
    /// is allowed to be created by any account.",
    ///      "type": "integer",
    ///      "format": "uint8",
    ///      "minimum": 0.0
    ///    },
    ///    "registrar_account_id": {
    ///      "description": "The account ID of the account registrar. This account ID allowed to create top-level accounts of any valid length.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/AccountId"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AccountCreationConfigView {
        ///The minimum length of the top-level account ID that is allowed to be
        /// created by any account.
        pub min_allowed_top_level_account_length: u8,
        ///The account ID of the account registrar. This account ID allowed to
        /// create top-level accounts of any valid length.
        pub registrar_account_id: AccountId,
    }

    impl ::std::convert::From<&AccountCreationConfigView> for AccountCreationConfigView {
        fn from(value: &AccountCreationConfigView) -> Self {
            value.clone()
        }
    }

    ///AccountDataView
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "account_key",
    ///    "peer_id",
    ///    "proxies",
    ///    "timestamp"
    ///  ],
    ///  "properties": {
    ///    "account_key": {
    ///      "$ref": "#/components/schemas/PublicKey"
    ///    },
    ///    "peer_id": {
    ///      "$ref": "#/components/schemas/PublicKey"
    ///    },
    ///    "proxies": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Tier1ProxyView"
    ///      }
    ///    },
    ///    "timestamp": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AccountDataView {
        pub account_key: PublicKey,
        pub peer_id: PublicKey,
        pub proxies: ::std::vec::Vec<Tier1ProxyView>,
        pub timestamp: ::std::string::String,
    }

    impl ::std::convert::From<&AccountDataView> for AccountDataView {
        fn from(value: &AccountDataView) -> Self {
            value.clone()
        }
    }

    ///NEAR Account Identifier.
    ///
    ///This is a unique, syntactically valid, human-readable account identifier
    /// on the NEAR network.
    ///
    ///[See the crate-level docs for information about
    /// validation.](index.html#account-id-rules)
    ///
    ///Also see [Error kind precedence](AccountId#error-kind-precedence).
    ///
    ///## Examples
    ///
    ///``` use near_account_id::AccountId;
    /// 
    /// let alice: AccountId = "alice.near".parse().unwrap();
    ///
    /// assert!("ƒelicia.near".parse::<AccountId>().is_err()); // (ƒ is not f) ```
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "NEAR Account Identifier.\n\nThis is a unique,
    /// syntactically valid, human-readable account identifier on the NEAR
    /// network.\n\n[See the crate-level docs for information about
    /// validation.](index.html#account-id-rules)\n\nAlso see [Error kind
    /// precedence](AccountId#error-kind-precedence).\n\n## Examples\n\n``` use
    /// near_account_id::AccountId;\n\nlet alice: AccountId =
    /// \"alice.near\".parse().unwrap();\n\nassert!(\"ƒelicia.near\".
    /// parse::<AccountId>().is_err()); // (ƒ is not f) ```",
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

    ///Account info for validators
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Account info for validators",
    ///  "type": "object",
    ///  "required": [
    ///    "account_id",
    ///    "amount",
    ///    "public_key"
    ///  ],
    ///  "properties": {
    ///    "account_id": {
    ///      "$ref": "#/components/schemas/AccountId"
    ///    },
    ///    "amount": {
    ///      "type": "string"
    ///    },
    ///    "public_key": {
    ///      "$ref": "#/components/schemas/PublicKey"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AccountInfo {
        pub account_id: AccountId,
        pub amount: ::std::string::String,
        pub public_key: PublicKey,
    }

    impl ::std::convert::From<&AccountInfo> for AccountInfo {
        fn from(value: &AccountInfo) -> Self {
            value.clone()
        }
    }

    ///Action
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "description": "Create an (sub)account using a transaction `receiver_id` as an ID for a new account ID must pass validation rules described here <http://nomicon.io/Primitives/Account.html>.",
    ///      "type": "object",
    ///      "required": [
    ///        "CreateAccount"
    ///      ],
    ///      "properties": {
    ///        "CreateAccount": {
    ///          "$ref": "#/components/schemas/CreateAccountAction"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "Sets a Wasm code to a receiver_id",
    ///      "type": "object",
    ///      "required": [
    ///        "DeployContract"
    ///      ],
    ///      "properties": {
    ///        "DeployContract": {
    ///          "$ref": "#/components/schemas/DeployContractAction"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "FunctionCall"
    ///      ],
    ///      "properties": {
    ///        "FunctionCall": {
    ///          "$ref": "#/components/schemas/FunctionCallAction"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "Transfer"
    ///      ],
    ///      "properties": {
    ///        "Transfer": {
    ///          "$ref": "#/components/schemas/TransferAction"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "Stake"
    ///      ],
    ///      "properties": {
    ///        "Stake": {
    ///          "$ref": "#/components/schemas/StakeAction"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "AddKey"
    ///      ],
    ///      "properties": {
    ///        "AddKey": {
    ///          "$ref": "#/components/schemas/AddKeyAction"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "DeleteKey"
    ///      ],
    ///      "properties": {
    ///        "DeleteKey": {
    ///          "$ref": "#/components/schemas/DeleteKeyAction"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "DeleteAccount"
    ///      ],
    ///      "properties": {
    ///        "DeleteAccount": {
    ///          "$ref": "#/components/schemas/DeleteAccountAction"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "Delegate"
    ///      ],
    ///      "properties": {
    ///        "Delegate": {
    ///          "$ref": "#/components/schemas/SignedDelegateAction"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "DeployGlobalContract"
    ///      ],
    ///      "properties": {
    ///        "DeployGlobalContract": {
    ///          "$ref": "#/components/schemas/DeployGlobalContractAction"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "UseGlobalContract"
    ///      ],
    ///      "properties": {
    ///        "UseGlobalContract": {
    ///          "$ref": "#/components/schemas/UseGlobalContractAction"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub enum Action {
        ///Create an (sub)account using a transaction `receiver_id` as an ID for a new account ID must pass validation rules described here <http://nomicon.io/Primitives/Account.html>.
        CreateAccount(CreateAccountAction),
        ///Sets a Wasm code to a receiver_id
        DeployContract(DeployContractAction),
        FunctionCall(FunctionCallAction),
        Transfer(TransferAction),
        Stake(StakeAction),
        AddKey(AddKeyAction),
        DeleteKey(DeleteKeyAction),
        DeleteAccount(DeleteAccountAction),
        Delegate(SignedDelegateAction),
        DeployGlobalContract(DeployGlobalContractAction),
        UseGlobalContract(UseGlobalContractAction),
    }

    impl ::std::convert::From<&Self> for Action {
        fn from(value: &Action) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<CreateAccountAction> for Action {
        fn from(value: CreateAccountAction) -> Self {
            Self::CreateAccount(value)
        }
    }

    impl ::std::convert::From<DeployContractAction> for Action {
        fn from(value: DeployContractAction) -> Self {
            Self::DeployContract(value)
        }
    }

    impl ::std::convert::From<FunctionCallAction> for Action {
        fn from(value: FunctionCallAction) -> Self {
            Self::FunctionCall(value)
        }
    }

    impl ::std::convert::From<TransferAction> for Action {
        fn from(value: TransferAction) -> Self {
            Self::Transfer(value)
        }
    }

    impl ::std::convert::From<StakeAction> for Action {
        fn from(value: StakeAction) -> Self {
            Self::Stake(value)
        }
    }

    impl ::std::convert::From<AddKeyAction> for Action {
        fn from(value: AddKeyAction) -> Self {
            Self::AddKey(value)
        }
    }

    impl ::std::convert::From<DeleteKeyAction> for Action {
        fn from(value: DeleteKeyAction) -> Self {
            Self::DeleteKey(value)
        }
    }

    impl ::std::convert::From<DeleteAccountAction> for Action {
        fn from(value: DeleteAccountAction) -> Self {
            Self::DeleteAccount(value)
        }
    }

    impl ::std::convert::From<SignedDelegateAction> for Action {
        fn from(value: SignedDelegateAction) -> Self {
            Self::Delegate(value)
        }
    }

    impl ::std::convert::From<DeployGlobalContractAction> for Action {
        fn from(value: DeployGlobalContractAction) -> Self {
            Self::DeployGlobalContract(value)
        }
    }

    impl ::std::convert::From<UseGlobalContractAction> for Action {
        fn from(value: UseGlobalContractAction) -> Self {
            Self::UseGlobalContract(value)
        }
    }

    ///Describes the cost of creating a specific action, `Action`. Includes all
    /// variants.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Describes the cost of creating a specific action,
    /// `Action`. Includes all variants.",
    ///  "type": "object",
    ///  "required": [
    ///    "add_key_cost",
    ///    "create_account_cost",
    ///    "delegate_cost",
    ///    "delete_account_cost",
    ///    "delete_key_cost",
    ///    "deploy_contract_cost",
    ///    "deploy_contract_cost_per_byte",
    ///    "function_call_cost",
    ///    "function_call_cost_per_byte",
    ///    "stake_cost",
    ///    "transfer_cost"
    ///  ],
    ///  "properties": {
    ///    "add_key_cost": {
    ///      "description": "Base cost of adding a key.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/AccessKeyCreationConfigView"
    ///        }
    ///      ]
    ///    },
    ///    "create_account_cost": {
    ///      "description": "Base cost of creating an account.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Fee"
    ///        }
    ///      ]
    ///    },
    ///    "delegate_cost": {
    ///      "description": "Base cost for processing a delegate action.\n\nThis
    /// is on top of the costs for the actions inside the delegate action.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Fee"
    ///        }
    ///      ]
    ///    },
    ///    "delete_account_cost": {
    ///      "description": "Base cost of deleting an account.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Fee"
    ///        }
    ///      ]
    ///    },
    ///    "delete_key_cost": {
    ///      "description": "Base cost of deleting a key.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Fee"
    ///        }
    ///      ]
    ///    },
    ///    "deploy_contract_cost": {
    ///      "description": "Base cost of deploying a contract.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Fee"
    ///        }
    ///      ]
    ///    },
    ///    "deploy_contract_cost_per_byte": {
    ///      "description": "Cost per byte of deploying a contract.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Fee"
    ///        }
    ///      ]
    ///    },
    ///    "function_call_cost": {
    ///      "description": "Base cost of calling a function.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Fee"
    ///        }
    ///      ]
    ///    },
    ///    "function_call_cost_per_byte": {
    ///      "description": "Cost per byte of method name and arguments of
    /// calling a function.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Fee"
    ///        }
    ///      ]
    ///    },
    ///    "stake_cost": {
    ///      "description": "Base cost of staking.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Fee"
    ///        }
    ///      ]
    ///    },
    ///    "transfer_cost": {
    ///      "description": "Base cost of making a transfer.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Fee"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ActionCreationConfigView {
        ///Base cost of adding a key.
        pub add_key_cost: AccessKeyCreationConfigView,
        ///Base cost of creating an account.
        pub create_account_cost: Fee,
        ///Base cost for processing a delegate action.
        ///
        ///This is on top of the costs for the actions inside the delegate
        /// action.
        pub delegate_cost: Fee,
        ///Base cost of deleting an account.
        pub delete_account_cost: Fee,
        ///Base cost of deleting a key.
        pub delete_key_cost: Fee,
        ///Base cost of deploying a contract.
        pub deploy_contract_cost: Fee,
        ///Cost per byte of deploying a contract.
        pub deploy_contract_cost_per_byte: Fee,
        ///Base cost of calling a function.
        pub function_call_cost: Fee,
        ///Cost per byte of method name and arguments of calling a function.
        pub function_call_cost_per_byte: Fee,
        ///Base cost of staking.
        pub stake_cost: Fee,
        ///Base cost of making a transfer.
        pub transfer_cost: Fee,
    }

    impl ::std::convert::From<&ActionCreationConfigView> for ActionCreationConfigView {
        fn from(value: &ActionCreationConfigView) -> Self {
            value.clone()
        }
    }

    ///An error happened during Action execution
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "An error happened during Action execution",
    ///  "type": "object",
    ///  "required": [
    ///    "kind"
    ///  ],
    ///  "properties": {
    ///    "index": {
    ///      "description": "Index of the failed action in the transaction.
    /// Action index is not defined if ActionError.kind is
    /// `ActionErrorKind::LackBalanceForState`",
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "kind": {
    ///      "description": "The kind of ActionError happened",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/ActionErrorKind"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ActionError {
        ///Index of the failed action in the transaction. Action index is not
        /// defined if ActionError.kind is
        /// `ActionErrorKind::LackBalanceForState`
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub index: ::std::option::Option<u64>,
        ///The kind of ActionError happened
        pub kind: ActionErrorKind,
    }

    impl ::std::convert::From<&ActionError> for ActionError {
        fn from(value: &ActionError) -> Self {
            value.clone()
        }
    }

    ///ActionErrorKind
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "description": "Happens when CreateAccount action tries to create
    /// an account with account_id which is already exists in the storage",
    ///      "type": "object",
    ///      "required": [
    ///        "AccountAlreadyExists"
    ///      ],
    ///      "properties": {
    ///        "AccountAlreadyExists": {
    ///          "type": "object",
    ///          "required": [
    ///            "account_id"
    ///          ],
    ///          "properties": {
    ///            "account_id": {
    ///              "$ref": "#/components/schemas/AccountId"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "Happens when TX receiver_id doesn't exist (but
    /// action is not Action::CreateAccount)",
    ///      "type": "object",
    ///      "required": [
    ///        "AccountDoesNotExist"
    ///      ],
    ///      "properties": {
    ///        "AccountDoesNotExist": {
    ///          "type": "object",
    ///          "required": [
    ///            "account_id"
    ///          ],
    ///          "properties": {
    ///            "account_id": {
    ///              "$ref": "#/components/schemas/AccountId"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "A top-level account ID can only be created by
    /// registrar.",
    ///      "type": "object",
    ///      "required": [
    ///        "CreateAccountOnlyByRegistrar"
    ///      ],
    ///      "properties": {
    ///        "CreateAccountOnlyByRegistrar": {
    ///          "type": "object",
    ///          "required": [
    ///            "account_id",
    ///            "predecessor_id",
    ///            "registrar_account_id"
    ///          ],
    ///          "properties": {
    ///            "account_id": {
    ///              "$ref": "#/components/schemas/AccountId"
    ///            },
    ///            "predecessor_id": {
    ///              "$ref": "#/components/schemas/AccountId"
    ///            },
    ///            "registrar_account_id": {
    ///              "$ref": "#/components/schemas/AccountId"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "A newly created account must be under a namespace
    /// of the creator account",
    ///      "type": "object",
    ///      "required": [
    ///        "CreateAccountNotAllowed"
    ///      ],
    ///      "properties": {
    ///        "CreateAccountNotAllowed": {
    ///          "type": "object",
    ///          "required": [
    ///            "account_id",
    ///            "predecessor_id"
    ///          ],
    ///          "properties": {
    ///            "account_id": {
    ///              "$ref": "#/components/schemas/AccountId"
    ///            },
    ///            "predecessor_id": {
    ///              "$ref": "#/components/schemas/AccountId"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "Administrative actions like `DeployContract`,
    /// `Stake`, `AddKey`, `DeleteKey`. can be proceed only if sender=receiver
    /// or the first TX action is a `CreateAccount` action",
    ///      "type": "object",
    ///      "required": [
    ///        "ActorNoPermission"
    ///      ],
    ///      "properties": {
    ///        "ActorNoPermission": {
    ///          "type": "object",
    ///          "required": [
    ///            "account_id",
    ///            "actor_id"
    ///          ],
    ///          "properties": {
    ///            "account_id": {
    ///              "$ref": "#/components/schemas/AccountId"
    ///            },
    ///            "actor_id": {
    ///              "$ref": "#/components/schemas/AccountId"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "Account tries to remove an access key that doesn't
    /// exist",
    ///      "type": "object",
    ///      "required": [
    ///        "DeleteKeyDoesNotExist"
    ///      ],
    ///      "properties": {
    ///        "DeleteKeyDoesNotExist": {
    ///          "type": "object",
    ///          "required": [
    ///            "account_id",
    ///            "public_key"
    ///          ],
    ///          "properties": {
    ///            "account_id": {
    ///              "$ref": "#/components/schemas/AccountId"
    ///            },
    ///            "public_key": {
    ///              "$ref": "#/components/schemas/PublicKey"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "The public key is already used for an existing
    /// access key",
    ///      "type": "object",
    ///      "required": [
    ///        "AddKeyAlreadyExists"
    ///      ],
    ///      "properties": {
    ///        "AddKeyAlreadyExists": {
    ///          "type": "object",
    ///          "required": [
    ///            "account_id",
    ///            "public_key"
    ///          ],
    ///          "properties": {
    ///            "account_id": {
    ///              "$ref": "#/components/schemas/AccountId"
    ///            },
    ///            "public_key": {
    ///              "$ref": "#/components/schemas/PublicKey"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "Account is staking and can not be deleted",
    ///      "type": "object",
    ///      "required": [
    ///        "DeleteAccountStaking"
    ///      ],
    ///      "properties": {
    ///        "DeleteAccountStaking": {
    ///          "type": "object",
    ///          "required": [
    ///            "account_id"
    ///          ],
    ///          "properties": {
    ///            "account_id": {
    ///              "$ref": "#/components/schemas/AccountId"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "ActionReceipt can't be completed, because the
    /// remaining balance will not be enough to cover storage.",
    ///      "type": "object",
    ///      "required": [
    ///        "LackBalanceForState"
    ///      ],
    ///      "properties": {
    ///        "LackBalanceForState": {
    ///          "type": "object",
    ///          "required": [
    ///            "account_id",
    ///            "amount"
    ///          ],
    ///          "properties": {
    ///            "account_id": {
    ///              "description": "An account which needs balance",
    ///              "allOf": [
    ///                {
    ///                  "$ref": "#/components/schemas/AccountId"
    ///                }
    ///              ]
    ///            },
    ///            "amount": {
    ///              "description": "Balance required to complete an action.",
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "Account is not yet staked, but tries to unstake",
    ///      "type": "object",
    ///      "required": [
    ///        "TriesToUnstake"
    ///      ],
    ///      "properties": {
    ///        "TriesToUnstake": {
    ///          "type": "object",
    ///          "required": [
    ///            "account_id"
    ///          ],
    ///          "properties": {
    ///            "account_id": {
    ///              "$ref": "#/components/schemas/AccountId"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "The account doesn't have enough balance to increase
    /// the stake.",
    ///      "type": "object",
    ///      "required": [
    ///        "TriesToStake"
    ///      ],
    ///      "properties": {
    ///        "TriesToStake": {
    ///          "type": "object",
    ///          "required": [
    ///            "account_id",
    ///            "balance",
    ///            "locked",
    ///            "stake"
    ///          ],
    ///          "properties": {
    ///            "account_id": {
    ///              "$ref": "#/components/schemas/AccountId"
    ///            },
    ///            "balance": {
    ///              "type": "string"
    ///            },
    ///            "locked": {
    ///              "type": "string"
    ///            },
    ///            "stake": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "InsufficientStake"
    ///      ],
    ///      "properties": {
    ///        "InsufficientStake": {
    ///          "type": "object",
    ///          "required": [
    ///            "account_id",
    ///            "minimum_stake",
    ///            "stake"
    ///          ],
    ///          "properties": {
    ///            "account_id": {
    ///              "$ref": "#/components/schemas/AccountId"
    ///            },
    ///            "minimum_stake": {
    ///              "type": "string"
    ///            },
    ///            "stake": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "An error occurred during a `FunctionCall` Action,
    /// parameter is debug message.",
    ///      "type": "object",
    ///      "required": [
    ///        "FunctionCallError"
    ///      ],
    ///      "properties": {
    ///        "FunctionCallError": {
    ///          "$ref": "#/components/schemas/FunctionCallError"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "Error occurs when a new `ActionReceipt` created by
    /// the `FunctionCall` action fails receipt validation.",
    ///      "type": "object",
    ///      "required": [
    ///        "NewReceiptValidationError"
    ///      ],
    ///      "properties": {
    ///        "NewReceiptValidationError": {
    ///          "$ref": "#/components/schemas/ReceiptValidationError"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "Error occurs when a `CreateAccount` action is called on a NEAR-implicit or ETH-implicit account. See NEAR-implicit account creation NEP: <https://github.com/nearprotocol/NEPs/pull/71>. Also, see ETH-implicit account creation NEP: <https://github.com/near/NEPs/issues/518>.\n\nTODO(#8598): This error is named very poorly. A better name would be `OnlyNamedAccountCreationAllowed`.",
    ///      "type": "object",
    ///      "required": [
    ///        "OnlyImplicitAccountCreationAllowed"
    ///      ],
    ///      "properties": {
    ///        "OnlyImplicitAccountCreationAllowed": {
    ///          "type": "object",
    ///          "required": [
    ///            "account_id"
    ///          ],
    ///          "properties": {
    ///            "account_id": {
    ///              "$ref": "#/components/schemas/AccountId"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "Delete account whose state is large is temporarily
    /// banned.",
    ///      "type": "object",
    ///      "required": [
    ///        "DeleteAccountWithLargeState"
    ///      ],
    ///      "properties": {
    ///        "DeleteAccountWithLargeState": {
    ///          "type": "object",
    ///          "required": [
    ///            "account_id"
    ///          ],
    ///          "properties": {
    ///            "account_id": {
    ///              "$ref": "#/components/schemas/AccountId"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "Signature does not match the provided actions and
    /// given signer public key.",
    ///      "type": "string",
    ///      "enum": [
    ///        "DelegateActionInvalidSignature"
    ///      ]
    ///    },
    ///    {
    ///      "description": "Receiver of the transaction doesn't match Sender of
    /// the delegate action",
    ///      "type": "object",
    ///      "required": [
    ///        "DelegateActionSenderDoesNotMatchTxReceiver"
    ///      ],
    ///      "properties": {
    ///        "DelegateActionSenderDoesNotMatchTxReceiver": {
    ///          "type": "object",
    ///          "required": [
    ///            "receiver_id",
    ///            "sender_id"
    ///          ],
    ///          "properties": {
    ///            "receiver_id": {
    ///              "$ref": "#/components/schemas/AccountId"
    ///            },
    ///            "sender_id": {
    ///              "$ref": "#/components/schemas/AccountId"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "Delegate action has expired. `max_block_height` is
    /// less than actual block height.",
    ///      "type": "string",
    ///      "enum": [
    ///        "DelegateActionExpired"
    ///      ]
    ///    },
    ///    {
    ///      "description": "The given public key doesn't exist for Sender
    /// account",
    ///      "type": "object",
    ///      "required": [
    ///        "DelegateActionAccessKeyError"
    ///      ],
    ///      "properties": {
    ///        "DelegateActionAccessKeyError": {
    ///          "$ref": "#/components/schemas/InvalidAccessKeyError"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "DelegateAction nonce must be greater
    /// sender[public_key].nonce",
    ///      "type": "object",
    ///      "required": [
    ///        "DelegateActionInvalidNonce"
    ///      ],
    ///      "properties": {
    ///        "DelegateActionInvalidNonce": {
    ///          "type": "object",
    ///          "required": [
    ///            "ak_nonce",
    ///            "delegate_nonce"
    ///          ],
    ///          "properties": {
    ///            "ak_nonce": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            },
    ///            "delegate_nonce": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "DelegateAction nonce is larger than the upper bound
    /// given by the block height",
    ///      "type": "object",
    ///      "required": [
    ///        "DelegateActionNonceTooLarge"
    ///      ],
    ///      "properties": {
    ///        "DelegateActionNonceTooLarge": {
    ///          "type": "object",
    ///          "required": [
    ///            "delegate_nonce",
    ///            "upper_bound"
    ///          ],
    ///          "properties": {
    ///            "delegate_nonce": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            },
    ///            "upper_bound": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "GlobalContractDoesNotExist"
    ///      ],
    ///      "properties": {
    ///        "GlobalContractDoesNotExist": {
    ///          "type": "object",
    ///          "required": [
    ///            "identifier"
    ///          ],
    ///          "properties": {
    ///            "identifier": {
    ///              "$ref": "#/components/schemas/GlobalContractIdentifier"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub enum ActionErrorKind {
        ///Happens when CreateAccount action tries to create an account with
        /// account_id which is already exists in the storage
        AccountAlreadyExists { account_id: AccountId },
        ///Happens when TX receiver_id doesn't exist (but action is not
        /// Action::CreateAccount)
        AccountDoesNotExist { account_id: AccountId },
        ///A top-level account ID can only be created by registrar.
        CreateAccountOnlyByRegistrar {
            account_id: AccountId,
            predecessor_id: AccountId,
            registrar_account_id: AccountId,
        },
        ///A newly created account must be under a namespace of the creator
        /// account
        CreateAccountNotAllowed {
            account_id: AccountId,
            predecessor_id: AccountId,
        },
        ///Administrative actions like `DeployContract`, `Stake`, `AddKey`,
        /// `DeleteKey`. can be proceed only if sender=receiver or the first TX
        /// action is a `CreateAccount` action
        ActorNoPermission {
            account_id: AccountId,
            actor_id: AccountId,
        },
        ///Account tries to remove an access key that doesn't exist
        DeleteKeyDoesNotExist {
            account_id: AccountId,
            public_key: PublicKey,
        },
        ///The public key is already used for an existing access key
        AddKeyAlreadyExists {
            account_id: AccountId,
            public_key: PublicKey,
        },
        ///Account is staking and can not be deleted
        DeleteAccountStaking { account_id: AccountId },
        ///ActionReceipt can't be completed, because the remaining balance will
        /// not be enough to cover storage.
        LackBalanceForState {
            ///An account which needs balance
            account_id: AccountId,
            ///Balance required to complete an action.
            amount: ::std::string::String,
        },
        ///Account is not yet staked, but tries to unstake
        TriesToUnstake { account_id: AccountId },
        ///The account doesn't have enough balance to increase the stake.
        TriesToStake {
            account_id: AccountId,
            balance: ::std::string::String,
            locked: ::std::string::String,
            stake: ::std::string::String,
        },
        InsufficientStake {
            account_id: AccountId,
            minimum_stake: ::std::string::String,
            stake: ::std::string::String,
        },
        ///An error occurred during a `FunctionCall` Action, parameter is debug
        /// message.
        FunctionCallError(FunctionCallError),
        ///Error occurs when a new `ActionReceipt` created by the
        /// `FunctionCall` action fails receipt validation.
        NewReceiptValidationError(ReceiptValidationError),
        ///Error occurs when a `CreateAccount` action is called on a NEAR-implicit or ETH-implicit account. See NEAR-implicit account creation NEP: <https://github.com/nearprotocol/NEPs/pull/71>. Also, see ETH-implicit account creation NEP: <https://github.com/near/NEPs/issues/518>.
        ///
        ///TODO(#8598): This error is named very poorly. A better name would be
        /// `OnlyNamedAccountCreationAllowed`.
        OnlyImplicitAccountCreationAllowed { account_id: AccountId },
        ///Delete account whose state is large is temporarily banned.
        DeleteAccountWithLargeState { account_id: AccountId },
        ///Signature does not match the provided actions and given signer
        /// public key.
        DelegateActionInvalidSignature,
        ///Receiver of the transaction doesn't match Sender of the delegate
        /// action
        DelegateActionSenderDoesNotMatchTxReceiver {
            receiver_id: AccountId,
            sender_id: AccountId,
        },
        ///Delegate action has expired. `max_block_height` is less than actual
        /// block height.
        DelegateActionExpired,
        ///The given public key doesn't exist for Sender account
        DelegateActionAccessKeyError(InvalidAccessKeyError),
        ///DelegateAction nonce must be greater sender[public_key].nonce
        DelegateActionInvalidNonce { ak_nonce: u64, delegate_nonce: u64 },
        ///DelegateAction nonce is larger than the upper bound given by the
        /// block height
        DelegateActionNonceTooLarge {
            delegate_nonce: u64,
            upper_bound: u64,
        },
        GlobalContractDoesNotExist {
            identifier: GlobalContractIdentifier,
        },
    }

    impl ::std::convert::From<&Self> for ActionErrorKind {
        fn from(value: &ActionErrorKind) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<FunctionCallError> for ActionErrorKind {
        fn from(value: FunctionCallError) -> Self {
            Self::FunctionCallError(value)
        }
    }

    impl ::std::convert::From<ReceiptValidationError> for ActionErrorKind {
        fn from(value: ReceiptValidationError) -> Self {
            Self::NewReceiptValidationError(value)
        }
    }

    impl ::std::convert::From<InvalidAccessKeyError> for ActionErrorKind {
        fn from(value: InvalidAccessKeyError) -> Self {
            Self::DelegateActionAccessKeyError(value)
        }
    }

    ///ActionView
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "string",
    ///      "enum": [
    ///        "CreateAccount"
    ///      ]
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "DeployContract"
    ///      ],
    ///      "properties": {
    ///        "DeployContract": {
    ///          "type": "object",
    ///          "required": [
    ///            "code"
    ///          ],
    ///          "properties": {
    ///            "code": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "FunctionCall"
    ///      ],
    ///      "properties": {
    ///        "FunctionCall": {
    ///          "type": "object",
    ///          "required": [
    ///            "args",
    ///            "deposit",
    ///            "gas",
    ///            "method_name"
    ///          ],
    ///          "properties": {
    ///            "args": {
    ///              "type": "string"
    ///            },
    ///            "deposit": {
    ///              "type": "string"
    ///            },
    ///            "gas": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            },
    ///            "method_name": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "Transfer"
    ///      ],
    ///      "properties": {
    ///        "Transfer": {
    ///          "type": "object",
    ///          "required": [
    ///            "deposit"
    ///          ],
    ///          "properties": {
    ///            "deposit": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "Stake"
    ///      ],
    ///      "properties": {
    ///        "Stake": {
    ///          "type": "object",
    ///          "required": [
    ///            "public_key",
    ///            "stake"
    ///          ],
    ///          "properties": {
    ///            "public_key": {
    ///              "$ref": "#/components/schemas/PublicKey"
    ///            },
    ///            "stake": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "AddKey"
    ///      ],
    ///      "properties": {
    ///        "AddKey": {
    ///          "type": "object",
    ///          "required": [
    ///            "access_key",
    ///            "public_key"
    ///          ],
    ///          "properties": {
    ///            "access_key": {
    ///              "$ref": "#/components/schemas/AccessKeyView"
    ///            },
    ///            "public_key": {
    ///              "$ref": "#/components/schemas/PublicKey"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "DeleteKey"
    ///      ],
    ///      "properties": {
    ///        "DeleteKey": {
    ///          "type": "object",
    ///          "required": [
    ///            "public_key"
    ///          ],
    ///          "properties": {
    ///            "public_key": {
    ///              "$ref": "#/components/schemas/PublicKey"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "DeleteAccount"
    ///      ],
    ///      "properties": {
    ///        "DeleteAccount": {
    ///          "type": "object",
    ///          "required": [
    ///            "beneficiary_id"
    ///          ],
    ///          "properties": {
    ///            "beneficiary_id": {
    ///              "$ref": "#/components/schemas/AccountId"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "Delegate"
    ///      ],
    ///      "properties": {
    ///        "Delegate": {
    ///          "type": "object",
    ///          "required": [
    ///            "delegate_action",
    ///            "signature"
    ///          ],
    ///          "properties": {
    ///            "delegate_action": {
    ///              "$ref": "#/components/schemas/DelegateAction"
    ///            },
    ///            "signature": {
    ///              "$ref": "#/components/schemas/Signature"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "DeployGlobalContract"
    ///      ],
    ///      "properties": {
    ///        "DeployGlobalContract": {
    ///          "type": "object",
    ///          "required": [
    ///            "code"
    ///          ],
    ///          "properties": {
    ///            "code": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "DeployGlobalContractByAccountId"
    ///      ],
    ///      "properties": {
    ///        "DeployGlobalContractByAccountId": {
    ///          "type": "object",
    ///          "required": [
    ///            "code"
    ///          ],
    ///          "properties": {
    ///            "code": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "UseGlobalContract"
    ///      ],
    ///      "properties": {
    ///        "UseGlobalContract": {
    ///          "type": "object",
    ///          "required": [
    ///            "code_hash"
    ///          ],
    ///          "properties": {
    ///            "code_hash": {
    ///              "$ref": "#/components/schemas/CryptoHash"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "UseGlobalContractByAccountId"
    ///      ],
    ///      "properties": {
    ///        "UseGlobalContractByAccountId": {
    ///          "type": "object",
    ///          "required": [
    ///            "account_id"
    ///          ],
    ///          "properties": {
    ///            "account_id": {
    ///              "$ref": "#/components/schemas/AccountId"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub enum ActionView {
        CreateAccount,
        DeployContract {
            code: ::std::string::String,
        },
        FunctionCall {
            args: ::std::string::String,
            deposit: ::std::string::String,
            gas: u64,
            method_name: ::std::string::String,
        },
        Transfer {
            deposit: ::std::string::String,
        },
        Stake {
            public_key: PublicKey,
            stake: ::std::string::String,
        },
        AddKey {
            access_key: AccessKeyView,
            public_key: PublicKey,
        },
        DeleteKey {
            public_key: PublicKey,
        },
        DeleteAccount {
            beneficiary_id: AccountId,
        },
        Delegate {
            delegate_action: DelegateAction,
            signature: Signature,
        },
        DeployGlobalContract {
            code: ::std::string::String,
        },
        DeployGlobalContractByAccountId {
            code: ::std::string::String,
        },
        UseGlobalContract {
            code_hash: CryptoHash,
        },
        UseGlobalContractByAccountId {
            account_id: AccountId,
        },
    }

    impl ::std::convert::From<&Self> for ActionView {
        fn from(value: &ActionView) -> Self {
            value.clone()
        }
    }

    ///Describes the error for validating a list of actions.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Describes the error for validating a list of actions.",
    ///  "oneOf": [
    ///    {
    ///      "description": "The delete action must be a final action in
    /// transaction",
    ///      "type": "string",
    ///      "enum": [
    ///        "DeleteActionMustBeFinal"
    ///      ]
    ///    },
    ///    {
    ///      "description": "The total prepaid gas (for all given actions)
    /// exceeded the limit.",
    ///      "type": "object",
    ///      "required": [
    ///        "TotalPrepaidGasExceeded"
    ///      ],
    ///      "properties": {
    ///        "TotalPrepaidGasExceeded": {
    ///          "type": "object",
    ///          "required": [
    ///            "limit",
    ///            "total_prepaid_gas"
    ///          ],
    ///          "properties": {
    ///            "limit": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            },
    ///            "total_prepaid_gas": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "The number of actions exceeded the given limit.",
    ///      "type": "object",
    ///      "required": [
    ///        "TotalNumberOfActionsExceeded"
    ///      ],
    ///      "properties": {
    ///        "TotalNumberOfActionsExceeded": {
    ///          "type": "object",
    ///          "required": [
    ///            "limit",
    ///            "total_number_of_actions"
    ///          ],
    ///          "properties": {
    ///            "limit": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            },
    ///            "total_number_of_actions": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "The total number of bytes of the method names
    /// exceeded the limit in a Add Key action.",
    ///      "type": "object",
    ///      "required": [
    ///        "AddKeyMethodNamesNumberOfBytesExceeded"
    ///      ],
    ///      "properties": {
    ///        "AddKeyMethodNamesNumberOfBytesExceeded": {
    ///          "type": "object",
    ///          "required": [
    ///            "limit",
    ///            "total_number_of_bytes"
    ///          ],
    ///          "properties": {
    ///            "limit": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            },
    ///            "total_number_of_bytes": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "The length of some method name exceeded the limit
    /// in a Add Key action.",
    ///      "type": "object",
    ///      "required": [
    ///        "AddKeyMethodNameLengthExceeded"
    ///      ],
    ///      "properties": {
    ///        "AddKeyMethodNameLengthExceeded": {
    ///          "type": "object",
    ///          "required": [
    ///            "length",
    ///            "limit"
    ///          ],
    ///          "properties": {
    ///            "length": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            },
    ///            "limit": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "Integer overflow during a compute.",
    ///      "type": "string",
    ///      "enum": [
    ///        "IntegerOverflow"
    ///      ]
    ///    },
    ///    {
    ///      "description": "Invalid account ID.",
    ///      "type": "object",
    ///      "required": [
    ///        "InvalidAccountId"
    ///      ],
    ///      "properties": {
    ///        "InvalidAccountId": {
    ///          "type": "object",
    ///          "required": [
    ///            "account_id"
    ///          ],
    ///          "properties": {
    ///            "account_id": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "The size of the contract code exceeded the limit in
    /// a DeployContract action.",
    ///      "type": "object",
    ///      "required": [
    ///        "ContractSizeExceeded"
    ///      ],
    ///      "properties": {
    ///        "ContractSizeExceeded": {
    ///          "type": "object",
    ///          "required": [
    ///            "limit",
    ///            "size"
    ///          ],
    ///          "properties": {
    ///            "limit": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            },
    ///            "size": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "The length of the method name exceeded the limit in
    /// a Function Call action.",
    ///      "type": "object",
    ///      "required": [
    ///        "FunctionCallMethodNameLengthExceeded"
    ///      ],
    ///      "properties": {
    ///        "FunctionCallMethodNameLengthExceeded": {
    ///          "type": "object",
    ///          "required": [
    ///            "length",
    ///            "limit"
    ///          ],
    ///          "properties": {
    ///            "length": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            },
    ///            "limit": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "The length of the arguments exceeded the limit in a
    /// Function Call action.",
    ///      "type": "object",
    ///      "required": [
    ///        "FunctionCallArgumentsLengthExceeded"
    ///      ],
    ///      "properties": {
    ///        "FunctionCallArgumentsLengthExceeded": {
    ///          "type": "object",
    ///          "required": [
    ///            "length",
    ///            "limit"
    ///          ],
    ///          "properties": {
    ///            "length": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            },
    ///            "limit": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "An attempt to stake with a public key that is not
    /// convertible to ristretto.",
    ///      "type": "object",
    ///      "required": [
    ///        "UnsuitableStakingKey"
    ///      ],
    ///      "properties": {
    ///        "UnsuitableStakingKey": {
    ///          "type": "object",
    ///          "required": [
    ///            "public_key"
    ///          ],
    ///          "properties": {
    ///            "public_key": {
    ///              "$ref": "#/components/schemas/PublicKey"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "The attached amount of gas in a FunctionCall action
    /// has to be a positive number.",
    ///      "type": "string",
    ///      "enum": [
    ///        "FunctionCallZeroAttachedGas"
    ///      ]
    ///    },
    ///    {
    ///      "description": "There should be the only one DelegateAction",
    ///      "type": "string",
    ///      "enum": [
    ///        "DelegateActionMustBeOnlyOne"
    ///      ]
    ///    },
    ///    {
    ///      "description": "The transaction includes a feature that the current
    /// protocol version does not support.\n\nNote: we stringify the protocol
    /// feature name instead of using `ProtocolFeature` here because we don't
    /// want to leak the internals of that type into observable borsh
    /// serialization.",
    ///      "type": "object",
    ///      "required": [
    ///        "UnsupportedProtocolFeature"
    ///      ],
    ///      "properties": {
    ///        "UnsupportedProtocolFeature": {
    ///          "type": "object",
    ///          "required": [
    ///            "protocol_feature",
    ///            "version"
    ///          ],
    ///          "properties": {
    ///            "protocol_feature": {
    ///              "type": "string"
    ///            },
    ///            "version": {
    ///              "type": "integer",
    ///              "format": "uint32",
    ///              "minimum": 0.0
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub enum ActionsValidationError {
        ///The delete action must be a final action in transaction
        DeleteActionMustBeFinal,
        ///The total prepaid gas (for all given actions) exceeded the limit.
        TotalPrepaidGasExceeded { limit: u64, total_prepaid_gas: u64 },
        ///The number of actions exceeded the given limit.
        TotalNumberOfActionsExceeded {
            limit: u64,
            total_number_of_actions: u64,
        },
        ///The total number of bytes of the method names exceeded the limit in
        /// a Add Key action.
        AddKeyMethodNamesNumberOfBytesExceeded {
            limit: u64,
            total_number_of_bytes: u64,
        },
        ///The length of some method name exceeded the limit in a Add Key
        /// action.
        AddKeyMethodNameLengthExceeded { length: u64, limit: u64 },
        ///Integer overflow during a compute.
        IntegerOverflow,
        ///Invalid account ID.
        InvalidAccountId { account_id: ::std::string::String },
        ///The size of the contract code exceeded the limit in a DeployContract
        /// action.
        ContractSizeExceeded { limit: u64, size: u64 },
        ///The length of the method name exceeded the limit in a Function Call
        /// action.
        FunctionCallMethodNameLengthExceeded { length: u64, limit: u64 },
        ///The length of the arguments exceeded the limit in a Function Call
        /// action.
        FunctionCallArgumentsLengthExceeded { length: u64, limit: u64 },
        ///An attempt to stake with a public key that is not convertible to
        /// ristretto.
        UnsuitableStakingKey { public_key: PublicKey },
        ///The attached amount of gas in a FunctionCall action has to be a
        /// positive number.
        FunctionCallZeroAttachedGas,
        ///There should be the only one DelegateAction
        DelegateActionMustBeOnlyOne,
        ///The transaction includes a feature that the current protocol version
        /// does not support.
        ///
        ///Note: we stringify the protocol feature name instead of using
        /// `ProtocolFeature` here because we don't want to leak the internals
        /// of that type into observable borsh serialization.
        UnsupportedProtocolFeature {
            protocol_feature: ::std::string::String,
            version: u32,
        },
    }

    impl ::std::convert::From<&Self> for ActionsValidationError {
        fn from(value: &ActionsValidationError) -> Self {
            value.clone()
        }
    }

    ///AddKeyAction
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "access_key",
    ///    "public_key"
    ///  ],
    ///  "properties": {
    ///    "access_key": {
    ///      "description": "An access key with the permission",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/AccessKey"
    ///        }
    ///      ]
    ///    },
    ///    "public_key": {
    ///      "description": "A public key which will be associated with an
    /// access_key",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/PublicKey"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AddKeyAction {
        ///An access key with the permission
        pub access_key: AccessKey,
        ///A public key which will be associated with an access_key
        pub public_key: PublicKey,
    }

    impl ::std::convert::From<&AddKeyAction> for AddKeyAction {
        fn from(value: &AddKeyAction) -> Self {
            value.clone()
        }
    }

    ///`BandwidthRequest` describes the size of receipts that a shard would
    /// like to send to another shard. When a shard wants to send a lot of
    /// receipts to another shard, it needs to create a request and wait for a
    /// bandwidth grant from the bandwidth scheduler.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "`BandwidthRequest` describes the size of receipts that
    /// a shard would like to send to another shard. When a shard wants to send
    /// a lot of receipts to another shard, it needs to create a request and
    /// wait for a bandwidth grant from the bandwidth scheduler.",
    ///  "type": "object",
    ///  "required": [
    ///    "requested_values_bitmap",
    ///    "to_shard"
    ///  ],
    ///  "properties": {
    ///    "requested_values_bitmap": {
    ///      "description": "Bitmap which describes what values of bandwidth are
    /// requested.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/BandwidthRequestBitmap"
    ///        }
    ///      ]
    ///    },
    ///    "to_shard": {
    ///      "description": "Requesting bandwidth to this shard.",
    ///      "type": "integer",
    ///      "format": "uint16",
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct BandwidthRequest {
        ///Bitmap which describes what values of bandwidth are requested.
        pub requested_values_bitmap: BandwidthRequestBitmap,
        ///Requesting bandwidth to this shard.
        pub to_shard: u16,
    }

    impl ::std::convert::From<&BandwidthRequest> for BandwidthRequest {
        fn from(value: &BandwidthRequest) -> Self {
            value.clone()
        }
    }

    ///Bitmap which describes which values from the predefined list are being
    /// requested. The nth bit is set to 1 when the nth value from the list is
    /// being requested.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Bitmap which describes which values from the predefined
    /// list are being requested. The nth bit is set to 1 when the nth value
    /// from the list is being requested.",
    ///  "type": "object",
    ///  "required": [
    ///    "data"
    ///  ],
    ///  "properties": {
    ///    "data": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "integer",
    ///        "format": "uint8",
    ///        "minimum": 0.0
    ///      },
    ///      "maxItems": 5,
    ///      "minItems": 5
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct BandwidthRequestBitmap {
        pub data: [u8; 5usize],
    }

    impl ::std::convert::From<&BandwidthRequestBitmap> for BandwidthRequestBitmap {
        fn from(value: &BandwidthRequestBitmap) -> Self {
            value.clone()
        }
    }

    ///A list of shard's bandwidth requests. Describes how much the shard would
    /// like to send to other shards.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A list of shard's bandwidth requests. Describes how
    /// much the shard would like to send to other shards.",
    ///  "oneOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "V1"
    ///      ],
    ///      "properties": {
    ///        "V1": {
    ///          "$ref": "#/components/schemas/BandwidthRequestsV1"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub enum BandwidthRequests {
        V1(BandwidthRequestsV1),
    }

    impl ::std::convert::From<&Self> for BandwidthRequests {
        fn from(value: &BandwidthRequests) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<BandwidthRequestsV1> for BandwidthRequests {
        fn from(value: BandwidthRequestsV1) -> Self {
            Self::V1(value)
        }
    }

    ///BandwidthRequestsV1
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "requests"
    ///  ],
    ///  "properties": {
    ///    "requests": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/BandwidthRequest"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct BandwidthRequestsV1 {
        pub requests: ::std::vec::Vec<BandwidthRequest>,
    }

    impl ::std::convert::From<&BandwidthRequestsV1> for BandwidthRequestsV1 {
        fn from(value: &BandwidthRequestsV1) -> Self {
            value.clone()
        }
    }

    ///BlockHeaderInnerLiteView
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "block_merkle_root",
    ///    "epoch_id",
    ///    "height",
    ///    "next_bp_hash",
    ///    "next_epoch_id",
    ///    "outcome_root",
    ///    "prev_state_root",
    ///    "timestamp",
    ///    "timestamp_nanosec"
    ///  ],
    ///  "properties": {
    ///    "block_merkle_root": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "epoch_id": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "height": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "next_bp_hash": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "next_epoch_id": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "outcome_root": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "prev_state_root": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "timestamp": {
    ///      "description": "Legacy json number. Should not be used.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "timestamp_nanosec": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct BlockHeaderInnerLiteView {
        pub block_merkle_root: CryptoHash,
        pub epoch_id: CryptoHash,
        pub height: u64,
        pub next_bp_hash: CryptoHash,
        pub next_epoch_id: CryptoHash,
        pub outcome_root: CryptoHash,
        pub prev_state_root: CryptoHash,
        ///Legacy json number. Should not be used.
        pub timestamp: u64,
        pub timestamp_nanosec: ::std::string::String,
    }

    impl ::std::convert::From<&BlockHeaderInnerLiteView> for BlockHeaderInnerLiteView {
        fn from(value: &BlockHeaderInnerLiteView) -> Self {
            value.clone()
        }
    }

    ///BlockHeaderView
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "approvals",
    ///    "block_merkle_root",
    ///    "challenges_result",
    ///    "challenges_root",
    ///    "chunk_headers_root",
    ///    "chunk_mask",
    ///    "chunk_receipts_root",
    ///    "chunk_tx_root",
    ///    "chunks_included",
    ///    "epoch_id",
    ///    "gas_price",
    ///    "hash",
    ///    "height",
    ///    "last_ds_final_block",
    ///    "last_final_block",
    ///    "latest_protocol_version",
    ///    "next_bp_hash",
    ///    "next_epoch_id",
    ///    "outcome_root",
    ///    "prev_hash",
    ///    "prev_state_root",
    ///    "random_value",
    ///    "rent_paid",
    ///    "signature",
    ///    "timestamp",
    ///    "timestamp_nanosec",
    ///    "total_supply",
    ///    "validator_proposals",
    ///    "validator_reward"
    ///  ],
    ///  "properties": {
    ///    "approvals": {
    ///      "type": "array",
    ///      "items": {
    ///        "oneOf": [
    ///          {
    ///            "type": "null"
    ///          },
    ///          {
    ///            "allOf": [
    ///              {
    ///                "$ref": "#/components/schemas/Signature"
    ///              }
    ///            ]
    ///          }
    ///        ]
    ///      }
    ///    },
    ///    "block_body_hash": {
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/CryptoHash"
    ///            }
    ///          ]
    ///        }
    ///      ]
    ///    },
    ///    "block_merkle_root": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "block_ordinal": {
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "challenges_result": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/SlashedValidator"
    ///      }
    ///    },
    ///    "challenges_root": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "chunk_endorsements": {
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "array",
    ///        "items": {
    ///          "type": "integer",
    ///          "format": "uint8",
    ///          "minimum": 0.0
    ///        }
    ///      }
    ///    },
    ///    "chunk_headers_root": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "chunk_mask": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "boolean"
    ///      }
    ///    },
    ///    "chunk_receipts_root": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "chunk_tx_root": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "chunks_included": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "epoch_id": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "epoch_sync_data_hash": {
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/CryptoHash"
    ///            }
    ///          ]
    ///        }
    ///      ]
    ///    },
    ///    "gas_price": {
    ///      "type": "string"
    ///    },
    ///    "hash": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "height": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "last_ds_final_block": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "last_final_block": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "latest_protocol_version": {
    ///      "type": "integer",
    ///      "format": "uint32",
    ///      "minimum": 0.0
    ///    },
    ///    "next_bp_hash": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "next_epoch_id": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "outcome_root": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "prev_hash": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "prev_height": {
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "prev_state_root": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "random_value": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "rent_paid": {
    ///      "description": "TODO(2271): deprecated.",
    ///      "type": "string"
    ///    },
    ///    "signature": {
    ///      "$ref": "#/components/schemas/Signature"
    ///    },
    ///    "timestamp": {
    ///      "description": "Legacy json number. Should not be used.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "timestamp_nanosec": {
    ///      "type": "string"
    ///    },
    ///    "total_supply": {
    ///      "type": "string"
    ///    },
    ///    "validator_proposals": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ValidatorStakeView"
    ///      }
    ///    },
    ///    "validator_reward": {
    ///      "description": "TODO(2271): deprecated.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct BlockHeaderView {
        pub approvals: ::std::vec::Vec<::std::option::Option<Signature>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub block_body_hash: ::std::option::Option<CryptoHash>,
        pub block_merkle_root: CryptoHash,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub block_ordinal: ::std::option::Option<u64>,
        pub challenges_result: ::std::vec::Vec<SlashedValidator>,
        pub challenges_root: CryptoHash,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub chunk_endorsements: ::std::option::Option<::std::vec::Vec<::std::vec::Vec<u8>>>,
        pub chunk_headers_root: CryptoHash,
        pub chunk_mask: ::std::vec::Vec<bool>,
        pub chunk_receipts_root: CryptoHash,
        pub chunk_tx_root: CryptoHash,
        pub chunks_included: u64,
        pub epoch_id: CryptoHash,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub epoch_sync_data_hash: ::std::option::Option<CryptoHash>,
        pub gas_price: ::std::string::String,
        pub hash: CryptoHash,
        pub height: u64,
        pub last_ds_final_block: CryptoHash,
        pub last_final_block: CryptoHash,
        pub latest_protocol_version: u32,
        pub next_bp_hash: CryptoHash,
        pub next_epoch_id: CryptoHash,
        pub outcome_root: CryptoHash,
        pub prev_hash: CryptoHash,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub prev_height: ::std::option::Option<u64>,
        pub prev_state_root: CryptoHash,
        pub random_value: CryptoHash,
        ///TODO(2271): deprecated.
        pub rent_paid: ::std::string::String,
        pub signature: Signature,
        ///Legacy json number. Should not be used.
        pub timestamp: u64,
        pub timestamp_nanosec: ::std::string::String,
        pub total_supply: ::std::string::String,
        pub validator_proposals: ::std::vec::Vec<ValidatorStakeView>,
        ///TODO(2271): deprecated.
        pub validator_reward: ::std::string::String,
    }

    impl ::std::convert::From<&BlockHeaderView> for BlockHeaderView {
        fn from(value: &BlockHeaderView) -> Self {
            value.clone()
        }
    }

    ///BlockId
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "anyOf": [
    ///    {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum BlockId {
        Variant0(u64),
        Variant1(CryptoHash),
    }

    impl ::std::convert::From<&Self> for BlockId {
        fn from(value: &BlockId) -> Self {
            value.clone()
        }
    }

    impl ::std::str::FromStr for BlockId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if let Ok(v) = value.parse() {
                Ok(Self::Variant0(v))
            } else if let Ok(v) = value.parse() {
                Ok(Self::Variant1(v))
            } else {
                Err("string conversion failed for all variants".into())
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for BlockId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for BlockId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for BlockId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::fmt::Display for BlockId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                Self::Variant0(x) => x.fmt(f),
                Self::Variant1(x) => x.fmt(f),
            }
        }
    }

    impl ::std::convert::From<u64> for BlockId {
        fn from(value: u64) -> Self {
            Self::Variant0(value)
        }
    }

    impl ::std::convert::From<CryptoHash> for BlockId {
        fn from(value: CryptoHash) -> Self {
            Self::Variant1(value)
        }
    }

    ///BlockMethodNameHelperEnum
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "block"
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
    pub enum BlockMethodNameHelperEnum {
        #[serde(rename = "block")]
        Block,
    }

    impl ::std::convert::From<&Self> for BlockMethodNameHelperEnum {
        fn from(value: &BlockMethodNameHelperEnum) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for BlockMethodNameHelperEnum {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Block => write!(f, "block"),
            }
        }
    }

    impl ::std::str::FromStr for BlockMethodNameHelperEnum {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "block" => Ok(Self::Block),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for BlockMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for BlockMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for BlockMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///BlockStatusView
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "hash",
    ///    "height"
    ///  ],
    ///  "properties": {
    ///    "hash": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "height": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct BlockStatusView {
        pub hash: CryptoHash,
        pub height: u64,
    }

    impl ::std::convert::From<&BlockStatusView> for BlockStatusView {
        fn from(value: &BlockStatusView) -> Self {
            value.clone()
        }
    }

    ///BroadCastTxAsyncMethodNameHelperEnum
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "broadcast_tx_async"
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
    pub enum BroadCastTxAsyncMethodNameHelperEnum {
        #[serde(rename = "broadcast_tx_async")]
        BroadcastTxAsync,
    }

    impl ::std::convert::From<&Self> for BroadCastTxAsyncMethodNameHelperEnum {
        fn from(value: &BroadCastTxAsyncMethodNameHelperEnum) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for BroadCastTxAsyncMethodNameHelperEnum {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::BroadcastTxAsync => write!(f, "broadcast_tx_async"),
            }
        }
    }

    impl ::std::str::FromStr for BroadCastTxAsyncMethodNameHelperEnum {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "broadcast_tx_async" => Ok(Self::BroadcastTxAsync),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for BroadCastTxAsyncMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for BroadCastTxAsyncMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for BroadCastTxAsyncMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///BroadCastTxCommitMethodNameHelperEnum
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "broadcast_tx_commit"
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
    pub enum BroadCastTxCommitMethodNameHelperEnum {
        #[serde(rename = "broadcast_tx_commit")]
        BroadcastTxCommit,
    }

    impl ::std::convert::From<&Self> for BroadCastTxCommitMethodNameHelperEnum {
        fn from(value: &BroadCastTxCommitMethodNameHelperEnum) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for BroadCastTxCommitMethodNameHelperEnum {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::BroadcastTxCommit => write!(f, "broadcast_tx_commit"),
            }
        }
    }

    impl ::std::str::FromStr for BroadCastTxCommitMethodNameHelperEnum {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "broadcast_tx_commit" => Ok(Self::BroadcastTxCommit),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for BroadCastTxCommitMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for BroadCastTxCommitMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for BroadCastTxCommitMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///CatchupStatusView
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "blocks_to_catchup",
    ///    "shard_sync_status",
    ///    "sync_block_hash",
    ///    "sync_block_height"
    ///  ],
    ///  "properties": {
    ///    "blocks_to_catchup": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/BlockStatusView"
    ///      }
    ///    },
    ///    "shard_sync_status": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "sync_block_hash": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "sync_block_height": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CatchupStatusView {
        pub blocks_to_catchup: ::std::vec::Vec<BlockStatusView>,
        pub shard_sync_status:
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        pub sync_block_hash: CryptoHash,
        pub sync_block_height: u64,
    }

    impl ::std::convert::From<&CatchupStatusView> for CatchupStatusView {
        fn from(value: &CatchupStatusView) -> Self {
            value.clone()
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

    ///Config for the Chunk Distribution Network feature. This allows nodes to
    /// push and pull chunks from a central stream. The two benefits of this
    /// approach are: (1) less request/response traffic on the peer-to-peer
    /// network and (2) lower latency for RPC nodes indexing the chain.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Config for the Chunk Distribution Network feature. This
    /// allows nodes to push and pull chunks from a central stream. The two
    /// benefits of this approach are: (1) less request/response traffic on the
    /// peer-to-peer network and (2) lower latency for RPC nodes indexing the
    /// chain.",
    ///  "type": "object",
    ///  "required": [
    ///    "enabled",
    ///    "uris"
    ///  ],
    ///  "properties": {
    ///    "enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "uris": {
    ///      "$ref": "#/components/schemas/ChunkDistributionUris"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ChunkDistributionNetworkConfig {
        pub enabled: bool,
        pub uris: ChunkDistributionUris,
    }

    impl ::std::convert::From<&ChunkDistributionNetworkConfig> for ChunkDistributionNetworkConfig {
        fn from(value: &ChunkDistributionNetworkConfig) -> Self {
            value.clone()
        }
    }

    ///URIs for the Chunk Distribution Network feature.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "URIs for the Chunk Distribution Network feature.",
    ///  "type": "object",
    ///  "required": [
    ///    "get",
    ///    "set"
    ///  ],
    ///  "properties": {
    ///    "get": {
    ///      "description": "URI for pulling chunks from the stream.",
    ///      "type": "string"
    ///    },
    ///    "set": {
    ///      "description": "URI for publishing chunks to the stream.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ChunkDistributionUris {
        ///URI for pulling chunks from the stream.
        pub get: ::std::string::String,
        ///URI for publishing chunks to the stream.
        pub set: ::std::string::String,
    }

    impl ::std::convert::From<&ChunkDistributionUris> for ChunkDistributionUris {
        fn from(value: &ChunkDistributionUris) -> Self {
            value.clone()
        }
    }

    ///ChunkHeaderView
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "balance_burnt",
    ///    "chunk_hash",
    ///    "encoded_length",
    ///    "encoded_merkle_root",
    ///    "gas_limit",
    ///    "gas_used",
    ///    "height_created",
    ///    "height_included",
    ///    "outcome_root",
    ///    "outgoing_receipts_root",
    ///    "prev_block_hash",
    ///    "prev_state_root",
    ///    "rent_paid",
    ///    "shard_id",
    ///    "signature",
    ///    "tx_root",
    ///    "validator_proposals",
    ///    "validator_reward"
    ///  ],
    ///  "properties": {
    ///    "balance_burnt": {
    ///      "type": "string"
    ///    },
    ///    "bandwidth_requests": {
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/BandwidthRequests"
    ///            }
    ///          ]
    ///        }
    ///      ]
    ///    },
    ///    "chunk_hash": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "congestion_info": {
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/CongestionInfoView"
    ///            }
    ///          ]
    ///        }
    ///      ]
    ///    },
    ///    "encoded_length": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "encoded_merkle_root": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "gas_limit": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "gas_used": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "height_created": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "height_included": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "outcome_root": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "outgoing_receipts_root": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "prev_block_hash": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "prev_state_root": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "rent_paid": {
    ///      "description": "TODO(2271): deprecated.",
    ///      "type": "string"
    ///    },
    ///    "shard_id": {
    ///      "$ref": "#/components/schemas/ShardId"
    ///    },
    ///    "signature": {
    ///      "$ref": "#/components/schemas/Signature"
    ///    },
    ///    "tx_root": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "validator_proposals": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ValidatorStakeView"
    ///      }
    ///    },
    ///    "validator_reward": {
    ///      "description": "TODO(2271): deprecated.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ChunkHeaderView {
        pub balance_burnt: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub bandwidth_requests: ::std::option::Option<BandwidthRequests>,
        pub chunk_hash: CryptoHash,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub congestion_info: ::std::option::Option<CongestionInfoView>,
        pub encoded_length: u64,
        pub encoded_merkle_root: CryptoHash,
        pub gas_limit: u64,
        pub gas_used: u64,
        pub height_created: u64,
        pub height_included: u64,
        pub outcome_root: CryptoHash,
        pub outgoing_receipts_root: CryptoHash,
        pub prev_block_hash: CryptoHash,
        pub prev_state_root: CryptoHash,
        ///TODO(2271): deprecated.
        pub rent_paid: ::std::string::String,
        pub shard_id: ShardId,
        pub signature: Signature,
        pub tx_root: CryptoHash,
        pub validator_proposals: ::std::vec::Vec<ValidatorStakeView>,
        ///TODO(2271): deprecated.
        pub validator_reward: ::std::string::String,
    }

    impl ::std::convert::From<&ChunkHeaderView> for ChunkHeaderView {
        fn from(value: &ChunkHeaderView) -> Self {
            value.clone()
        }
    }

    ///ClientConfigMethodNameHelperEnum
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "client_config"
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
    pub enum ClientConfigMethodNameHelperEnum {
        #[serde(rename = "client_config")]
        ClientConfig,
    }

    impl ::std::convert::From<&Self> for ClientConfigMethodNameHelperEnum {
        fn from(value: &ClientConfigMethodNameHelperEnum) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for ClientConfigMethodNameHelperEnum {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::ClientConfig => write!(f, "client_config"),
            }
        }
    }

    impl ::std::str::FromStr for ClientConfigMethodNameHelperEnum {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "client_config" => Ok(Self::ClientConfig),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for ClientConfigMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for ClientConfigMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ClientConfigMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///CompilationError
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "CodeDoesNotExist"
    ///      ],
    ///      "properties": {
    ///        "CodeDoesNotExist": {
    ///          "type": "object",
    ///          "required": [
    ///            "account_id"
    ///          ],
    ///          "properties": {
    ///            "account_id": {
    ///              "$ref": "#/components/schemas/AccountId"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "PrepareError"
    ///      ],
    ///      "properties": {
    ///        "PrepareError": {
    ///          "$ref": "#/components/schemas/PrepareError"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "This is for defense in depth. We expect our
    /// runtime-independent preparation code to fully catch all invalid wasms,
    /// but, if it ever misses something we’ll emit this error",
    ///      "type": "object",
    ///      "required": [
    ///        "WasmerCompileError"
    ///      ],
    ///      "properties": {
    ///        "WasmerCompileError": {
    ///          "type": "object",
    ///          "required": [
    ///            "msg"
    ///          ],
    ///          "properties": {
    ///            "msg": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub enum CompilationError {
        CodeDoesNotExist {
            account_id: AccountId,
        },
        PrepareError(PrepareError),
        ///This is for defense in depth. We expect our runtime-independent
        /// preparation code to fully catch all invalid wasms, but, if it ever
        /// misses something we’ll emit this error
        WasmerCompileError {
            msg: ::std::string::String,
        },
    }

    impl ::std::convert::From<&Self> for CompilationError {
        fn from(value: &CompilationError) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<PrepareError> for CompilationError {
        fn from(value: PrepareError) -> Self {
            Self::PrepareError(value)
        }
    }

    ///CongestionControlConfigView
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "allowed_shard_outgoing_gas",
    ///    "max_congestion_incoming_gas",
    ///    "max_congestion_memory_consumption",
    ///    "max_congestion_missed_chunks",
    ///    "max_congestion_outgoing_gas",
    ///    "max_outgoing_gas",
    ///    "max_tx_gas",
    ///    "min_outgoing_gas",
    ///    "min_tx_gas",
    ///    "outgoing_receipts_big_size_limit",
    ///    "outgoing_receipts_usual_size_limit",
    ///    "reject_tx_congestion_threshold"
    ///  ],
    ///  "properties": {
    ///    "allowed_shard_outgoing_gas": {
    ///      "description": "How much gas the chosen allowed shard can send to a
    /// 100% congested shard.\n\nSee [`CongestionControlConfig`] for more
    /// details.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "max_congestion_incoming_gas": {
    ///      "description": "How much gas in delayed receipts of a shard is 100%
    /// incoming congestion.\n\nSee [`CongestionControlConfig`] for more
    /// details.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "max_congestion_memory_consumption": {
    ///      "description": "How much memory space of all delayed and buffered
    /// receipts in a shard is considered 100% congested.\n\nSee
    /// [`CongestionControlConfig`] for more details.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "max_congestion_missed_chunks": {
    ///      "description": "How many missed chunks in a row in a shard is
    /// considered 100% congested.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "max_congestion_outgoing_gas": {
    ///      "description": "How much gas in outgoing buffered receipts of a
    /// shard is 100% congested.\n\nOutgoing congestion contributes to overall
    /// congestion, which reduces how much other shards are allowed to forward
    /// to this shard.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "max_outgoing_gas": {
    ///      "description": "The maximum amount of gas attached to receipts a
    /// shard can forward to another shard per chunk.\n\nSee
    /// [`CongestionControlConfig`] for more details.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "max_tx_gas": {
    ///      "description": "The maximum amount of gas in a chunk spent on
    /// converting new transactions to receipts.\n\nSee
    /// [`CongestionControlConfig`] for more details.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "min_outgoing_gas": {
    ///      "description": "The minimum gas each shard can send to a shard that
    /// is not fully congested.\n\nSee [`CongestionControlConfig`] for more
    /// details.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "min_tx_gas": {
    ///      "description": "The minimum amount of gas in a chunk spent on converting new transactions to receipts, as long as the receiving shard is not congested.\n\nSee [`CongestionControlConfig`] for more details.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "outgoing_receipts_big_size_limit": {
    ///      "description": "Large size limit for outgoing receipts to a shard,
    /// used when it's safe to send a lot of receipts without making the state
    /// witness too large. It limits the total sum of outgoing receipts, not
    /// individual receipts.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "outgoing_receipts_usual_size_limit": {
    ///      "description": "The standard size limit for outgoing receipts aimed
    /// at a single shard. This limit is pretty small to keep the size of
    /// source_receipt_proofs under control. It limits the total sum of outgoing
    /// receipts, not individual receipts.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "reject_tx_congestion_threshold": {
    ///      "description": "How much congestion a shard can tolerate before it
    /// stops all shards from accepting new transactions with the receiver set
    /// to the congested shard.",
    ///      "type": "number",
    ///      "format": "double"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CongestionControlConfigView {
        ///How much gas the chosen allowed shard can send to a 100% congested
        /// shard.
        ///
        ///See [`CongestionControlConfig`] for more details.
        pub allowed_shard_outgoing_gas: u64,
        ///How much gas in delayed receipts of a shard is 100% incoming
        /// congestion.
        ///
        ///See [`CongestionControlConfig`] for more details.
        pub max_congestion_incoming_gas: u64,
        ///How much memory space of all delayed and buffered receipts in a
        /// shard is considered 100% congested.
        ///
        ///See [`CongestionControlConfig`] for more details.
        pub max_congestion_memory_consumption: u64,
        ///How many missed chunks in a row in a shard is considered 100%
        /// congested.
        pub max_congestion_missed_chunks: u64,
        ///How much gas in outgoing buffered receipts of a shard is 100%
        /// congested.
        ///
        ///Outgoing congestion contributes to overall congestion, which reduces
        /// how much other shards are allowed to forward to this shard.
        pub max_congestion_outgoing_gas: u64,
        ///The maximum amount of gas attached to receipts a shard can forward
        /// to another shard per chunk.
        ///
        ///See [`CongestionControlConfig`] for more details.
        pub max_outgoing_gas: u64,
        ///The maximum amount of gas in a chunk spent on converting new
        /// transactions to receipts.
        ///
        ///See [`CongestionControlConfig`] for more details.
        pub max_tx_gas: u64,
        ///The minimum gas each shard can send to a shard that is not fully
        /// congested.
        ///
        ///See [`CongestionControlConfig`] for more details.
        pub min_outgoing_gas: u64,
        ///The minimum amount of gas in a chunk spent on converting new
        /// transactions to receipts, as long as the receiving shard is not
        /// congested.
        ///
        ///See [`CongestionControlConfig`] for more details.
        pub min_tx_gas: u64,
        ///Large size limit for outgoing receipts to a shard, used when it's
        /// safe to send a lot of receipts without making the state witness too
        /// large. It limits the total sum of outgoing receipts, not individual
        /// receipts.
        pub outgoing_receipts_big_size_limit: u64,
        ///The standard size limit for outgoing receipts aimed at a single
        /// shard. This limit is pretty small to keep the size of
        /// source_receipt_proofs under control. It limits the total sum of
        /// outgoing receipts, not individual receipts.
        pub outgoing_receipts_usual_size_limit: u64,
        pub reject_tx_congestion_threshold: f64,
    }

    impl ::std::convert::From<&CongestionControlConfigView> for CongestionControlConfigView {
        fn from(value: &CongestionControlConfigView) -> Self {
            value.clone()
        }
    }

    ///CongestionInfoView
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "allowed_shard",
    ///    "buffered_receipts_gas",
    ///    "delayed_receipts_gas",
    ///    "receipt_bytes"
    ///  ],
    ///  "properties": {
    ///    "allowed_shard": {
    ///      "type": "integer",
    ///      "format": "uint16",
    ///      "minimum": 0.0
    ///    },
    ///    "buffered_receipts_gas": {
    ///      "type": "string"
    ///    },
    ///    "delayed_receipts_gas": {
    ///      "type": "string"
    ///    },
    ///    "receipt_bytes": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CongestionInfoView {
        pub allowed_shard: u16,
        pub buffered_receipts_gas: ::std::string::String,
        pub delayed_receipts_gas: ::std::string::String,
        pub receipt_bytes: u64,
    }

    impl ::std::convert::From<&CongestionInfoView> for CongestionInfoView {
        fn from(value: &CongestionInfoView) -> Self {
            value.clone()
        }
    }

    ///CostGasUsed
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "cost",
    ///    "cost_category",
    ///    "gas_used"
    ///  ],
    ///  "properties": {
    ///    "cost": {
    ///      "type": "string"
    ///    },
    ///    "cost_category": {
    ///      "type": "string"
    ///    },
    ///    "gas_used": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CostGasUsed {
        pub cost: ::std::string::String,
        pub cost_category: ::std::string::String,
        pub gas_used: ::std::string::String,
    }

    impl ::std::convert::From<&CostGasUsed> for CostGasUsed {
        fn from(value: &CostGasUsed) -> Self {
            value.clone()
        }
    }

    ///Create account action
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Create account action",
    ///  "type": "object"
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct CreateAccountAction(
        pub ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    );
    impl ::std::ops::Deref for CreateAccountAction {
        type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
        fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
            &self.0
        }
    }

    impl ::std::convert::From<CreateAccountAction>
        for ::serde_json::Map<::std::string::String, ::serde_json::Value>
    {
        fn from(value: CreateAccountAction) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&CreateAccountAction> for CreateAccountAction {
        fn from(value: &CreateAccountAction) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
        for CreateAccountAction
    {
        fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
            Self(value)
        }
    }

    ///CryptoHash
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
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
    pub struct CryptoHash(pub ::std::string::String);
    impl ::std::ops::Deref for CryptoHash {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<CryptoHash> for ::std::string::String {
        fn from(value: CryptoHash) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&CryptoHash> for CryptoHash {
        fn from(value: &CryptoHash) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<::std::string::String> for CryptoHash {
        fn from(value: ::std::string::String) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for CryptoHash {
        type Err = ::std::convert::Infallible;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::fmt::Display for CryptoHash {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    ///CurrentEpochValidatorInfo
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "account_id",
    ///    "is_slashed",
    ///    "num_expected_blocks",
    ///    "num_produced_blocks",
    ///    "public_key",
    ///    "shards",
    ///    "shards_endorsed",
    ///    "stake"
    ///  ],
    ///  "properties": {
    ///    "account_id": {
    ///      "$ref": "#/components/schemas/AccountId"
    ///    },
    ///    "is_slashed": {
    ///      "type": "boolean"
    ///    },
    ///    "num_expected_blocks": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "num_expected_chunks": {
    ///      "default": 0,
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "num_expected_chunks_per_shard": {
    ///      "description": "Number of chunks this validator was expected to
    /// produce in each shard. Each entry in the array corresponds to the shard
    /// in the `shards_produced` array.",
    ///      "default": [],
    ///      "type": "array",
    ///      "items": {
    ///        "type": "integer",
    ///        "format": "uint64",
    ///        "minimum": 0.0
    ///      }
    ///    },
    ///    "num_expected_endorsements": {
    ///      "default": 0,
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "num_expected_endorsements_per_shard": {
    ///      "description": "Number of chunks this validator was expected to
    /// validate and endorse in each shard. Each entry in the array corresponds
    /// to the shard in the `shards_endorsed` array.",
    ///      "default": [],
    ///      "type": "array",
    ///      "items": {
    ///        "type": "integer",
    ///        "format": "uint64",
    ///        "minimum": 0.0
    ///      }
    ///    },
    ///    "num_produced_blocks": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "num_produced_chunks": {
    ///      "default": 0,
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "num_produced_chunks_per_shard": {
    ///      "default": [],
    ///      "type": "array",
    ///      "items": {
    ///        "type": "integer",
    ///        "format": "uint64",
    ///        "minimum": 0.0
    ///      }
    ///    },
    ///    "num_produced_endorsements": {
    ///      "default": 0,
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "num_produced_endorsements_per_shard": {
    ///      "default": [],
    ///      "type": "array",
    ///      "items": {
    ///        "type": "integer",
    ///        "format": "uint64",
    ///        "minimum": 0.0
    ///      }
    ///    },
    ///    "public_key": {
    ///      "$ref": "#/components/schemas/PublicKey"
    ///    },
    ///    "shards": {
    ///      "description": "Shards this validator is assigned to as chunk
    /// producer in the current epoch.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ShardId"
    ///      }
    ///    },
    ///    "shards_endorsed": {
    ///      "description": "Shards this validator is assigned to as chunk
    /// validator in the current epoch.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ShardId"
    ///      }
    ///    },
    ///    "stake": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CurrentEpochValidatorInfo {
        pub account_id: AccountId,
        pub is_slashed: bool,
        pub num_expected_blocks: u64,
        #[serde(default)]
        pub num_expected_chunks: u64,
        ///Number of chunks this validator was expected to produce in each
        /// shard. Each entry in the array corresponds to the shard in the
        /// `shards_produced` array.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub num_expected_chunks_per_shard: ::std::vec::Vec<u64>,
        #[serde(default)]
        pub num_expected_endorsements: u64,
        ///Number of chunks this validator was expected to validate and endorse
        /// in each shard. Each entry in the array corresponds to the shard in
        /// the `shards_endorsed` array.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub num_expected_endorsements_per_shard: ::std::vec::Vec<u64>,
        pub num_produced_blocks: u64,
        #[serde(default)]
        pub num_produced_chunks: u64,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub num_produced_chunks_per_shard: ::std::vec::Vec<u64>,
        #[serde(default)]
        pub num_produced_endorsements: u64,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub num_produced_endorsements_per_shard: ::std::vec::Vec<u64>,
        pub public_key: PublicKey,
        ///Shards this validator is assigned to as chunk producer in the
        /// current epoch.
        pub shards: ::std::vec::Vec<ShardId>,
        ///Shards this validator is assigned to as chunk validator in the
        /// current epoch.
        pub shards_endorsed: ::std::vec::Vec<ShardId>,
        pub stake: ::std::string::String,
    }

    impl ::std::convert::From<&CurrentEpochValidatorInfo> for CurrentEpochValidatorInfo {
        fn from(value: &CurrentEpochValidatorInfo) -> Self {
            value.clone()
        }
    }

    ///DataReceiptCreationConfigView
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "base_cost",
    ///    "cost_per_byte"
    ///  ],
    ///  "properties": {
    ///    "base_cost": {
    ///      "description": "Base cost of creating a data receipt. Both `send`
    /// and `exec` costs are burned when a new receipt has input dependencies.
    /// The gas is charged for each input dependency. The dependencies are
    /// specified when a receipt is created using `promise_then` and
    /// `promise_batch_then`. NOTE: Any receipt with output dependencies will
    /// produce data receipts. Even if it fails. Even if the last action is not
    /// a function call (in case of success it will return empty value).",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Fee"
    ///        }
    ///      ]
    ///    },
    ///    "cost_per_byte": {
    ///      "description": "Additional cost per byte sent. Both `send` and
    /// `exec` costs are burned when a function call finishes execution and
    /// returns `N` bytes of data to every output dependency. For each output
    /// dependency the cost is `(send(sir) + exec()) * N`.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Fee"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DataReceiptCreationConfigView {
        ///Base cost of creating a data receipt. Both `send` and `exec` costs
        /// are burned when a new receipt has input dependencies. The gas is
        /// charged for each input dependency. The dependencies are specified
        /// when a receipt is created using `promise_then` and
        /// `promise_batch_then`. NOTE: Any receipt with output dependencies
        /// will produce data receipts. Even if it fails. Even if the last
        /// action is not a function call (in case of success it will return
        /// empty value).
        pub base_cost: Fee,
        ///Additional cost per byte sent. Both `send` and `exec` costs are
        /// burned when a function call finishes execution and returns `N` bytes
        /// of data to every output dependency. For each output dependency the
        /// cost is `(send(sir) + exec()) * N`.
        pub cost_per_byte: Fee,
    }

    impl ::std::convert::From<&DataReceiptCreationConfigView> for DataReceiptCreationConfigView {
        fn from(value: &DataReceiptCreationConfigView) -> Self {
            value.clone()
        }
    }

    ///DataReceiverView
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "data_id",
    ///    "receiver_id"
    ///  ],
    ///  "properties": {
    ///    "data_id": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "receiver_id": {
    ///      "$ref": "#/components/schemas/AccountId"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DataReceiverView {
        pub data_id: CryptoHash,
        pub receiver_id: AccountId,
    }

    impl ::std::convert::From<&DataReceiverView> for DataReceiverView {
        fn from(value: &DataReceiverView) -> Self {
            value.clone()
        }
    }

    ///This action allows to execute the inner actions behalf of the defined
    /// sender.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "This action allows to execute the inner actions behalf
    /// of the defined sender.",
    ///  "type": "object",
    ///  "required": [
    ///    "actions",
    ///    "max_block_height",
    ///    "nonce",
    ///    "public_key",
    ///    "receiver_id",
    ///    "sender_id"
    ///  ],
    ///  "properties": {
    ///    "actions": {
    ///      "description": "List of actions to be executed.\n\nWith the meta
    /// transactions MVP defined in NEP-366, nested DelegateActions are not
    /// allowed. A separate type is used to enforce it.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/NonDelegateAction"
    ///      }
    ///    },
    ///    "max_block_height": {
    ///      "description": "The maximal height of the block in the blockchain
    /// below which the given DelegateAction is valid.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "nonce": {
    ///      "description": "Nonce to ensure that the same delegate action is
    /// not sent twice by a relayer and should match for given account's
    /// `public_key`. After this action is processed it will increment.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "public_key": {
    ///      "description": "Public key used to sign this delegated action.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/PublicKey"
    ///        }
    ///      ]
    ///    },
    ///    "receiver_id": {
    ///      "description": "Receiver of the delegated actions.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/AccountId"
    ///        }
    ///      ]
    ///    },
    ///    "sender_id": {
    ///      "description": "Signer of the delegated actions",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/AccountId"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DelegateAction {
        ///List of actions to be executed.
        ///
        ///With the meta transactions MVP defined in NEP-366, nested
        /// DelegateActions are not allowed. A separate type is used to enforce
        /// it.
        pub actions: ::std::vec::Vec<NonDelegateAction>,
        ///The maximal height of the block in the blockchain below which the
        /// given DelegateAction is valid.
        pub max_block_height: u64,
        ///Nonce to ensure that the same delegate action is not sent twice by a
        /// relayer and should match for given account's `public_key`. After
        /// this action is processed it will increment.
        pub nonce: u64,
        ///Public key used to sign this delegated action.
        pub public_key: PublicKey,
        ///Receiver of the delegated actions.
        pub receiver_id: AccountId,
        ///Signer of the delegated actions
        pub sender_id: AccountId,
    }

    impl ::std::convert::From<&DelegateAction> for DelegateAction {
        fn from(value: &DelegateAction) -> Self {
            value.clone()
        }
    }

    ///DeleteAccountAction
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "beneficiary_id"
    ///  ],
    ///  "properties": {
    ///    "beneficiary_id": {
    ///      "$ref": "#/components/schemas/AccountId"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DeleteAccountAction {
        pub beneficiary_id: AccountId,
    }

    impl ::std::convert::From<&DeleteAccountAction> for DeleteAccountAction {
        fn from(value: &DeleteAccountAction) -> Self {
            value.clone()
        }
    }

    ///DeleteKeyAction
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "public_key"
    ///  ],
    ///  "properties": {
    ///    "public_key": {
    ///      "description": "A public key associated with the access_key to be
    /// deleted.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/PublicKey"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DeleteKeyAction {
        ///A public key associated with the access_key to be deleted.
        pub public_key: PublicKey,
    }

    impl ::std::convert::From<&DeleteKeyAction> for DeleteKeyAction {
        fn from(value: &DeleteKeyAction) -> Self {
            value.clone()
        }
    }

    ///Deploy contract action
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Deploy contract action",
    ///  "type": "object",
    ///  "required": [
    ///    "code"
    ///  ],
    ///  "properties": {
    ///    "code": {
    ///      "description": "WebAssembly binary",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DeployContractAction {
        ///WebAssembly binary
        pub code: ::std::string::String,
    }

    impl ::std::convert::From<&DeployContractAction> for DeployContractAction {
        fn from(value: &DeployContractAction) -> Self {
            value.clone()
        }
    }

    ///Deploy global contract action
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Deploy global contract action",
    ///  "type": "object",
    ///  "required": [
    ///    "code",
    ///    "deploy_mode"
    ///  ],
    ///  "properties": {
    ///    "code": {
    ///      "description": "WebAssembly binary",
    ///      "type": "string"
    ///    },
    ///    "deploy_mode": {
    ///      "$ref": "#/components/schemas/GlobalContractDeployMode"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DeployGlobalContractAction {
        ///WebAssembly binary
        pub code: ::std::string::String,
        pub deploy_mode: GlobalContractDeployMode,
    }

    impl ::std::convert::From<&DeployGlobalContractAction> for DeployGlobalContractAction {
        fn from(value: &DeployGlobalContractAction) -> Self {
            value.clone()
        }
    }

    ///DetailedDebugStatus
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "block_production_delay_millis",
    ///    "catchup_status",
    ///    "current_head_status",
    ///    "current_header_head_status",
    ///    "network_info",
    ///    "sync_status"
    ///  ],
    ///  "properties": {
    ///    "block_production_delay_millis": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "catchup_status": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/CatchupStatusView"
    ///      }
    ///    },
    ///    "current_head_status": {
    ///      "$ref": "#/components/schemas/BlockStatusView"
    ///    },
    ///    "current_header_head_status": {
    ///      "$ref": "#/components/schemas/BlockStatusView"
    ///    },
    ///    "network_info": {
    ///      "$ref": "#/components/schemas/NetworkInfoView"
    ///    },
    ///    "sync_status": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DetailedDebugStatus {
        pub block_production_delay_millis: u64,
        pub catchup_status: ::std::vec::Vec<CatchupStatusView>,
        pub current_head_status: BlockStatusView,
        pub current_header_head_status: BlockStatusView,
        pub network_info: NetworkInfoView,
        pub sync_status: ::std::string::String,
    }

    impl ::std::convert::From<&DetailedDebugStatus> for DetailedDebugStatus {
        fn from(value: &DetailedDebugStatus) -> Self {
            value.clone()
        }
    }

    ///Direction
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "Left",
    ///    "Right"
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
    pub enum Direction {
        Left,
        Right,
    }

    impl ::std::convert::From<&Self> for Direction {
        fn from(value: &Direction) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for Direction {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Left => write!(f, "Left"),
                Self::Right => write!(f, "Right"),
            }
        }
    }

    impl ::std::str::FromStr for Direction {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "Left" => Ok(Self::Left),
                "Right" => Ok(Self::Right),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for Direction {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for Direction {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for Direction {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///Configures how to dump state to external storage.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Configures how to dump state to external storage.",
    ///  "type": "object",
    ///  "required": [
    ///    "location"
    ///  ],
    ///  "properties": {
    ///    "credentials_file": {
    ///      "description": "Location of a json file with credentials allowing
    /// write access to the bucket.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "iteration_delay": {
    ///      "description": "How often to check if a new epoch has started. Feel
    /// free to set to `None`, defaults are sensible.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/DurationSchemeProvider"
    ///        }
    ///      ]
    ///    },
    ///    "location": {
    ///      "description": "Specifies where to write the obtained state
    /// parts.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/ExternalStorageLocation"
    ///        }
    ///      ]
    ///    },
    ///    "restart_dump_for_shards": {
    ///      "description": "Use in case a node that dumps state to the external
    /// storage gets in trouble.",
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "$ref": "#/components/schemas/ShardId"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DumpConfig {
        ///Location of a json file with credentials allowing write access to
        /// the bucket.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub credentials_file: ::std::option::Option<::std::string::String>,
        ///How often to check if a new epoch has started. Feel free to set to
        /// `None`, defaults are sensible.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub iteration_delay: ::std::option::Option<DurationSchemeProvider>,
        ///Specifies where to write the obtained state parts.
        pub location: ExternalStorageLocation,
        ///Use in case a node that dumps state to the external storage gets in
        /// trouble.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub restart_dump_for_shards: ::std::option::Option<::std::vec::Vec<ShardId>>,
    }

    impl ::std::convert::From<&DumpConfig> for DumpConfig {
        fn from(value: &DumpConfig) -> Self {
            value.clone()
        }
    }

    ///DurationSchemeProvider
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "nanoseconds",
    ///    "seconds"
    ///  ],
    ///  "properties": {
    ///    "nanoseconds": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "seconds": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DurationSchemeProvider {
        pub nanoseconds: i32,
        pub seconds: i64,
    }

    impl ::std::convert::From<&DurationSchemeProvider> for DurationSchemeProvider {
        fn from(value: &DurationSchemeProvider) -> Self {
            value.clone()
        }
    }

    ///Epoch identifier -- wrapped hash, to make it easier to distinguish.
    /// EpochId of epoch T is the hash of last block in T-2 EpochId of first two
    /// epochs is 0
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Epoch identifier -- wrapped hash, to make it easier to
    /// distinguish. EpochId of epoch T is the hash of last block in T-2 EpochId
    /// of first two epochs is 0",
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct EpochId(pub CryptoHash);
    impl ::std::ops::Deref for EpochId {
        type Target = CryptoHash;
        fn deref(&self) -> &CryptoHash {
            &self.0
        }
    }

    impl ::std::convert::From<EpochId> for CryptoHash {
        fn from(value: EpochId) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&EpochId> for EpochId {
        fn from(value: &EpochId) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<CryptoHash> for EpochId {
        fn from(value: CryptoHash) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for EpochId {
        type Err = <CryptoHash as ::std::str::FromStr>::Err;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl ::std::convert::TryFrom<&str> for EpochId {
        type Error = <CryptoHash as ::std::str::FromStr>::Err;
        fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&String> for EpochId {
        type Error = <CryptoHash as ::std::str::FromStr>::Err;
        fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for EpochId {
        type Error = <CryptoHash as ::std::str::FromStr>::Err;
        fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::fmt::Display for EpochId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    ///EpochSyncConfig
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "epoch_sync_horizon",
    ///    "timeout_for_epoch_sync"
    ///  ],
    ///  "properties": {
    ///    "disable_epoch_sync_for_bootstrapping": {
    ///      "description": "If true, even if the node started from genesis, it
    /// will not perform epoch sync. There should be no reason to set this flag
    /// in production, because on both mainnet and testnet it would be
    /// infeasible to catch up from genesis without epoch sync.",
    ///      "default": false,
    ///      "type": "boolean"
    ///    },
    ///    "epoch_sync_horizon": {
    ///      "description": "This serves as two purposes: (1) the node will not
    /// epoch sync and instead resort to header sync, if the genesis block is
    /// within this many blocks from the current block; (2) the node will reject
    /// an epoch sync proof if the provided proof is for an epoch that is more
    /// than this many blocks behind the current block.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "ignore_epoch_sync_network_requests": {
    ///      "description": "If true, the node will ignore epoch sync requests
    /// from the network. It is strongly recommended not to set this flag,
    /// because it will prevent other nodes from bootstrapping. This flag is
    /// only included as a kill-switch and may be removed in a future release.
    /// Please note that epoch sync requests are heavily rate limited and
    /// cached, and therefore should not affect the performance of the node or
    /// introduce any non-negligible increase in network traffic.",
    ///      "default": false,
    ///      "type": "boolean"
    ///    },
    ///    "timeout_for_epoch_sync": {
    ///      "description": "Timeout for epoch sync requests. The node will
    /// continue retrying indefinitely even if this timeout is exceeded.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/DurationSchemeProvider"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct EpochSyncConfig {
        ///If true, even if the node started from genesis, it will not perform
        /// epoch sync. There should be no reason to set this flag in
        /// production, because on both mainnet and testnet it would be
        /// infeasible to catch up from genesis without epoch sync.
        #[serde(default)]
        pub disable_epoch_sync_for_bootstrapping: bool,
        ///This serves as two purposes: (1) the node will not epoch sync and
        /// instead resort to header sync, if the genesis block is within this
        /// many blocks from the current block; (2) the node will reject an
        /// epoch sync proof if the provided proof is for an epoch that is more
        /// than this many blocks behind the current block.
        pub epoch_sync_horizon: u64,
        ///If true, the node will ignore epoch sync requests from the network.
        /// It is strongly recommended not to set this flag, because it will
        /// prevent other nodes from bootstrapping. This flag is only included
        /// as a kill-switch and may be removed in a future release. Please note
        /// that epoch sync requests are heavily rate limited and cached, and
        /// therefore should not affect the performance of the node or introduce
        /// any non-negligible increase in network traffic.
        #[serde(default)]
        pub ignore_epoch_sync_network_requests: bool,
        ///Timeout for epoch sync requests. The node will continue retrying
        /// indefinitely even if this timeout is exceeded.
        pub timeout_for_epoch_sync: DurationSchemeProvider,
    }

    impl ::std::convert::From<&EpochSyncConfig> for EpochSyncConfig {
        fn from(value: &EpochSyncConfig) -> Self {
            value.clone()
        }
    }

    ///ExecutionMetadataView
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "version"
    ///  ],
    ///  "properties": {
    ///    "gas_profile": {
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "$ref": "#/components/schemas/CostGasUsed"
    ///      }
    ///    },
    ///    "version": {
    ///      "type": "integer",
    ///      "format": "uint32",
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ExecutionMetadataView {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub gas_profile: ::std::option::Option<::std::vec::Vec<CostGasUsed>>,
        pub version: u32,
    }

    impl ::std::convert::From<&ExecutionMetadataView> for ExecutionMetadataView {
        fn from(value: &ExecutionMetadataView) -> Self {
            value.clone()
        }
    }

    ///ExecutionOutcomeView
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "executor_id",
    ///    "gas_burnt",
    ///    "logs",
    ///    "receipt_ids",
    ///    "status",
    ///    "tokens_burnt"
    ///  ],
    ///  "properties": {
    ///    "executor_id": {
    ///      "description": "The id of the account on which the execution
    /// happens. For transaction this is signer_id, for receipt this is
    /// receiver_id.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/AccountId"
    ///        }
    ///      ]
    ///    },
    ///    "gas_burnt": {
    ///      "description": "The amount of the gas burnt by the given
    /// transaction or receipt.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "logs": {
    ///      "description": "Logs from this transaction or receipt.",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "metadata": {
    ///      "description": "Execution metadata, versioned",
    ///      "default": {
    ///        "gas_profile": null,
    ///        "version": 1
    ///      },
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/ExecutionMetadataView"
    ///        }
    ///      ]
    ///    },
    ///    "receipt_ids": {
    ///      "description": "Receipt IDs generated by this transaction or
    /// receipt.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/CryptoHash"
    ///      }
    ///    },
    ///    "status": {
    ///      "description": "Execution status. Contains the result in case of
    /// successful execution.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/ExecutionStatusView"
    ///        }
    ///      ]
    ///    },
    ///    "tokens_burnt": {
    ///      "description": "The amount of tokens burnt corresponding to the
    /// burnt gas amount. This value doesn't always equal to the `gas_burnt`
    /// multiplied by the gas price, because the prepaid gas price might be
    /// lower than the actual gas price and it creates a deficit.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ExecutionOutcomeView {
        ///The id of the account on which the execution happens. For
        /// transaction this is signer_id, for receipt this is receiver_id.
        pub executor_id: AccountId,
        ///The amount of the gas burnt by the given transaction or receipt.
        pub gas_burnt: u64,
        ///Logs from this transaction or receipt.
        pub logs: ::std::vec::Vec<::std::string::String>,
        ///Execution metadata, versioned
        #[serde(default = "defaults::execution_outcome_view_metadata")]
        pub metadata: ExecutionMetadataView,
        ///Receipt IDs generated by this transaction or receipt.
        pub receipt_ids: ::std::vec::Vec<CryptoHash>,
        ///Execution status. Contains the result in case of successful
        /// execution.
        pub status: ExecutionStatusView,
        ///The amount of tokens burnt corresponding to the burnt gas amount.
        /// This value doesn't always equal to the `gas_burnt` multiplied by the
        /// gas price, because the prepaid gas price might be lower than the
        /// actual gas price and it creates a deficit.
        pub tokens_burnt: ::std::string::String,
    }

    impl ::std::convert::From<&ExecutionOutcomeView> for ExecutionOutcomeView {
        fn from(value: &ExecutionOutcomeView) -> Self {
            value.clone()
        }
    }

    ///ExecutionOutcomeWithIdView
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "block_hash",
    ///    "id",
    ///    "outcome",
    ///    "proof"
    ///  ],
    ///  "properties": {
    ///    "block_hash": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "id": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "outcome": {
    ///      "$ref": "#/components/schemas/ExecutionOutcomeView"
    ///    },
    ///    "proof": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/MerklePathItem"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ExecutionOutcomeWithIdView {
        pub block_hash: CryptoHash,
        pub id: CryptoHash,
        pub outcome: ExecutionOutcomeView,
        pub proof: ::std::vec::Vec<MerklePathItem>,
    }

    impl ::std::convert::From<&ExecutionOutcomeWithIdView> for ExecutionOutcomeWithIdView {
        fn from(value: &ExecutionOutcomeWithIdView) -> Self {
            value.clone()
        }
    }

    ///ExecutionStatusView
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "description": "The execution is pending or unknown.",
    ///      "type": "string",
    ///      "enum": [
    ///        "Unknown"
    ///      ]
    ///    },
    ///    {
    ///      "description": "The execution has failed.",
    ///      "type": "object",
    ///      "required": [
    ///        "Failure"
    ///      ],
    ///      "properties": {
    ///        "Failure": {
    ///          "$ref": "#/components/schemas/TxExecutionError"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "The final action succeeded and returned some value
    /// or an empty vec encoded in base64.",
    ///      "type": "object",
    ///      "required": [
    ///        "SuccessValue"
    ///      ],
    ///      "properties": {
    ///        "SuccessValue": {
    ///          "type": "string"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "The final action of the receipt returned a promise
    /// or the signed transaction was converted to a receipt. Contains the
    /// receipt_id of the generated receipt.",
    ///      "type": "object",
    ///      "required": [
    ///        "SuccessReceiptId"
    ///      ],
    ///      "properties": {
    ///        "SuccessReceiptId": {
    ///          "$ref": "#/components/schemas/CryptoHash"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub enum ExecutionStatusView {
        ///The execution is pending or unknown.
        Unknown,
        ///The execution has failed.
        Failure(TxExecutionError),
        ///The final action succeeded and returned some value or an empty vec
        /// encoded in base64.
        SuccessValue(::std::string::String),
        ///The final action of the receipt returned a promise or the signed
        /// transaction was converted to a receipt. Contains the receipt_id of
        /// the generated receipt.
        SuccessReceiptId(CryptoHash),
    }

    impl ::std::convert::From<&Self> for ExecutionStatusView {
        fn from(value: &ExecutionStatusView) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<TxExecutionError> for ExecutionStatusView {
        fn from(value: TxExecutionError) -> Self {
            Self::Failure(value)
        }
    }

    impl ::std::convert::From<CryptoHash> for ExecutionStatusView {
        fn from(value: CryptoHash) -> Self {
            Self::SuccessReceiptId(value)
        }
    }

    ///ExpChangeMethodNameHelperEnum
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "EXPERIMENTAL_changes"
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
    pub enum ExpChangeMethodNameHelperEnum {
        #[serde(rename = "EXPERIMENTAL_changes")]
        ExperimentalChanges,
    }

    impl ::std::convert::From<&Self> for ExpChangeMethodNameHelperEnum {
        fn from(value: &ExpChangeMethodNameHelperEnum) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for ExpChangeMethodNameHelperEnum {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::ExperimentalChanges => write!(f, "EXPERIMENTAL_changes"),
            }
        }
    }

    impl ::std::str::FromStr for ExpChangeMethodNameHelperEnum {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "EXPERIMENTAL_changes" => Ok(Self::ExperimentalChanges),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for ExpChangeMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for ExpChangeMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ExpChangeMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///ExpChangesBlockMethodNameHelperEnum
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "EXPERIMENTAL_changes_in_block"
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
    pub enum ExpChangesBlockMethodNameHelperEnum {
        #[serde(rename = "EXPERIMENTAL_changes_in_block")]
        ExperimentalChangesInBlock,
    }

    impl ::std::convert::From<&Self> for ExpChangesBlockMethodNameHelperEnum {
        fn from(value: &ExpChangesBlockMethodNameHelperEnum) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for ExpChangesBlockMethodNameHelperEnum {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::ExperimentalChangesInBlock => write!(f, "EXPERIMENTAL_changes_in_block"),
            }
        }
    }

    impl ::std::str::FromStr for ExpChangesBlockMethodNameHelperEnum {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "EXPERIMENTAL_changes_in_block" => Ok(Self::ExperimentalChangesInBlock),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for ExpChangesBlockMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for ExpChangesBlockMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ExpChangesBlockMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///ExpGenesisMethodNameHelperEnum
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "EXPERIMENTAL_genesis_config"
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
    pub enum ExpGenesisMethodNameHelperEnum {
        #[serde(rename = "EXPERIMENTAL_genesis_config")]
        ExperimentalGenesisConfig,
    }

    impl ::std::convert::From<&Self> for ExpGenesisMethodNameHelperEnum {
        fn from(value: &ExpGenesisMethodNameHelperEnum) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for ExpGenesisMethodNameHelperEnum {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::ExperimentalGenesisConfig => write!(f, "EXPERIMENTAL_genesis_config"),
            }
        }
    }

    impl ::std::str::FromStr for ExpGenesisMethodNameHelperEnum {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "EXPERIMENTAL_genesis_config" => Ok(Self::ExperimentalGenesisConfig),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for ExpGenesisMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for ExpGenesisMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ExpGenesisMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///ExpGongestionMethodNameHelperEnum
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "EXPERIMENTAL_congestion_level"
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
    pub enum ExpGongestionMethodNameHelperEnum {
        #[serde(rename = "EXPERIMENTAL_congestion_level")]
        ExperimentalCongestionLevel,
    }

    impl ::std::convert::From<&Self> for ExpGongestionMethodNameHelperEnum {
        fn from(value: &ExpGongestionMethodNameHelperEnum) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for ExpGongestionMethodNameHelperEnum {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::ExperimentalCongestionLevel => write!(f, "EXPERIMENTAL_congestion_level"),
            }
        }
    }

    impl ::std::str::FromStr for ExpGongestionMethodNameHelperEnum {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "EXPERIMENTAL_congestion_level" => Ok(Self::ExperimentalCongestionLevel),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for ExpGongestionMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for ExpGongestionMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ExpGongestionMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///ExpLightClientBlockProofMethodNameHelperEnum
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "EXPERIMENTAL_light_client_block_proof"
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
    pub enum ExpLightClientBlockProofMethodNameHelperEnum {
        #[serde(rename = "EXPERIMENTAL_light_client_block_proof")]
        ExperimentalLightClientBlockProof,
    }

    impl ::std::convert::From<&Self> for ExpLightClientBlockProofMethodNameHelperEnum {
        fn from(value: &ExpLightClientBlockProofMethodNameHelperEnum) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for ExpLightClientBlockProofMethodNameHelperEnum {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::ExperimentalLightClientBlockProof => {
                    write!(f, "EXPERIMENTAL_light_client_block_proof")
                }
            }
        }
    }

    impl ::std::str::FromStr for ExpLightClientBlockProofMethodNameHelperEnum {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "EXPERIMENTAL_light_client_block_proof" => {
                    Ok(Self::ExperimentalLightClientBlockProof)
                }
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for ExpLightClientBlockProofMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for ExpLightClientBlockProofMethodNameHelperEnum
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for ExpLightClientBlockProofMethodNameHelperEnum
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///ExpLightClientProofMethodNameHelperEnum
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "EXPERIMENTAL_light_client_proof"
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
    pub enum ExpLightClientProofMethodNameHelperEnum {
        #[serde(rename = "EXPERIMENTAL_light_client_proof")]
        ExperimentalLightClientProof,
    }

    impl ::std::convert::From<&Self> for ExpLightClientProofMethodNameHelperEnum {
        fn from(value: &ExpLightClientProofMethodNameHelperEnum) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for ExpLightClientProofMethodNameHelperEnum {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::ExperimentalLightClientProof => write!(f, "EXPERIMENTAL_light_client_proof"),
            }
        }
    }

    impl ::std::str::FromStr for ExpLightClientProofMethodNameHelperEnum {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "EXPERIMENTAL_light_client_proof" => Ok(Self::ExperimentalLightClientProof),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for ExpLightClientProofMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for ExpLightClientProofMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ExpLightClientProofMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///ExpProtocolConfigMethodNameHelperEnum
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "EXPERIMENTAL_protocol_config"
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
    pub enum ExpProtocolConfigMethodNameHelperEnum {
        #[serde(rename = "EXPERIMENTAL_protocol_config")]
        ExperimentalProtocolConfig,
    }

    impl ::std::convert::From<&Self> for ExpProtocolConfigMethodNameHelperEnum {
        fn from(value: &ExpProtocolConfigMethodNameHelperEnum) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for ExpProtocolConfigMethodNameHelperEnum {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::ExperimentalProtocolConfig => write!(f, "EXPERIMENTAL_protocol_config"),
            }
        }
    }

    impl ::std::str::FromStr for ExpProtocolConfigMethodNameHelperEnum {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "EXPERIMENTAL_protocol_config" => Ok(Self::ExperimentalProtocolConfig),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for ExpProtocolConfigMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for ExpProtocolConfigMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ExpProtocolConfigMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///ExpReceiptMethodNameHelperEnum
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "EXPERIMENTAL_receipt"
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
    pub enum ExpReceiptMethodNameHelperEnum {
        #[serde(rename = "EXPERIMENTAL_receipt")]
        ExperimentalReceipt,
    }

    impl ::std::convert::From<&Self> for ExpReceiptMethodNameHelperEnum {
        fn from(value: &ExpReceiptMethodNameHelperEnum) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for ExpReceiptMethodNameHelperEnum {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::ExperimentalReceipt => write!(f, "EXPERIMENTAL_receipt"),
            }
        }
    }

    impl ::std::str::FromStr for ExpReceiptMethodNameHelperEnum {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "EXPERIMENTAL_receipt" => Ok(Self::ExperimentalReceipt),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for ExpReceiptMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for ExpReceiptMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ExpReceiptMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///ExpTxStatusMethodNameHelperEnum
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "EXPERIMENTAL_tx_status"
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
    pub enum ExpTxStatusMethodNameHelperEnum {
        #[serde(rename = "EXPERIMENTAL_tx_status")]
        ExperimentalTxStatus,
    }

    impl ::std::convert::From<&Self> for ExpTxStatusMethodNameHelperEnum {
        fn from(value: &ExpTxStatusMethodNameHelperEnum) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for ExpTxStatusMethodNameHelperEnum {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::ExperimentalTxStatus => write!(f, "EXPERIMENTAL_tx_status"),
            }
        }
    }

    impl ::std::str::FromStr for ExpTxStatusMethodNameHelperEnum {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "EXPERIMENTAL_tx_status" => Ok(Self::ExperimentalTxStatus),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for ExpTxStatusMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for ExpTxStatusMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ExpTxStatusMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///ExpValidatorsMethodNameHelperEnum
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "EXPERIMENTAL_validators_ordered"
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
    pub enum ExpValidatorsMethodNameHelperEnum {
        #[serde(rename = "EXPERIMENTAL_validators_ordered")]
        ExperimentalValidatorsOrdered,
    }

    impl ::std::convert::From<&Self> for ExpValidatorsMethodNameHelperEnum {
        fn from(value: &ExpValidatorsMethodNameHelperEnum) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for ExpValidatorsMethodNameHelperEnum {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::ExperimentalValidatorsOrdered => write!(f, "EXPERIMENTAL_validators_ordered"),
            }
        }
    }

    impl ::std::str::FromStr for ExpValidatorsMethodNameHelperEnum {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "EXPERIMENTAL_validators_ordered" => Ok(Self::ExperimentalValidatorsOrdered),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for ExpValidatorsMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for ExpValidatorsMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ExpValidatorsMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///Typed view of ExtCostsConfig to preserve JSON output field names in
    /// protocol config RPC output.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Typed view of ExtCostsConfig to preserve JSON output
    /// field names in protocol config RPC output.",
    ///  "type": "object",
    ///  "required": [
    ///    "alt_bn128_g1_multiexp_base",
    ///    "alt_bn128_g1_multiexp_element",
    ///    "alt_bn128_g1_sum_base",
    ///    "alt_bn128_g1_sum_element",
    ///    "alt_bn128_pairing_check_base",
    ///    "alt_bn128_pairing_check_element",
    ///    "base",
    ///    "bls12381_g1_multiexp_base",
    ///    "bls12381_g1_multiexp_element",
    ///    "bls12381_g2_multiexp_base",
    ///    "bls12381_g2_multiexp_element",
    ///    "bls12381_map_fp2_to_g2_base",
    ///    "bls12381_map_fp2_to_g2_element",
    ///    "bls12381_map_fp_to_g1_base",
    ///    "bls12381_map_fp_to_g1_element",
    ///    "bls12381_p1_decompress_base",
    ///    "bls12381_p1_decompress_element",
    ///    "bls12381_p1_sum_base",
    ///    "bls12381_p1_sum_element",
    ///    "bls12381_p2_decompress_base",
    ///    "bls12381_p2_decompress_element",
    ///    "bls12381_p2_sum_base",
    ///    "bls12381_p2_sum_element",
    ///    "bls12381_pairing_base",
    ///    "bls12381_pairing_element",
    ///    "contract_compile_base",
    ///    "contract_compile_bytes",
    ///    "contract_loading_base",
    ///    "contract_loading_bytes",
    ///    "ecrecover_base",
    ///    "ed25519_verify_base",
    ///    "ed25519_verify_byte",
    ///    "keccak256_base",
    ///    "keccak256_byte",
    ///    "keccak512_base",
    ///    "keccak512_byte",
    ///    "log_base",
    ///    "log_byte",
    ///    "promise_and_base",
    ///    "promise_and_per_promise",
    ///    "promise_return",
    ///    "read_cached_trie_node",
    ///    "read_memory_base",
    ///    "read_memory_byte",
    ///    "read_register_base",
    ///    "read_register_byte",
    ///    "ripemd160_base",
    ///    "ripemd160_block",
    ///    "sha256_base",
    ///    "sha256_byte",
    ///    "storage_has_key_base",
    ///    "storage_has_key_byte",
    ///    "storage_iter_create_from_byte",
    ///    "storage_iter_create_prefix_base",
    ///    "storage_iter_create_prefix_byte",
    ///    "storage_iter_create_range_base",
    ///    "storage_iter_create_to_byte",
    ///    "storage_iter_next_base",
    ///    "storage_iter_next_key_byte",
    ///    "storage_iter_next_value_byte",
    ///    "storage_large_read_overhead_base",
    ///    "storage_large_read_overhead_byte",
    ///    "storage_read_base",
    ///    "storage_read_key_byte",
    ///    "storage_read_value_byte",
    ///    "storage_remove_base",
    ///    "storage_remove_key_byte",
    ///    "storage_remove_ret_value_byte",
    ///    "storage_write_base",
    ///    "storage_write_evicted_byte",
    ///    "storage_write_key_byte",
    ///    "storage_write_value_byte",
    ///    "touching_trie_node",
    ///    "utf16_decoding_base",
    ///    "utf16_decoding_byte",
    ///    "utf8_decoding_base",
    ///    "utf8_decoding_byte",
    ///    "validator_stake_base",
    ///    "validator_total_stake_base",
    ///    "write_memory_base",
    ///    "write_memory_byte",
    ///    "write_register_base",
    ///    "write_register_byte",
    ///    "yield_create_base",
    ///    "yield_create_byte",
    ///    "yield_resume_base",
    ///    "yield_resume_byte"
    ///  ],
    ///  "properties": {
    ///    "alt_bn128_g1_multiexp_base": {
    ///      "description": "Base cost for multiexp",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "alt_bn128_g1_multiexp_element": {
    ///      "description": "Per element cost for multiexp",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "alt_bn128_g1_sum_base": {
    ///      "description": "Base cost for sum",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "alt_bn128_g1_sum_element": {
    ///      "description": "Per element cost for sum",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "alt_bn128_pairing_check_base": {
    ///      "description": "Base cost for pairing check",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "alt_bn128_pairing_check_element": {
    ///      "description": "Per element cost for pairing check",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "base": {
    ///      "description": "Base cost for calling a host function.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "bls12381_g1_multiexp_base": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "bls12381_g1_multiexp_element": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "bls12381_g2_multiexp_base": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "bls12381_g2_multiexp_element": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "bls12381_map_fp2_to_g2_base": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "bls12381_map_fp2_to_g2_element": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "bls12381_map_fp_to_g1_base": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "bls12381_map_fp_to_g1_element": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "bls12381_p1_decompress_base": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "bls12381_p1_decompress_element": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "bls12381_p1_sum_base": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "bls12381_p1_sum_element": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "bls12381_p2_decompress_base": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "bls12381_p2_decompress_element": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "bls12381_p2_sum_base": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "bls12381_p2_sum_element": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "bls12381_pairing_base": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "bls12381_pairing_element": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "contract_compile_base": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "contract_compile_bytes": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "contract_loading_base": {
    ///      "description": "Base cost of loading a pre-compiled contract",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "contract_loading_bytes": {
    ///      "description": "Cost per byte of loading a pre-compiled contract",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "ecrecover_base": {
    ///      "description": "Cost of calling ecrecover",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "ed25519_verify_base": {
    ///      "description": "Cost of getting ed25519 base",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "ed25519_verify_byte": {
    ///      "description": "Cost of getting ed25519 per byte",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "keccak256_base": {
    ///      "description": "Cost of getting sha256 base",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "keccak256_byte": {
    ///      "description": "Cost of getting sha256 per byte",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "keccak512_base": {
    ///      "description": "Cost of getting sha256 base",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "keccak512_byte": {
    ///      "description": "Cost of getting sha256 per byte",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "log_base": {
    ///      "description": "Cost for calling logging.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "log_byte": {
    ///      "description": "Cost for logging per byte",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "promise_and_base": {
    ///      "description": "Cost for calling `promise_and`",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "promise_and_per_promise": {
    ///      "description": "Cost for calling `promise_and` for each promise",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "promise_return": {
    ///      "description": "Cost for calling `promise_return`",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "read_cached_trie_node": {
    ///      "description": "Cost for reading trie node from memory",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "read_memory_base": {
    ///      "description": "Base cost for guest memory read",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "read_memory_byte": {
    ///      "description": "Cost for guest memory read",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "read_register_base": {
    ///      "description": "Base cost for reading from register",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "read_register_byte": {
    ///      "description": "Cost for reading byte from register",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "ripemd160_base": {
    ///      "description": "Cost of getting ripemd160 base",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "ripemd160_block": {
    ///      "description": "Cost of getting ripemd160 per message block",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "sha256_base": {
    ///      "description": "Cost of getting sha256 base",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "sha256_byte": {
    ///      "description": "Cost of getting sha256 per byte",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "storage_has_key_base": {
    ///      "description": "Storage trie check for key existence cost base",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "storage_has_key_byte": {
    ///      "description": "Storage trie check for key existence per key byte",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "storage_iter_create_from_byte": {
    ///      "description": "Create trie range iterator cost per byte of from
    /// key.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "storage_iter_create_prefix_base": {
    ///      "description": "Create trie prefix iterator cost base",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "storage_iter_create_prefix_byte": {
    ///      "description": "Create trie prefix iterator cost per byte.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "storage_iter_create_range_base": {
    ///      "description": "Create trie range iterator cost base",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "storage_iter_create_to_byte": {
    ///      "description": "Create trie range iterator cost per byte of to
    /// key.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "storage_iter_next_base": {
    ///      "description": "Trie iterator per key base cost",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "storage_iter_next_key_byte": {
    ///      "description": "Trie iterator next key byte cost",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "storage_iter_next_value_byte": {
    ///      "description": "Trie iterator next key byte cost",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "storage_large_read_overhead_base": {
    ///      "description": "Storage trie read key overhead base cost, when
    /// doing large reads",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "storage_large_read_overhead_byte": {
    ///      "description": "Storage trie read key overhead  per-byte cost, when
    /// doing large reads",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "storage_read_base": {
    ///      "description": "Storage trie read key base cost",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "storage_read_key_byte": {
    ///      "description": "Storage trie read key per byte cost",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "storage_read_value_byte": {
    ///      "description": "Storage trie read value cost per byte cost",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "storage_remove_base": {
    ///      "description": "Remove key from trie base cost",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "storage_remove_key_byte": {
    ///      "description": "Remove key from trie per byte cost",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "storage_remove_ret_value_byte": {
    ///      "description": "Remove key from trie ret value byte cost",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "storage_write_base": {
    ///      "description": "Storage trie write key base cost",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "storage_write_evicted_byte": {
    ///      "description": "Storage trie write cost per byte of evicted
    /// value.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "storage_write_key_byte": {
    ///      "description": "Storage trie write key per byte cost",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "storage_write_value_byte": {
    ///      "description": "Storage trie write value per byte cost",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "touching_trie_node": {
    ///      "description": "Cost per reading trie node from DB",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "utf16_decoding_base": {
    ///      "description": "Base cost of decoding utf16. It's used for
    /// `log_utf16`.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "utf16_decoding_byte": {
    ///      "description": "Cost per byte of decoding utf16. It's used for
    /// `log_utf16`.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "utf8_decoding_base": {
    ///      "description": "Base cost of decoding utf8. It's used for
    /// `log_utf8` and `panic_utf8`.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "utf8_decoding_byte": {
    ///      "description": "Cost per byte of decoding utf8. It's used for
    /// `log_utf8` and `panic_utf8`.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "validator_stake_base": {
    ///      "description": "Cost of calling `validator_stake`.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "validator_total_stake_base": {
    ///      "description": "Cost of calling `validator_total_stake`.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "write_memory_base": {
    ///      "description": "Base cost for guest memory write",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "write_memory_byte": {
    ///      "description": "Cost for guest memory write per byte",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "write_register_base": {
    ///      "description": "Base cost for writing into register",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "write_register_byte": {
    ///      "description": "Cost for writing byte into register",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "yield_create_base": {
    ///      "description": "Base cost for creating a yield promise.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "yield_create_byte": {
    ///      "description": "Per byte cost of arguments and method name.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "yield_resume_base": {
    ///      "description": "Base cost for resuming a yield receipt.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "yield_resume_byte": {
    ///      "description": "Per byte cost of resume payload.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ExtCostsConfigView {
        ///Base cost for multiexp
        pub alt_bn128_g1_multiexp_base: u64,
        ///Per element cost for multiexp
        pub alt_bn128_g1_multiexp_element: u64,
        ///Base cost for sum
        pub alt_bn128_g1_sum_base: u64,
        ///Per element cost for sum
        pub alt_bn128_g1_sum_element: u64,
        ///Base cost for pairing check
        pub alt_bn128_pairing_check_base: u64,
        ///Per element cost for pairing check
        pub alt_bn128_pairing_check_element: u64,
        ///Base cost for calling a host function.
        pub base: u64,
        pub bls12381_g1_multiexp_base: u64,
        pub bls12381_g1_multiexp_element: u64,
        pub bls12381_g2_multiexp_base: u64,
        pub bls12381_g2_multiexp_element: u64,
        pub bls12381_map_fp2_to_g2_base: u64,
        pub bls12381_map_fp2_to_g2_element: u64,
        pub bls12381_map_fp_to_g1_base: u64,
        pub bls12381_map_fp_to_g1_element: u64,
        pub bls12381_p1_decompress_base: u64,
        pub bls12381_p1_decompress_element: u64,
        pub bls12381_p1_sum_base: u64,
        pub bls12381_p1_sum_element: u64,
        pub bls12381_p2_decompress_base: u64,
        pub bls12381_p2_decompress_element: u64,
        pub bls12381_p2_sum_base: u64,
        pub bls12381_p2_sum_element: u64,
        pub bls12381_pairing_base: u64,
        pub bls12381_pairing_element: u64,
        pub contract_compile_base: u64,
        pub contract_compile_bytes: u64,
        ///Base cost of loading a pre-compiled contract
        pub contract_loading_base: u64,
        ///Cost per byte of loading a pre-compiled contract
        pub contract_loading_bytes: u64,
        ///Cost of calling ecrecover
        pub ecrecover_base: u64,
        ///Cost of getting ed25519 base
        pub ed25519_verify_base: u64,
        ///Cost of getting ed25519 per byte
        pub ed25519_verify_byte: u64,
        ///Cost of getting sha256 base
        pub keccak256_base: u64,
        ///Cost of getting sha256 per byte
        pub keccak256_byte: u64,
        ///Cost of getting sha256 base
        pub keccak512_base: u64,
        ///Cost of getting sha256 per byte
        pub keccak512_byte: u64,
        ///Cost for calling logging.
        pub log_base: u64,
        ///Cost for logging per byte
        pub log_byte: u64,
        ///Cost for calling `promise_and`
        pub promise_and_base: u64,
        ///Cost for calling `promise_and` for each promise
        pub promise_and_per_promise: u64,
        ///Cost for calling `promise_return`
        pub promise_return: u64,
        ///Cost for reading trie node from memory
        pub read_cached_trie_node: u64,
        ///Base cost for guest memory read
        pub read_memory_base: u64,
        ///Cost for guest memory read
        pub read_memory_byte: u64,
        ///Base cost for reading from register
        pub read_register_base: u64,
        ///Cost for reading byte from register
        pub read_register_byte: u64,
        ///Cost of getting ripemd160 base
        pub ripemd160_base: u64,
        ///Cost of getting ripemd160 per message block
        pub ripemd160_block: u64,
        ///Cost of getting sha256 base
        pub sha256_base: u64,
        ///Cost of getting sha256 per byte
        pub sha256_byte: u64,
        ///Storage trie check for key existence cost base
        pub storage_has_key_base: u64,
        ///Storage trie check for key existence per key byte
        pub storage_has_key_byte: u64,
        ///Create trie range iterator cost per byte of from key.
        pub storage_iter_create_from_byte: u64,
        ///Create trie prefix iterator cost base
        pub storage_iter_create_prefix_base: u64,
        ///Create trie prefix iterator cost per byte.
        pub storage_iter_create_prefix_byte: u64,
        ///Create trie range iterator cost base
        pub storage_iter_create_range_base: u64,
        ///Create trie range iterator cost per byte of to key.
        pub storage_iter_create_to_byte: u64,
        ///Trie iterator per key base cost
        pub storage_iter_next_base: u64,
        ///Trie iterator next key byte cost
        pub storage_iter_next_key_byte: u64,
        ///Trie iterator next key byte cost
        pub storage_iter_next_value_byte: u64,
        ///Storage trie read key overhead base cost, when doing large reads
        pub storage_large_read_overhead_base: u64,
        ///Storage trie read key overhead  per-byte cost, when doing large
        /// reads
        pub storage_large_read_overhead_byte: u64,
        ///Storage trie read key base cost
        pub storage_read_base: u64,
        ///Storage trie read key per byte cost
        pub storage_read_key_byte: u64,
        ///Storage trie read value cost per byte cost
        pub storage_read_value_byte: u64,
        ///Remove key from trie base cost
        pub storage_remove_base: u64,
        ///Remove key from trie per byte cost
        pub storage_remove_key_byte: u64,
        ///Remove key from trie ret value byte cost
        pub storage_remove_ret_value_byte: u64,
        ///Storage trie write key base cost
        pub storage_write_base: u64,
        ///Storage trie write cost per byte of evicted value.
        pub storage_write_evicted_byte: u64,
        ///Storage trie write key per byte cost
        pub storage_write_key_byte: u64,
        ///Storage trie write value per byte cost
        pub storage_write_value_byte: u64,
        ///Cost per reading trie node from DB
        pub touching_trie_node: u64,
        ///Base cost of decoding utf16. It's used for `log_utf16`.
        pub utf16_decoding_base: u64,
        ///Cost per byte of decoding utf16. It's used for `log_utf16`.
        pub utf16_decoding_byte: u64,
        ///Base cost of decoding utf8. It's used for `log_utf8` and
        /// `panic_utf8`.
        pub utf8_decoding_base: u64,
        ///Cost per byte of decoding utf8. It's used for `log_utf8` and
        /// `panic_utf8`.
        pub utf8_decoding_byte: u64,
        ///Cost of calling `validator_stake`.
        pub validator_stake_base: u64,
        ///Cost of calling `validator_total_stake`.
        pub validator_total_stake_base: u64,
        ///Base cost for guest memory write
        pub write_memory_base: u64,
        ///Cost for guest memory write per byte
        pub write_memory_byte: u64,
        ///Base cost for writing into register
        pub write_register_base: u64,
        ///Cost for writing byte into register
        pub write_register_byte: u64,
        ///Base cost for creating a yield promise.
        pub yield_create_base: u64,
        ///Per byte cost of arguments and method name.
        pub yield_create_byte: u64,
        ///Base cost for resuming a yield receipt.
        pub yield_resume_base: u64,
        ///Per byte cost of resume payload.
        pub yield_resume_byte: u64,
    }

    impl ::std::convert::From<&ExtCostsConfigView> for ExtCostsConfigView {
        fn from(value: &ExtCostsConfigView) -> Self {
            value.clone()
        }
    }

    ///ExternalStorageConfig
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "location"
    ///  ],
    ///  "properties": {
    ///    "external_storage_fallback_threshold": {
    ///      "description": "The number of attempts the node will make to obtain
    /// a part from peers in the network before it fetches from external
    /// storage.",
    ///      "default": 3,
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "location": {
    ///      "description": "Location of state parts.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/ExternalStorageLocation"
    ///        }
    ///      ]
    ///    },
    ///    "num_concurrent_requests": {
    ///      "description": "When fetching state parts from external storage,
    /// throttle fetch requests to this many concurrent requests.",
    ///      "default": 25,
    ///      "type": "integer",
    ///      "format": "uint32",
    ///      "minimum": 0.0
    ///    },
    ///    "num_concurrent_requests_during_catchup": {
    ///      "description": "During catchup, the node will use a different
    /// number of concurrent requests to reduce the performance impact of state
    /// sync.",
    ///      "default": 5,
    ///      "type": "integer",
    ///      "format": "uint32",
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ExternalStorageConfig {
        ///The number of attempts the node will make to obtain a part from
        /// peers in the network before it fetches from external storage.
        #[serde(default = "defaults::default_u64::<u64, 3>")]
        pub external_storage_fallback_threshold: u64,
        ///Location of state parts.
        pub location: ExternalStorageLocation,
        ///When fetching state parts from external storage, throttle fetch
        /// requests to this many concurrent requests.
        #[serde(default = "defaults::default_u64::<u32, 25>")]
        pub num_concurrent_requests: u32,
        ///During catchup, the node will use a different number of concurrent
        /// requests to reduce the performance impact of state sync.
        #[serde(default = "defaults::default_u64::<u32, 5>")]
        pub num_concurrent_requests_during_catchup: u32,
    }

    impl ::std::convert::From<&ExternalStorageConfig> for ExternalStorageConfig {
        fn from(value: &ExternalStorageConfig) -> Self {
            value.clone()
        }
    }

    ///ExternalStorageLocation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "S3"
    ///      ],
    ///      "properties": {
    ///        "S3": {
    ///          "type": "object",
    ///          "required": [
    ///            "bucket",
    ///            "region"
    ///          ],
    ///          "properties": {
    ///            "bucket": {
    ///              "description": "Location of state dumps on S3.",
    ///              "type": "string"
    ///            },
    ///            "region": {
    ///              "description": "Data may only be available in certain
    /// locations.",
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "Filesystem"
    ///      ],
    ///      "properties": {
    ///        "Filesystem": {
    ///          "type": "object",
    ///          "required": [
    ///            "root_dir"
    ///          ],
    ///          "properties": {
    ///            "root_dir": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "GCS"
    ///      ],
    ///      "properties": {
    ///        "GCS": {
    ///          "type": "object",
    ///          "required": [
    ///            "bucket"
    ///          ],
    ///          "properties": {
    ///            "bucket": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub enum ExternalStorageLocation {
        S3 {
            ///Location of state dumps on S3.
            bucket: ::std::string::String,
            ///Data may only be available in certain locations.
            region: ::std::string::String,
        },
        Filesystem {
            root_dir: ::std::string::String,
        },
        #[serde(rename = "GCS")]
        Gcs {
            bucket: ::std::string::String,
        },
    }

    impl ::std::convert::From<&Self> for ExternalStorageLocation {
        fn from(value: &ExternalStorageLocation) -> Self {
            value.clone()
        }
    }

    ///Costs associated with an object that can only be sent over the network
    /// (and executed by the receiver). NOTE: `send_sir` or `send_not_sir` fees
    /// are usually burned when the item is being created. And `execution` fee
    /// is burned when the item is being executed.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Costs associated with an object that can only be sent
    /// over the network (and executed by the receiver). NOTE: `send_sir` or
    /// `send_not_sir` fees are usually burned when the item is being created.
    /// And `execution` fee is burned when the item is being executed.",
    ///  "type": "object",
    ///  "required": [
    ///    "execution",
    ///    "send_not_sir",
    ///    "send_sir"
    ///  ],
    ///  "properties": {
    ///    "execution": {
    ///      "description": "Fee for executing the object.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "send_not_sir": {
    ///      "description": "Fee for sending an object potentially across the
    /// shards.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "send_sir": {
    ///      "description": "Fee for sending an object from the sender to
    /// itself, guaranteeing that it does not leave the shard.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Fee {
        ///Fee for executing the object.
        pub execution: u64,
        ///Fee for sending an object potentially across the shards.
        pub send_not_sir: u64,
        ///Fee for sending an object from the sender to itself, guaranteeing
        /// that it does not leave the shard.
        pub send_sir: u64,
    }

    impl ::std::convert::From<&Fee> for Fee {
        fn from(value: &Fee) -> Self {
            value.clone()
        }
    }

    ///Execution outcome of the transaction and all the subsequent receipts.
    /// Could be not finalized yet
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Execution outcome of the transaction and all the
    /// subsequent receipts. Could be not finalized yet",
    ///  "type": "object",
    ///  "required": [
    ///    "receipts_outcome",
    ///    "status",
    ///    "transaction",
    ///    "transaction_outcome"
    ///  ],
    ///  "properties": {
    ///    "receipts_outcome": {
    ///      "description": "The execution outcome of receipts.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ExecutionOutcomeWithIdView"
    ///      }
    ///    },
    ///    "status": {
    ///      "description": "Execution status defined by
    /// chain.rs:get_final_transaction_result FinalExecutionStatus::NotStarted -
    /// the tx is not converted to the receipt yet FinalExecutionStatus::Started
    /// - we have at least 1 receipt, but the first leaf receipt_id (using dfs)
    /// hasn't finished the execution FinalExecutionStatus::Failure - the result
    /// of the first leaf receipt_id FinalExecutionStatus::SuccessValue - the
    /// result of the first leaf receipt_id",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/FinalExecutionStatus"
    ///        }
    ///      ]
    ///    },
    ///    "transaction": {
    ///      "description": "Signed Transaction",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/SignedTransactionView"
    ///        }
    ///      ]
    ///    },
    ///    "transaction_outcome": {
    ///      "description": "The execution outcome of the signed transaction.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/ExecutionOutcomeWithIdView"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct FinalExecutionOutcomeView {
        ///The execution outcome of receipts.
        pub receipts_outcome: ::std::vec::Vec<ExecutionOutcomeWithIdView>,
        ///Execution status defined by chain.rs:get_final_transaction_result
        /// FinalExecutionStatus::NotStarted - the tx is not converted to the
        /// receipt yet FinalExecutionStatus::Started - we have at least 1
        /// receipt, but the first leaf receipt_id (using dfs) hasn't finished
        /// the execution FinalExecutionStatus::Failure - the result of the
        /// first leaf receipt_id FinalExecutionStatus::SuccessValue - the
        /// result of the first leaf receipt_id
        pub status: FinalExecutionStatus,
        ///Signed Transaction
        pub transaction: SignedTransactionView,
        ///The execution outcome of the signed transaction.
        pub transaction_outcome: ExecutionOutcomeWithIdView,
    }

    impl ::std::convert::From<&FinalExecutionOutcomeView> for FinalExecutionOutcomeView {
        fn from(value: &FinalExecutionOutcomeView) -> Self {
            value.clone()
        }
    }

    ///Final execution outcome of the transaction and all of subsequent the
    /// receipts. Also includes the generated receipt.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Final execution outcome of the transaction and all of
    /// subsequent the receipts. Also includes the generated receipt.",
    ///  "type": "object",
    ///  "required": [
    ///    "receipts",
    ///    "receipts_outcome",
    ///    "status",
    ///    "transaction",
    ///    "transaction_outcome"
    ///  ],
    ///  "properties": {
    ///    "receipts": {
    ///      "description": "Receipts generated from the transaction",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ReceiptView"
    ///      }
    ///    },
    ///    "receipts_outcome": {
    ///      "description": "The execution outcome of receipts.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ExecutionOutcomeWithIdView"
    ///      }
    ///    },
    ///    "status": {
    ///      "description": "Execution status defined by
    /// chain.rs:get_final_transaction_result FinalExecutionStatus::NotStarted -
    /// the tx is not converted to the receipt yet FinalExecutionStatus::Started
    /// - we have at least 1 receipt, but the first leaf receipt_id (using dfs)
    /// hasn't finished the execution FinalExecutionStatus::Failure - the result
    /// of the first leaf receipt_id FinalExecutionStatus::SuccessValue - the
    /// result of the first leaf receipt_id",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/FinalExecutionStatus"
    ///        }
    ///      ]
    ///    },
    ///    "transaction": {
    ///      "description": "Signed Transaction",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/SignedTransactionView"
    ///        }
    ///      ]
    ///    },
    ///    "transaction_outcome": {
    ///      "description": "The execution outcome of the signed transaction.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/ExecutionOutcomeWithIdView"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct FinalExecutionOutcomeWithReceiptView {
        ///Receipts generated from the transaction
        pub receipts: ::std::vec::Vec<ReceiptView>,
        ///The execution outcome of receipts.
        pub receipts_outcome: ::std::vec::Vec<ExecutionOutcomeWithIdView>,
        ///Execution status defined by chain.rs:get_final_transaction_result
        /// FinalExecutionStatus::NotStarted - the tx is not converted to the
        /// receipt yet FinalExecutionStatus::Started - we have at least 1
        /// receipt, but the first leaf receipt_id (using dfs) hasn't finished
        /// the execution FinalExecutionStatus::Failure - the result of the
        /// first leaf receipt_id FinalExecutionStatus::SuccessValue - the
        /// result of the first leaf receipt_id
        pub status: FinalExecutionStatus,
        ///Signed Transaction
        pub transaction: SignedTransactionView,
        ///The execution outcome of the signed transaction.
        pub transaction_outcome: ExecutionOutcomeWithIdView,
    }

    impl ::std::convert::From<&FinalExecutionOutcomeWithReceiptView>
        for FinalExecutionOutcomeWithReceiptView
    {
        fn from(value: &FinalExecutionOutcomeWithReceiptView) -> Self {
            value.clone()
        }
    }

    ///FinalExecutionStatus
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "description": "The execution has not yet started.",
    ///      "type": "string",
    ///      "enum": [
    ///        "NotStarted"
    ///      ]
    ///    },
    ///    {
    ///      "description": "The execution has started and still going.",
    ///      "type": "string",
    ///      "enum": [
    ///        "Started"
    ///      ]
    ///    },
    ///    {
    ///      "description": "The execution has failed with the given error.",
    ///      "type": "object",
    ///      "required": [
    ///        "Failure"
    ///      ],
    ///      "properties": {
    ///        "Failure": {
    ///          "$ref": "#/components/schemas/TxExecutionError"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "The execution has succeeded and returned some value
    /// or an empty vec encoded in base64.",
    ///      "type": "object",
    ///      "required": [
    ///        "SuccessValue"
    ///      ],
    ///      "properties": {
    ///        "SuccessValue": {
    ///          "type": "string"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub enum FinalExecutionStatus {
        ///The execution has not yet started.
        NotStarted,
        ///The execution has started and still going.
        Started,
        ///The execution has failed with the given error.
        Failure(TxExecutionError),
        ///The execution has succeeded and returned some value or an empty vec
        /// encoded in base64.
        SuccessValue(::std::string::String),
    }

    impl ::std::convert::From<&Self> for FinalExecutionStatus {
        fn from(value: &FinalExecutionStatus) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<TxExecutionError> for FinalExecutionStatus {
        fn from(value: TxExecutionError) -> Self {
            Self::Failure(value)
        }
    }

    ///Different types of finality.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Different types of finality.",
    ///  "type": "string",
    ///  "enum": [
    ///    "optimistic",
    ///    "near-final",
    ///    "final"
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
    pub enum Finality {
        #[serde(rename = "optimistic")]
        Optimistic,
        #[serde(rename = "near-final")]
        NearFinal,
        #[serde(rename = "final")]
        Final,
    }

    impl ::std::convert::From<&Self> for Finality {
        fn from(value: &Finality) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for Finality {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Optimistic => write!(f, "optimistic"),
                Self::NearFinal => write!(f, "near-final"),
                Self::Final => write!(f, "final"),
            }
        }
    }

    impl ::std::str::FromStr for Finality {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "optimistic" => Ok(Self::Optimistic),
                "near-final" => Ok(Self::NearFinal),
                "final" => Ok(Self::Final),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for Finality {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for Finality {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for Finality {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///FunctionCallAction
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "args",
    ///    "deposit",
    ///    "gas",
    ///    "method_name"
    ///  ],
    ///  "properties": {
    ///    "args": {
    ///      "type": "string"
    ///    },
    ///    "deposit": {
    ///      "type": "string"
    ///    },
    ///    "gas": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "method_name": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct FunctionCallAction {
        pub args: ::std::string::String,
        pub deposit: ::std::string::String,
        pub gas: u64,
        pub method_name: ::std::string::String,
    }

    impl ::std::convert::From<&FunctionCallAction> for FunctionCallAction {
        fn from(value: &FunctionCallAction) -> Self {
            value.clone()
        }
    }

    ///Serializable version of `near-vm-runner::FunctionCallError`.
    ///
    ///Must never reorder/remove elements, can only add new variants at the end
    /// (but do that very carefully). It describes stable serialization format,
    /// and only used by serialization logic.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Serializable version of
    /// `near-vm-runner::FunctionCallError`.\n\nMust never reorder/remove
    /// elements, can only add new variants at the end (but do that very
    /// carefully). It describes stable serialization format, and only used by
    /// serialization logic.",
    ///  "oneOf": [
    ///    {
    ///      "type": "string",
    ///      "enum": [
    ///        "WasmUnknownError",
    ///        "_EVMError"
    ///      ]
    ///    },
    ///    {
    ///      "description": "Wasm compilation error",
    ///      "type": "object",
    ///      "required": [
    ///        "CompilationError"
    ///      ],
    ///      "properties": {
    ///        "CompilationError": {
    ///          "$ref": "#/components/schemas/CompilationError"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "Wasm binary env link error\n\nNote: this is only to
    /// deserialize old data, use execution error for new data",
    ///      "type": "object",
    ///      "required": [
    ///        "LinkError"
    ///      ],
    ///      "properties": {
    ///        "LinkError": {
    ///          "type": "object",
    ///          "required": [
    ///            "msg"
    ///          ],
    ///          "properties": {
    ///            "msg": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "Import/export resolve error",
    ///      "type": "object",
    ///      "required": [
    ///        "MethodResolveError"
    ///      ],
    ///      "properties": {
    ///        "MethodResolveError": {
    ///          "$ref": "#/components/schemas/MethodResolveError"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "A trap happened during execution of a
    /// binary\n\nNote: this is only to deserialize old data, use execution
    /// error for new data",
    ///      "type": "object",
    ///      "required": [
    ///        "WasmTrap"
    ///      ],
    ///      "properties": {
    ///        "WasmTrap": {
    ///          "$ref": "#/components/schemas/WasmTrap"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "Note: this is only to deserialize old data, use
    /// execution error for new data",
    ///      "type": "object",
    ///      "required": [
    ///        "HostError"
    ///      ],
    ///      "properties": {
    ///        "HostError": {
    ///          "$ref": "#/components/schemas/HostError"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "ExecutionError"
    ///      ],
    ///      "properties": {
    ///        "ExecutionError": {
    ///          "type": "string"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub enum FunctionCallError {
        WasmUnknownError,
        #[serde(rename = "_EVMError")]
        EvmError,
        ///Wasm compilation error
        CompilationError(CompilationError),
        ///Wasm binary env link error
        ///
        ///Note: this is only to deserialize old data, use execution error for
        /// new data
        LinkError {
            msg: ::std::string::String,
        },
        ///Import/export resolve error
        MethodResolveError(MethodResolveError),
        ///A trap happened during execution of a binary
        ///
        ///Note: this is only to deserialize old data, use execution error for
        /// new data
        WasmTrap(WasmTrap),
        ///Note: this is only to deserialize old data, use execution error for
        /// new data
        HostError(HostError),
        ExecutionError(::std::string::String),
    }

    impl ::std::convert::From<&Self> for FunctionCallError {
        fn from(value: &FunctionCallError) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<CompilationError> for FunctionCallError {
        fn from(value: CompilationError) -> Self {
            Self::CompilationError(value)
        }
    }

    impl ::std::convert::From<MethodResolveError> for FunctionCallError {
        fn from(value: MethodResolveError) -> Self {
            Self::MethodResolveError(value)
        }
    }

    impl ::std::convert::From<WasmTrap> for FunctionCallError {
        fn from(value: WasmTrap) -> Self {
            Self::WasmTrap(value)
        }
    }

    impl ::std::convert::From<HostError> for FunctionCallError {
        fn from(value: HostError) -> Self {
            Self::HostError(value)
        }
    }

    ///Grants limited permission to make transactions with FunctionCallActions
    /// The permission can limit the allowed balance to be spent on the prepaid
    /// gas. It also restrict the account ID of the receiver for this function
    /// call. It also can restrict the method name for the allowed function
    /// calls.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Grants limited permission to make transactions with
    /// FunctionCallActions The permission can limit the allowed balance to be
    /// spent on the prepaid gas. It also restrict the account ID of the
    /// receiver for this function call. It also can restrict the method name
    /// for the allowed function calls.",
    ///  "type": "object",
    ///  "required": [
    ///    "allowance",
    ///    "method_names",
    ///    "receiver_id"
    ///  ],
    ///  "properties": {
    ///    "allowance": {
    ///      "description": "Allowance is a balance limit to use by this access
    /// key to pay for function call gas and transaction fees. When this access
    /// key is used, both account balance and the allowance is decreased by the
    /// same value. `None` means unlimited allowance. NOTE: To change or
    /// increase the allowance, the old access key needs to be deleted and a new
    /// access key should be created.",
    ///      "type": "string"
    ///    },
    ///    "method_names": {
    ///      "description": "A list of method names that can be used. The access
    /// key only allows transactions with the function call of one of the given
    /// method names. Empty list means any method name can be used.",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "receiver_id": {
    ///      "description": "The access key only allows transactions with the
    /// given receiver's account id.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct FunctionCallPermission {
        ///Allowance is a balance limit to use by this access key to pay for
        /// function call gas and transaction fees. When this access key is
        /// used, both account balance and the allowance is decreased by the
        /// same value. `None` means unlimited allowance. NOTE: To change or
        /// increase the allowance, the old access key needs to be deleted and a
        /// new access key should be created.
        pub allowance: ::std::string::String,
        ///A list of method names that can be used. The access key only allows
        /// transactions with the function call of one of the given method
        /// names. Empty list means any method name can be used.
        pub method_names: ::std::vec::Vec<::std::string::String>,
        ///The access key only allows transactions with the given receiver's
        /// account id.
        pub receiver_id: ::std::string::String,
    }

    impl ::std::convert::From<&FunctionCallPermission> for FunctionCallPermission {
        fn from(value: &FunctionCallPermission) -> Self {
            value.clone()
        }
    }

    ///GasPriceMethodNameHelperEnum
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "gas_price"
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
    pub enum GasPriceMethodNameHelperEnum {
        #[serde(rename = "gas_price")]
        GasPrice,
    }

    impl ::std::convert::From<&Self> for GasPriceMethodNameHelperEnum {
        fn from(value: &GasPriceMethodNameHelperEnum) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for GasPriceMethodNameHelperEnum {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::GasPrice => write!(f, "gas_price"),
            }
        }
    }

    impl ::std::str::FromStr for GasPriceMethodNameHelperEnum {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "gas_price" => Ok(Self::GasPrice),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for GasPriceMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for GasPriceMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GasPriceMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///Configuration for garbage collection.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Configuration for garbage collection.",
    ///  "type": "object",
    ///  "properties": {
    ///    "gc_blocks_limit": {
    ///      "description": "Maximum number of blocks to garbage collect at
    /// every garbage collection call.",
    ///      "default": 2,
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "gc_fork_clean_step": {
    ///      "description": "Maximum number of height to go through at each garbage collection step when cleaning forks during garbage collection.",
    ///      "default": 100,
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "gc_num_epochs_to_keep": {
    ///      "description": "Number of epochs for which we keep store data.",
    ///      "default": 5,
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "gc_step_period": {
    ///      "description": "How often gc should be run",
    ///      "default": {
    ///        "nanoseconds": 0,
    ///        "seconds": 1
    ///      },
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/DurationSchemeProvider"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GcConfig {
        ///Maximum number of blocks to garbage collect at every garbage
        /// collection call.
        #[serde(default = "defaults::default_u64::<u64, 2>")]
        pub gc_blocks_limit: u64,
        ///Maximum number of height to go through at each garbage collection
        /// step when cleaning forks during garbage collection.
        #[serde(default = "defaults::default_u64::<u64, 100>")]
        pub gc_fork_clean_step: u64,
        ///Number of epochs for which we keep store data.
        #[serde(default = "defaults::default_u64::<u64, 5>")]
        pub gc_num_epochs_to_keep: u64,
        ///How often gc should be run
        #[serde(default = "defaults::gc_config_gc_step_period")]
        pub gc_step_period: DurationSchemeProvider,
    }

    impl ::std::convert::From<&GcConfig> for GcConfig {
        fn from(value: &GcConfig) -> Self {
            value.clone()
        }
    }

    impl ::std::default::Default for GcConfig {
        fn default() -> Self {
            Self {
                gc_blocks_limit: defaults::default_u64::<u64, 2>(),
                gc_fork_clean_step: defaults::default_u64::<u64, 100>(),
                gc_num_epochs_to_keep: defaults::default_u64::<u64, 5>(),
                gc_step_period: defaults::gc_config_gc_step_period(),
            }
        }
    }

    ///GenesisConfig
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "avg_hidden_validator_seats_per_shard",
    ///    "block_producer_kickout_threshold",
    ///    "chain_id",
    ///    "chunk_producer_kickout_threshold",
    ///    "dynamic_resharding",
    ///    "epoch_length",
    ///    "fishermen_threshold",
    ///    "gas_limit",
    ///    "gas_price_adjustment_rate",
    ///    "genesis_height",
    ///    "genesis_time",
    ///    "max_gas_price",
    ///    "max_inflation_rate",
    ///    "min_gas_price",
    ///    "num_block_producer_seats",
    ///    "num_block_producer_seats_per_shard",
    ///    "num_blocks_per_year",
    ///    "protocol_reward_rate",
    ///    "protocol_treasury_account",
    ///    "protocol_version",
    ///    "total_supply",
    ///    "transaction_validity_period",
    ///    "validators"
    ///  ],
    ///  "properties": {
    ///    "avg_hidden_validator_seats_per_shard": {
    ///      "description": "Expected number of hidden validators per shard.",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "integer",
    ///        "format": "uint64",
    ///        "minimum": 0.0
    ///      }
    ///    },
    ///    "block_producer_kickout_threshold": {
    ///      "description": "Threshold for kicking out block producers, between
    /// 0 and 100.",
    ///      "type": "integer",
    ///      "format": "uint8",
    ///      "minimum": 0.0
    ///    },
    ///    "chain_id": {
    ///      "description": "ID of the blockchain. This must be unique for every
    /// blockchain. If your testnet blockchains do not have unique chain IDs,
    /// you will have a bad time.",
    ///      "type": "string"
    ///    },
    ///    "chunk_producer_assignment_changes_limit": {
    ///      "description": "Limits the number of shard changes in chunk
    /// producer assignments, if algorithm is able to choose assignment with
    /// better balance of number of chunk producers for shards.",
    ///      "default": 5,
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "chunk_producer_kickout_threshold": {
    ///      "description": "Threshold for kicking out chunk producers, between
    /// 0 and 100.",
    ///      "type": "integer",
    ///      "format": "uint8",
    ///      "minimum": 0.0
    ///    },
    ///    "chunk_validator_only_kickout_threshold": {
    ///      "description": "Threshold for kicking out nodes which are only
    /// chunk validators, between 0 and 100.",
    ///      "default": 80,
    ///      "type": "integer",
    ///      "format": "uint8",
    ///      "minimum": 0.0
    ///    },
    ///    "dynamic_resharding": {
    ///      "description": "Enable dynamic re-sharding.",
    ///      "type": "boolean"
    ///    },
    ///    "epoch_length": {
    ///      "description": "Epoch length counted in block heights.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "fishermen_threshold": {
    ///      "description": "Fishermen stake threshold.",
    ///      "type": "string"
    ///    },
    ///    "gas_limit": {
    ///      "description": "Initial gas limit.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "gas_price_adjustment_rate": {
    ///      "description": "Gas price adjustment rate",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Rational32SchemaProvider"
    ///        }
    ///      ]
    ///    },
    ///    "genesis_height": {
    ///      "description": "Height of genesis block.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "genesis_time": {
    ///      "description": "Official time of blockchain start.",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "max_gas_price": {
    ///      "type": "string"
    ///    },
    ///    "max_inflation_rate": {
    ///      "description": "Maximum inflation on the total supply every
    /// epoch.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Rational32SchemaProvider"
    ///        }
    ///      ]
    ///    },
    ///    "max_kickout_stake_perc": {
    ///      "description": "Max stake percentage of the validators we will kick
    /// out.",
    ///      "default": 100,
    ///      "type": "integer",
    ///      "format": "uint8",
    ///      "minimum": 0.0
    ///    },
    ///    "min_gas_price": {
    ///      "description": "Minimum gas price. It is also the initial gas
    /// price.",
    ///      "type": "string"
    ///    },
    ///    "minimum_stake_divisor": {
    ///      "description": "The minimum stake required for staking is last seat
    /// price divided by this number.",
    ///      "default": 10,
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "minimum_stake_ratio": {
    ///      "description": "The lowest ratio s/s_total any block producer can have. See <https://github.com/near/NEPs/pull/167> for details",
    ///      "default": {
    ///        "denom": 1,
    ///        "numer": 6250
    ///      },
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Rational32SchemaProvider"
    ///        }
    ///      ]
    ///    },
    ///    "minimum_validators_per_shard": {
    ///      "description": "The minimum number of validators each shard must
    /// have",
    ///      "default": 1,
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "num_block_producer_seats": {
    ///      "description": "Number of block producer seats at genesis.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "num_block_producer_seats_per_shard": {
    ///      "description": "Defines number of shards and number of block
    /// producer seats per each shard at genesis. Note: not used with
    /// protocol_feature_chunk_only_producers -- replaced by
    /// minimum_validators_per_shard Note: not used before as all block
    /// producers produce chunks for all shards",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "integer",
    ///        "format": "uint64",
    ///        "minimum": 0.0
    ///      }
    ///    },
    ///    "num_blocks_per_year": {
    ///      "description": "Expected number of blocks per year",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "num_chunk_only_producer_seats": {
    ///      "description": "Deprecated.",
    ///      "default": 300,
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "num_chunk_producer_seats": {
    ///      "description": "Number of chunk producers. Don't mess it up with
    /// chunk-only producers feature which is deprecated.",
    ///      "default": 100,
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "num_chunk_validator_seats": {
    ///      "default": 300,
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "online_max_threshold": {
    ///      "description": "Online maximum threshold above which validator gets
    /// full reward.",
    ///      "default": {
    ///        "denom": 99,
    ///        "numer": 100
    ///      },
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Rational32SchemaProvider"
    ///        }
    ///      ]
    ///    },
    ///    "online_min_threshold": {
    ///      "description": "Online minimum threshold below which validator
    /// doesn't receive reward.",
    ///      "default": {
    ///        "denom": 9,
    ///        "numer": 10
    ///      },
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Rational32SchemaProvider"
    ///        }
    ///      ]
    ///    },
    ///    "protocol_reward_rate": {
    ///      "description": "Protocol treasury rate",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Rational32SchemaProvider"
    ///        }
    ///      ]
    ///    },
    ///    "protocol_treasury_account": {
    ///      "description": "Protocol treasury account",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/AccountId"
    ///        }
    ///      ]
    ///    },
    ///    "protocol_upgrade_stake_threshold": {
    ///      "description": "Threshold of stake that needs to indicate that they
    /// ready for upgrade.",
    ///      "default": {
    ///        "denom": 4,
    ///        "numer": 5
    ///      },
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Rational32SchemaProvider"
    ///        }
    ///      ]
    ///    },
    ///    "protocol_version": {
    ///      "description": "Protocol version that this genesis works with.",
    ///      "type": "integer",
    ///      "format": "uint32",
    ///      "minimum": 0.0
    ///    },
    ///    "shard_layout": {
    ///      "description": "Layout information regarding how to split accounts
    /// to shards",
    ///      "default": {
    ///        "V2": {
    ///          "boundary_accounts": [],
    ///          "id_to_index_map": {
    ///            "0": 0
    ///          },
    ///          "index_to_id_map": {
    ///            "0": 0
    ///          },
    ///          "shard_ids": [
    ///            0
    ///          ],
    ///          "shards_parent_map": null,
    ///          "shards_split_map": null,
    ///          "version": 0
    ///        }
    ///      },
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/ShardLayout"
    ///        }
    ///      ]
    ///    },
    ///    "shuffle_shard_assignment_for_chunk_producers": {
    ///      "description": "If true, shuffle the chunk producers across shards.
    /// In other words, if the shard assignments were `[S_0, S_1, S_2, S_3]`
    /// where `S_i` represents the set of chunk producers for shard `i`, if this
    /// flag were true, the shard assignments might become, for example, `[S_2,
    /// S_0, S_3, S_1]`.",
    ///      "default": false,
    ///      "type": "boolean"
    ///    },
    ///    "target_validator_mandates_per_shard": {
    ///      "description": "Number of target chunk validator mandates for each
    /// shard.",
    ///      "default": 68,
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "total_supply": {
    ///      "description": "Total supply of tokens at genesis.",
    ///      "type": "string"
    ///    },
    ///    "transaction_validity_period": {
    ///      "description": "Number of blocks for which a given transaction is
    /// valid",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "use_production_config": {
    ///      "description": "This is only for test purposes. We hard code some configs for mainnet and testnet in AllEpochConfig, and we want to have a way to test that code path. This flag is for that. If set to true, the node will use the same config override path as mainnet and testnet.",
    ///      "default": false,
    ///      "type": "boolean"
    ///    },
    ///    "validators": {
    ///      "description": "List of initial validators.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/AccountInfo"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GenesisConfig {
        ///Expected number of hidden validators per shard.
        pub avg_hidden_validator_seats_per_shard: ::std::vec::Vec<u64>,
        ///Threshold for kicking out block producers, between 0 and 100.
        pub block_producer_kickout_threshold: u8,
        ///ID of the blockchain. This must be unique for every blockchain. If
        /// your testnet blockchains do not have unique chain IDs, you will have
        /// a bad time.
        pub chain_id: ::std::string::String,
        ///Limits the number of shard changes in chunk producer assignments, if
        /// algorithm is able to choose assignment with better balance of number
        /// of chunk producers for shards.
        #[serde(default = "defaults::default_u64::<u64, 5>")]
        pub chunk_producer_assignment_changes_limit: u64,
        ///Threshold for kicking out chunk producers, between 0 and 100.
        pub chunk_producer_kickout_threshold: u8,
        ///Threshold for kicking out nodes which are only chunk validators,
        /// between 0 and 100.
        #[serde(default = "defaults::default_u64::<u8, 80>")]
        pub chunk_validator_only_kickout_threshold: u8,
        ///Enable dynamic re-sharding.
        pub dynamic_resharding: bool,
        ///Epoch length counted in block heights.
        pub epoch_length: u64,
        ///Fishermen stake threshold.
        pub fishermen_threshold: ::std::string::String,
        ///Initial gas limit.
        pub gas_limit: u64,
        ///Gas price adjustment rate
        pub gas_price_adjustment_rate: Rational32SchemaProvider,
        ///Height of genesis block.
        pub genesis_height: u64,
        ///Official time of blockchain start.
        pub genesis_time: chrono::DateTime<chrono::offset::Utc>,
        pub max_gas_price: ::std::string::String,
        ///Maximum inflation on the total supply every epoch.
        pub max_inflation_rate: Rational32SchemaProvider,
        ///Max stake percentage of the validators we will kick out.
        #[serde(default = "defaults::default_u64::<u8, 100>")]
        pub max_kickout_stake_perc: u8,
        ///Minimum gas price. It is also the initial gas price.
        pub min_gas_price: ::std::string::String,
        ///The minimum stake required for staking is last seat price divided by
        /// this number.
        #[serde(default = "defaults::default_u64::<u64, 10>")]
        pub minimum_stake_divisor: u64,
        ///The lowest ratio s/s_total any block producer can have. See <https://github.com/near/NEPs/pull/167> for details
        #[serde(default = "defaults::genesis_config_minimum_stake_ratio")]
        pub minimum_stake_ratio: Rational32SchemaProvider,
        ///The minimum number of validators each shard must have
        #[serde(default = "defaults::default_u64::<u64, 1>")]
        pub minimum_validators_per_shard: u64,
        ///Number of block producer seats at genesis.
        pub num_block_producer_seats: u64,
        ///Defines number of shards and number of block producer seats per each
        /// shard at genesis. Note: not used with
        /// protocol_feature_chunk_only_producers -- replaced by
        /// minimum_validators_per_shard Note: not used before as all block
        /// producers produce chunks for all shards
        pub num_block_producer_seats_per_shard: ::std::vec::Vec<u64>,
        ///Expected number of blocks per year
        pub num_blocks_per_year: u64,
        ///Deprecated.
        #[serde(default = "defaults::default_u64::<u64, 300>")]
        pub num_chunk_only_producer_seats: u64,
        ///Number of chunk producers. Don't mess it up with chunk-only
        /// producers feature which is deprecated.
        #[serde(default = "defaults::default_u64::<u64, 100>")]
        pub num_chunk_producer_seats: u64,
        #[serde(default = "defaults::default_u64::<u64, 300>")]
        pub num_chunk_validator_seats: u64,
        ///Online maximum threshold above which validator gets full reward.
        #[serde(default = "defaults::genesis_config_online_max_threshold")]
        pub online_max_threshold: Rational32SchemaProvider,
        ///Online minimum threshold below which validator doesn't receive
        /// reward.
        #[serde(default = "defaults::genesis_config_online_min_threshold")]
        pub online_min_threshold: Rational32SchemaProvider,
        ///Protocol treasury rate
        pub protocol_reward_rate: Rational32SchemaProvider,
        ///Protocol treasury account
        pub protocol_treasury_account: AccountId,
        ///Threshold of stake that needs to indicate that they ready for
        /// upgrade.
        #[serde(default = "defaults::genesis_config_protocol_upgrade_stake_threshold")]
        pub protocol_upgrade_stake_threshold: Rational32SchemaProvider,
        ///Protocol version that this genesis works with.
        pub protocol_version: u32,
        ///Layout information regarding how to split accounts to shards
        #[serde(default = "defaults::genesis_config_shard_layout")]
        pub shard_layout: ShardLayout,
        ///If true, shuffle the chunk producers across shards. In other words,
        /// if the shard assignments were `[S_0, S_1, S_2, S_3]` where `S_i`
        /// represents the set of chunk producers for shard `i`, if this flag
        /// were true, the shard assignments might become, for example, `[S_2,
        /// S_0, S_3, S_1]`.
        #[serde(default)]
        pub shuffle_shard_assignment_for_chunk_producers: bool,
        ///Number of target chunk validator mandates for each shard.
        #[serde(default = "defaults::default_u64::<u64, 68>")]
        pub target_validator_mandates_per_shard: u64,
        ///Total supply of tokens at genesis.
        pub total_supply: ::std::string::String,
        ///Number of blocks for which a given transaction is valid
        pub transaction_validity_period: u64,
        ///This is only for test purposes. We hard code some configs for
        /// mainnet and testnet in AllEpochConfig, and we want to have a way to
        /// test that code path. This flag is for that. If set to true, the node
        /// will use the same config override path as mainnet and testnet.
        #[serde(default)]
        pub use_production_config: bool,
        ///List of initial validators.
        pub validators: ::std::vec::Vec<AccountInfo>,
    }

    impl ::std::convert::From<&GenesisConfig> for GenesisConfig {
        fn from(value: &GenesisConfig) -> Self {
            value.clone()
        }
    }

    ///GlobalContractData
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "code",
    ///    "id"
    ///  ],
    ///  "properties": {
    ///    "code": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "$ref": "#/components/schemas/GlobalContractIdentifier"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GlobalContractData {
        pub code: ::std::string::String,
        pub id: GlobalContractIdentifier,
    }

    impl ::std::convert::From<&GlobalContractData> for GlobalContractData {
        fn from(value: &GlobalContractData) -> Self {
            value.clone()
        }
    }

    ///GlobalContractDeployMode
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "description": "Contract is deployed under its code hash. Users
    /// will be able reference it by that hash. This effectively makes the
    /// contract immutable.",
    ///      "type": "string",
    ///      "enum": [
    ///        "CodeHash"
    ///      ]
    ///    },
    ///    {
    ///      "description": "Contract is deployed under the owner account id.
    /// Users will be able reference it by that account id. This allows the
    /// owner to update the contract for all its users.",
    ///      "type": "string",
    ///      "enum": [
    ///        "AccountId"
    ///      ]
    ///    }
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
    pub enum GlobalContractDeployMode {
        ///Contract is deployed under its code hash. Users will be able
        /// reference it by that hash. This effectively makes the contract
        /// immutable.
        CodeHash,
        ///Contract is deployed under the owner account id. Users will be able
        /// reference it by that account id. This allows the owner to update the
        /// contract for all its users.
        AccountId,
    }

    impl ::std::convert::From<&Self> for GlobalContractDeployMode {
        fn from(value: &GlobalContractDeployMode) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for GlobalContractDeployMode {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::CodeHash => write!(f, "CodeHash"),
                Self::AccountId => write!(f, "AccountId"),
            }
        }
    }

    impl ::std::str::FromStr for GlobalContractDeployMode {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "CodeHash" => Ok(Self::CodeHash),
                "AccountId" => Ok(Self::AccountId),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for GlobalContractDeployMode {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for GlobalContractDeployMode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GlobalContractDeployMode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///GlobalContractIdentifier
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "CodeHash"
    ///      ],
    ///      "properties": {
    ///        "CodeHash": {
    ///          "$ref": "#/components/schemas/CryptoHash"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "AccountId"
    ///      ],
    ///      "properties": {
    ///        "AccountId": {
    ///          "$ref": "#/components/schemas/AccountId"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub enum GlobalContractIdentifier {
        CodeHash(CryptoHash),
        AccountId(AccountId),
    }

    impl ::std::convert::From<&Self> for GlobalContractIdentifier {
        fn from(value: &GlobalContractIdentifier) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<CryptoHash> for GlobalContractIdentifier {
        fn from(value: CryptoHash) -> Self {
            Self::CodeHash(value)
        }
    }

    impl ::std::convert::From<AccountId> for GlobalContractIdentifier {
        fn from(value: AccountId) -> Self {
            Self::AccountId(value)
        }
    }

    ///HealthMethodNameHelperEnum
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "health"
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
    pub enum HealthMethodNameHelperEnum {
        #[serde(rename = "health")]
        Health,
    }

    impl ::std::convert::From<&Self> for HealthMethodNameHelperEnum {
        fn from(value: &HealthMethodNameHelperEnum) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for HealthMethodNameHelperEnum {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Health => write!(f, "health"),
            }
        }
    }

    impl ::std::str::FromStr for HealthMethodNameHelperEnum {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "health" => Ok(Self::Health),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for HealthMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for HealthMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for HealthMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///HostError
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "description": "String encoding is bad UTF-16 sequence",
    ///      "type": "string",
    ///      "enum": [
    ///        "BadUTF16"
    ///      ]
    ///    },
    ///    {
    ///      "description": "String encoding is bad UTF-8 sequence",
    ///      "type": "string",
    ///      "enum": [
    ///        "BadUTF8"
    ///      ]
    ///    },
    ///    {
    ///      "description": "Exceeded the prepaid gas",
    ///      "type": "string",
    ///      "enum": [
    ///        "GasExceeded"
    ///      ]
    ///    },
    ///    {
    ///      "description": "Exceeded the maximum amount of gas allowed to burn
    /// per contract",
    ///      "type": "string",
    ///      "enum": [
    ///        "GasLimitExceeded"
    ///      ]
    ///    },
    ///    {
    ///      "description": "Exceeded the account balance",
    ///      "type": "string",
    ///      "enum": [
    ///        "BalanceExceeded"
    ///      ]
    ///    },
    ///    {
    ///      "description": "Tried to call an empty method name",
    ///      "type": "string",
    ///      "enum": [
    ///        "EmptyMethodName"
    ///      ]
    ///    },
    ///    {
    ///      "description": "Smart contract panicked",
    ///      "type": "object",
    ///      "required": [
    ///        "GuestPanic"
    ///      ],
    ///      "properties": {
    ///        "GuestPanic": {
    ///          "type": "object",
    ///          "required": [
    ///            "panic_msg"
    ///          ],
    ///          "properties": {
    ///            "panic_msg": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "IntegerOverflow happened during a contract
    /// execution",
    ///      "type": "string",
    ///      "enum": [
    ///        "IntegerOverflow"
    ///      ]
    ///    },
    ///    {
    ///      "description": "`promise_idx` does not correspond to existing
    /// promises",
    ///      "type": "object",
    ///      "required": [
    ///        "InvalidPromiseIndex"
    ///      ],
    ///      "properties": {
    ///        "InvalidPromiseIndex": {
    ///          "type": "object",
    ///          "required": [
    ///            "promise_idx"
    ///          ],
    ///          "properties": {
    ///            "promise_idx": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "Actions can only be appended to non-joint
    /// promise.",
    ///      "type": "string",
    ///      "enum": [
    ///        "CannotAppendActionToJointPromise"
    ///      ]
    ///    },
    ///    {
    ///      "description": "Returning joint promise is currently prohibited",
    ///      "type": "string",
    ///      "enum": [
    ///        "CannotReturnJointPromise"
    ///      ]
    ///    },
    ///    {
    ///      "description": "Accessed invalid promise result index",
    ///      "type": "object",
    ///      "required": [
    ///        "InvalidPromiseResultIndex"
    ///      ],
    ///      "properties": {
    ///        "InvalidPromiseResultIndex": {
    ///          "type": "object",
    ///          "required": [
    ///            "result_idx"
    ///          ],
    ///          "properties": {
    ///            "result_idx": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "Accessed invalid register id",
    ///      "type": "object",
    ///      "required": [
    ///        "InvalidRegisterId"
    ///      ],
    ///      "properties": {
    ///        "InvalidRegisterId": {
    ///          "type": "object",
    ///          "required": [
    ///            "register_id"
    ///          ],
    ///          "properties": {
    ///            "register_id": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "Iterator `iterator_index` was invalidated after its
    /// creation by performing a mutable operation on trie",
    ///      "type": "object",
    ///      "required": [
    ///        "IteratorWasInvalidated"
    ///      ],
    ///      "properties": {
    ///        "IteratorWasInvalidated": {
    ///          "type": "object",
    ///          "required": [
    ///            "iterator_index"
    ///          ],
    ///          "properties": {
    ///            "iterator_index": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "Accessed memory outside the bounds",
    ///      "type": "string",
    ///      "enum": [
    ///        "MemoryAccessViolation"
    ///      ]
    ///    },
    ///    {
    ///      "description": "VM Logic returned an invalid receipt index",
    ///      "type": "object",
    ///      "required": [
    ///        "InvalidReceiptIndex"
    ///      ],
    ///      "properties": {
    ///        "InvalidReceiptIndex": {
    ///          "type": "object",
    ///          "required": [
    ///            "receipt_index"
    ///          ],
    ///          "properties": {
    ///            "receipt_index": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "Iterator index `iterator_index` does not exist",
    ///      "type": "object",
    ///      "required": [
    ///        "InvalidIteratorIndex"
    ///      ],
    ///      "properties": {
    ///        "InvalidIteratorIndex": {
    ///          "type": "object",
    ///          "required": [
    ///            "iterator_index"
    ///          ],
    ///          "properties": {
    ///            "iterator_index": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "VM Logic returned an invalid account id",
    ///      "type": "string",
    ///      "enum": [
    ///        "InvalidAccountId"
    ///      ]
    ///    },
    ///    {
    ///      "description": "VM Logic returned an invalid method name",
    ///      "type": "string",
    ///      "enum": [
    ///        "InvalidMethodName"
    ///      ]
    ///    },
    ///    {
    ///      "description": "VM Logic provided an invalid public key",
    ///      "type": "string",
    ///      "enum": [
    ///        "InvalidPublicKey"
    ///      ]
    ///    },
    ///    {
    ///      "description": "`method_name` is not allowed in view calls",
    ///      "type": "object",
    ///      "required": [
    ///        "ProhibitedInView"
    ///      ],
    ///      "properties": {
    ///        "ProhibitedInView": {
    ///          "type": "object",
    ///          "required": [
    ///            "method_name"
    ///          ],
    ///          "properties": {
    ///            "method_name": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "The total number of logs will exceed the limit.",
    ///      "type": "object",
    ///      "required": [
    ///        "NumberOfLogsExceeded"
    ///      ],
    ///      "properties": {
    ///        "NumberOfLogsExceeded": {
    ///          "type": "object",
    ///          "required": [
    ///            "limit"
    ///          ],
    ///          "properties": {
    ///            "limit": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "The storage key length exceeded the limit.",
    ///      "type": "object",
    ///      "required": [
    ///        "KeyLengthExceeded"
    ///      ],
    ///      "properties": {
    ///        "KeyLengthExceeded": {
    ///          "type": "object",
    ///          "required": [
    ///            "length",
    ///            "limit"
    ///          ],
    ///          "properties": {
    ///            "length": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            },
    ///            "limit": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "The storage value length exceeded the limit.",
    ///      "type": "object",
    ///      "required": [
    ///        "ValueLengthExceeded"
    ///      ],
    ///      "properties": {
    ///        "ValueLengthExceeded": {
    ///          "type": "object",
    ///          "required": [
    ///            "length",
    ///            "limit"
    ///          ],
    ///          "properties": {
    ///            "length": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            },
    ///            "limit": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "The total log length exceeded the limit.",
    ///      "type": "object",
    ///      "required": [
    ///        "TotalLogLengthExceeded"
    ///      ],
    ///      "properties": {
    ///        "TotalLogLengthExceeded": {
    ///          "type": "object",
    ///          "required": [
    ///            "length",
    ///            "limit"
    ///          ],
    ///          "properties": {
    ///            "length": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            },
    ///            "limit": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "The maximum number of promises within a
    /// FunctionCall exceeded the limit.",
    ///      "type": "object",
    ///      "required": [
    ///        "NumberPromisesExceeded"
    ///      ],
    ///      "properties": {
    ///        "NumberPromisesExceeded": {
    ///          "type": "object",
    ///          "required": [
    ///            "limit",
    ///            "number_of_promises"
    ///          ],
    ///          "properties": {
    ///            "limit": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            },
    ///            "number_of_promises": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "The maximum number of input data dependencies
    /// exceeded the limit.",
    ///      "type": "object",
    ///      "required": [
    ///        "NumberInputDataDependenciesExceeded"
    ///      ],
    ///      "properties": {
    ///        "NumberInputDataDependenciesExceeded": {
    ///          "type": "object",
    ///          "required": [
    ///            "limit",
    ///            "number_of_input_data_dependencies"
    ///          ],
    ///          "properties": {
    ///            "limit": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            },
    ///            "number_of_input_data_dependencies": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "The returned value length exceeded the limit.",
    ///      "type": "object",
    ///      "required": [
    ///        "ReturnedValueLengthExceeded"
    ///      ],
    ///      "properties": {
    ///        "ReturnedValueLengthExceeded": {
    ///          "type": "object",
    ///          "required": [
    ///            "length",
    ///            "limit"
    ///          ],
    ///          "properties": {
    ///            "length": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            },
    ///            "limit": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "The contract size for DeployContract action
    /// exceeded the limit.",
    ///      "type": "object",
    ///      "required": [
    ///        "ContractSizeExceeded"
    ///      ],
    ///      "properties": {
    ///        "ContractSizeExceeded": {
    ///          "type": "object",
    ///          "required": [
    ///            "limit",
    ///            "size"
    ///          ],
    ///          "properties": {
    ///            "limit": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            },
    ///            "size": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "The host function was deprecated.",
    ///      "type": "object",
    ///      "required": [
    ///        "Deprecated"
    ///      ],
    ///      "properties": {
    ///        "Deprecated": {
    ///          "type": "object",
    ///          "required": [
    ///            "method_name"
    ///          ],
    ///          "properties": {
    ///            "method_name": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "General errors for ECDSA recover.",
    ///      "type": "object",
    ///      "required": [
    ///        "ECRecoverError"
    ///      ],
    ///      "properties": {
    ///        "ECRecoverError": {
    ///          "type": "object",
    ///          "required": [
    ///            "msg"
    ///          ],
    ///          "properties": {
    ///            "msg": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "Invalid input to alt_bn128 family of functions
    /// (e.g., point which isn't on the curve).",
    ///      "type": "object",
    ///      "required": [
    ///        "AltBn128InvalidInput"
    ///      ],
    ///      "properties": {
    ///        "AltBn128InvalidInput": {
    ///          "type": "object",
    ///          "required": [
    ///            "msg"
    ///          ],
    ///          "properties": {
    ///            "msg": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "Invalid input to ed25519 signature verification
    /// function (e.g. signature cannot be derived from bytes).",
    ///      "type": "object",
    ///      "required": [
    ///        "Ed25519VerifyInvalidInput"
    ///      ],
    ///      "properties": {
    ///        "Ed25519VerifyInvalidInput": {
    ///          "type": "object",
    ///          "required": [
    ///            "msg"
    ///          ],
    ///          "properties": {
    ///            "msg": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub enum HostError {
        ///String encoding is bad UTF-16 sequence
        #[serde(rename = "BadUTF16")]
        BadUtf16,
        ///String encoding is bad UTF-8 sequence
        #[serde(rename = "BadUTF8")]
        BadUtf8,
        ///Exceeded the prepaid gas
        GasExceeded,
        ///Exceeded the maximum amount of gas allowed to burn per contract
        GasLimitExceeded,
        ///Exceeded the account balance
        BalanceExceeded,
        ///Tried to call an empty method name
        EmptyMethodName,
        ///Smart contract panicked
        GuestPanic { panic_msg: ::std::string::String },
        ///IntegerOverflow happened during a contract execution
        IntegerOverflow,
        ///`promise_idx` does not correspond to existing promises
        InvalidPromiseIndex { promise_idx: u64 },
        ///Actions can only be appended to non-joint promise.
        CannotAppendActionToJointPromise,
        ///Returning joint promise is currently prohibited
        CannotReturnJointPromise,
        ///Accessed invalid promise result index
        InvalidPromiseResultIndex { result_idx: u64 },
        ///Accessed invalid register id
        InvalidRegisterId { register_id: u64 },
        ///Iterator `iterator_index` was invalidated after its creation by
        /// performing a mutable operation on trie
        IteratorWasInvalidated { iterator_index: u64 },
        ///Accessed memory outside the bounds
        MemoryAccessViolation,
        ///VM Logic returned an invalid receipt index
        InvalidReceiptIndex { receipt_index: u64 },
        ///Iterator index `iterator_index` does not exist
        InvalidIteratorIndex { iterator_index: u64 },
        ///VM Logic returned an invalid account id
        InvalidAccountId,
        ///VM Logic returned an invalid method name
        InvalidMethodName,
        ///VM Logic provided an invalid public key
        InvalidPublicKey,
        ///`method_name` is not allowed in view calls
        ProhibitedInView { method_name: ::std::string::String },
        ///The total number of logs will exceed the limit.
        NumberOfLogsExceeded { limit: u64 },
        ///The storage key length exceeded the limit.
        KeyLengthExceeded { length: u64, limit: u64 },
        ///The storage value length exceeded the limit.
        ValueLengthExceeded { length: u64, limit: u64 },
        ///The total log length exceeded the limit.
        TotalLogLengthExceeded { length: u64, limit: u64 },
        ///The maximum number of promises within a FunctionCall exceeded the
        /// limit.
        NumberPromisesExceeded { limit: u64, number_of_promises: u64 },
        ///The maximum number of input data dependencies exceeded the limit.
        NumberInputDataDependenciesExceeded {
            limit: u64,
            number_of_input_data_dependencies: u64,
        },
        ///The returned value length exceeded the limit.
        ReturnedValueLengthExceeded { length: u64, limit: u64 },
        ///The contract size for DeployContract action exceeded the limit.
        ContractSizeExceeded { limit: u64, size: u64 },
        ///The host function was deprecated.
        Deprecated { method_name: ::std::string::String },
        ///General errors for ECDSA recover.
        #[serde(rename = "ECRecoverError")]
        EcRecoverError { msg: ::std::string::String },
        ///Invalid input to alt_bn128 family of functions (e.g., point which
        /// isn't on the curve).
        AltBn128InvalidInput { msg: ::std::string::String },
        ///Invalid input to ed25519 signature verification function (e.g.
        /// signature cannot be derived from bytes).
        Ed25519VerifyInvalidInput { msg: ::std::string::String },
    }

    impl ::std::convert::From<&Self> for HostError {
        fn from(value: &HostError) -> Self {
            value.clone()
        }
    }

    ///InvalidAccessKeyError
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "description": "The access key identified by the `public_key`
    /// doesn't exist for the account",
    ///      "type": "object",
    ///      "required": [
    ///        "AccessKeyNotFound"
    ///      ],
    ///      "properties": {
    ///        "AccessKeyNotFound": {
    ///          "type": "object",
    ///          "required": [
    ///            "account_id",
    ///            "public_key"
    ///          ],
    ///          "properties": {
    ///            "account_id": {
    ///              "$ref": "#/components/schemas/AccountId"
    ///            },
    ///            "public_key": {
    ///              "$ref": "#/components/schemas/PublicKey"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "Transaction `receiver_id` doesn't match the access
    /// key receiver_id",
    ///      "type": "object",
    ///      "required": [
    ///        "ReceiverMismatch"
    ///      ],
    ///      "properties": {
    ///        "ReceiverMismatch": {
    ///          "type": "object",
    ///          "required": [
    ///            "ak_receiver",
    ///            "tx_receiver"
    ///          ],
    ///          "properties": {
    ///            "ak_receiver": {
    ///              "type": "string"
    ///            },
    ///            "tx_receiver": {
    ///              "$ref": "#/components/schemas/AccountId"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "Transaction method name isn't allowed by the access
    /// key",
    ///      "type": "object",
    ///      "required": [
    ///        "MethodNameMismatch"
    ///      ],
    ///      "properties": {
    ///        "MethodNameMismatch": {
    ///          "type": "object",
    ///          "required": [
    ///            "method_name"
    ///          ],
    ///          "properties": {
    ///            "method_name": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "Transaction requires a full permission access
    /// key.",
    ///      "type": "string",
    ///      "enum": [
    ///        "RequiresFullAccess"
    ///      ]
    ///    },
    ///    {
    ///      "description": "Access Key does not have enough allowance to cover
    /// transaction cost",
    ///      "type": "object",
    ///      "required": [
    ///        "NotEnoughAllowance"
    ///      ],
    ///      "properties": {
    ///        "NotEnoughAllowance": {
    ///          "type": "object",
    ///          "required": [
    ///            "account_id",
    ///            "allowance",
    ///            "cost",
    ///            "public_key"
    ///          ],
    ///          "properties": {
    ///            "account_id": {
    ///              "$ref": "#/components/schemas/AccountId"
    ///            },
    ///            "allowance": {
    ///              "type": "string"
    ///            },
    ///            "cost": {
    ///              "type": "string"
    ///            },
    ///            "public_key": {
    ///              "$ref": "#/components/schemas/PublicKey"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "Having a deposit with a function call action is not
    /// allowed with a function call access key.",
    ///      "type": "string",
    ///      "enum": [
    ///        "DepositWithFunctionCall"
    ///      ]
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub enum InvalidAccessKeyError {
        ///The access key identified by the `public_key` doesn't exist for the
        /// account
        AccessKeyNotFound {
            account_id: AccountId,
            public_key: PublicKey,
        },
        ///Transaction `receiver_id` doesn't match the access key receiver_id
        ReceiverMismatch {
            ak_receiver: ::std::string::String,
            tx_receiver: AccountId,
        },
        ///Transaction method name isn't allowed by the access key
        MethodNameMismatch { method_name: ::std::string::String },
        ///Transaction requires a full permission access key.
        RequiresFullAccess,
        ///Access Key does not have enough allowance to cover transaction cost
        NotEnoughAllowance {
            account_id: AccountId,
            allowance: ::std::string::String,
            cost: ::std::string::String,
            public_key: PublicKey,
        },
        ///Having a deposit with a function call action is not allowed with a
        /// function call access key.
        DepositWithFunctionCall,
    }

    impl ::std::convert::From<&Self> for InvalidAccessKeyError {
        fn from(value: &InvalidAccessKeyError) -> Self {
            value.clone()
        }
    }

    ///An error happened during TX execution
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "An error happened during TX execution",
    ///  "oneOf": [
    ///    {
    ///      "description": "Happens if a wrong AccessKey used or AccessKey has
    /// not enough permissions",
    ///      "type": "object",
    ///      "required": [
    ///        "InvalidAccessKeyError"
    ///      ],
    ///      "properties": {
    ///        "InvalidAccessKeyError": {
    ///          "$ref": "#/components/schemas/InvalidAccessKeyError"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "TX signer_id is not a valid [`AccountId`]",
    ///      "type": "object",
    ///      "required": [
    ///        "InvalidSignerId"
    ///      ],
    ///      "properties": {
    ///        "InvalidSignerId": {
    ///          "type": "object",
    ///          "required": [
    ///            "signer_id"
    ///          ],
    ///          "properties": {
    ///            "signer_id": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "TX signer_id is not found in a storage",
    ///      "type": "object",
    ///      "required": [
    ///        "SignerDoesNotExist"
    ///      ],
    ///      "properties": {
    ///        "SignerDoesNotExist": {
    ///          "type": "object",
    ///          "required": [
    ///            "signer_id"
    ///          ],
    ///          "properties": {
    ///            "signer_id": {
    ///              "$ref": "#/components/schemas/AccountId"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "Transaction nonce must be
    /// `account[access_key].nonce + 1`.",
    ///      "type": "object",
    ///      "required": [
    ///        "InvalidNonce"
    ///      ],
    ///      "properties": {
    ///        "InvalidNonce": {
    ///          "type": "object",
    ///          "required": [
    ///            "ak_nonce",
    ///            "tx_nonce"
    ///          ],
    ///          "properties": {
    ///            "ak_nonce": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            },
    ///            "tx_nonce": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "Transaction nonce is larger than the upper bound
    /// given by the block height",
    ///      "type": "object",
    ///      "required": [
    ///        "NonceTooLarge"
    ///      ],
    ///      "properties": {
    ///        "NonceTooLarge": {
    ///          "type": "object",
    ///          "required": [
    ///            "tx_nonce",
    ///            "upper_bound"
    ///          ],
    ///          "properties": {
    ///            "tx_nonce": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            },
    ///            "upper_bound": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "TX receiver_id is not a valid AccountId",
    ///      "type": "object",
    ///      "required": [
    ///        "InvalidReceiverId"
    ///      ],
    ///      "properties": {
    ///        "InvalidReceiverId": {
    ///          "type": "object",
    ///          "required": [
    ///            "receiver_id"
    ///          ],
    ///          "properties": {
    ///            "receiver_id": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "TX signature is not valid",
    ///      "type": "string",
    ///      "enum": [
    ///        "InvalidSignature"
    ///      ]
    ///    },
    ///    {
    ///      "description": "Account does not have enough balance to cover TX
    /// cost",
    ///      "type": "object",
    ///      "required": [
    ///        "NotEnoughBalance"
    ///      ],
    ///      "properties": {
    ///        "NotEnoughBalance": {
    ///          "type": "object",
    ///          "required": [
    ///            "balance",
    ///            "cost",
    ///            "signer_id"
    ///          ],
    ///          "properties": {
    ///            "balance": {
    ///              "type": "string"
    ///            },
    ///            "cost": {
    ///              "type": "string"
    ///            },
    ///            "signer_id": {
    ///              "$ref": "#/components/schemas/AccountId"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "Signer account doesn't have enough balance after
    /// transaction.",
    ///      "type": "object",
    ///      "required": [
    ///        "LackBalanceForState"
    ///      ],
    ///      "properties": {
    ///        "LackBalanceForState": {
    ///          "type": "object",
    ///          "required": [
    ///            "amount",
    ///            "signer_id"
    ///          ],
    ///          "properties": {
    ///            "amount": {
    ///              "description": "Required balance to cover the state.",
    ///              "type": "string"
    ///            },
    ///            "signer_id": {
    ///              "description": "An account which doesn't have enough
    /// balance to cover storage.",
    ///              "allOf": [
    ///                {
    ///                  "$ref": "#/components/schemas/AccountId"
    ///                }
    ///              ]
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "An integer overflow occurred during transaction
    /// cost estimation.",
    ///      "type": "string",
    ///      "enum": [
    ///        "CostOverflow"
    ///      ]
    ///    },
    ///    {
    ///      "description": "Transaction parent block hash doesn't belong to the
    /// current chain",
    ///      "type": "string",
    ///      "enum": [
    ///        "InvalidChain"
    ///      ]
    ///    },
    ///    {
    ///      "description": "Transaction has expired",
    ///      "type": "string",
    ///      "enum": [
    ///        "Expired"
    ///      ]
    ///    },
    ///    {
    ///      "description": "An error occurred while validating actions of a
    /// Transaction.",
    ///      "type": "object",
    ///      "required": [
    ///        "ActionsValidation"
    ///      ],
    ///      "properties": {
    ///        "ActionsValidation": {
    ///          "$ref": "#/components/schemas/ActionsValidationError"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "The size of serialized transaction exceeded the
    /// limit.",
    ///      "type": "object",
    ///      "required": [
    ///        "TransactionSizeExceeded"
    ///      ],
    ///      "properties": {
    ///        "TransactionSizeExceeded": {
    ///          "type": "object",
    ///          "required": [
    ///            "limit",
    ///            "size"
    ///          ],
    ///          "properties": {
    ///            "limit": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            },
    ///            "size": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "Transaction version is invalid.",
    ///      "type": "string",
    ///      "enum": [
    ///        "InvalidTransactionVersion"
    ///      ]
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "StorageError"
    ///      ],
    ///      "properties": {
    ///        "StorageError": {
    ///          "$ref": "#/components/schemas/StorageError"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "The receiver shard of the transaction is too
    /// congested to accept new transactions at the moment.",
    ///      "type": "object",
    ///      "required": [
    ///        "ShardCongested"
    ///      ],
    ///      "properties": {
    ///        "ShardCongested": {
    ///          "type": "object",
    ///          "required": [
    ///            "congestion_level",
    ///            "shard_id"
    ///          ],
    ///          "properties": {
    ///            "congestion_level": {
    ///              "description": "A value between 0 (no congestion) and 1
    /// (max congestion).",
    ///              "type": "number",
    ///              "format": "double"
    ///            },
    ///            "shard_id": {
    ///              "description": "The congested shard.",
    ///              "type": "integer",
    ///              "format": "uint32",
    ///              "minimum": 0.0
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "The receiver shard of the transaction missed
    /// several chunks and rejects new transaction until it can make progress
    /// again.",
    ///      "type": "object",
    ///      "required": [
    ///        "ShardStuck"
    ///      ],
    ///      "properties": {
    ///        "ShardStuck": {
    ///          "type": "object",
    ///          "required": [
    ///            "missed_chunks",
    ///            "shard_id"
    ///          ],
    ///          "properties": {
    ///            "missed_chunks": {
    ///              "description": "The number of blocks since the last
    /// included chunk of the shard.",
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            },
    ///            "shard_id": {
    ///              "description": "The shard that fails making progress.",
    ///              "type": "integer",
    ///              "format": "uint32",
    ///              "minimum": 0.0
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub enum InvalidTxError {
        ///Happens if a wrong AccessKey used or AccessKey has not enough
        /// permissions
        InvalidAccessKeyError(InvalidAccessKeyError),
        ///TX signer_id is not a valid [`AccountId`]
        InvalidSignerId {
            signer_id: ::std::string::String,
        },
        ///TX signer_id is not found in a storage
        SignerDoesNotExist {
            signer_id: AccountId,
        },
        ///Transaction nonce must be `account[access_key].nonce + 1`.
        InvalidNonce {
            ak_nonce: u64,
            tx_nonce: u64,
        },
        ///Transaction nonce is larger than the upper bound given by the block
        /// height
        NonceTooLarge {
            tx_nonce: u64,
            upper_bound: u64,
        },
        ///TX receiver_id is not a valid AccountId
        InvalidReceiverId {
            receiver_id: ::std::string::String,
        },
        ///TX signature is not valid
        InvalidSignature,
        ///Account does not have enough balance to cover TX cost
        NotEnoughBalance {
            balance: ::std::string::String,
            cost: ::std::string::String,
            signer_id: AccountId,
        },
        ///Signer account doesn't have enough balance after transaction.
        LackBalanceForState {
            ///Required balance to cover the state.
            amount: ::std::string::String,
            ///An account which doesn't have enough balance to cover storage.
            signer_id: AccountId,
        },
        ///An integer overflow occurred during transaction cost estimation.
        CostOverflow,
        ///Transaction parent block hash doesn't belong to the current chain
        InvalidChain,
        ///Transaction has expired
        Expired,
        ///An error occurred while validating actions of a Transaction.
        ActionsValidation(ActionsValidationError),
        ///The size of serialized transaction exceeded the limit.
        TransactionSizeExceeded {
            limit: u64,
            size: u64,
        },
        ///Transaction version is invalid.
        InvalidTransactionVersion,
        StorageError(StorageError),
        ///The receiver shard of the transaction is too congested to accept new
        /// transactions at the moment.
        ShardCongested {
            congestion_level: f64,
            ///The congested shard.
            shard_id: u32,
        },
        ///The receiver shard of the transaction missed several chunks and
        /// rejects new transaction until it can make progress again.
        ShardStuck {
            ///The number of blocks since the last included chunk of the shard.
            missed_chunks: u64,
            ///The shard that fails making progress.
            shard_id: u32,
        },
    }

    impl ::std::convert::From<&Self> for InvalidTxError {
        fn from(value: &InvalidTxError) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<InvalidAccessKeyError> for InvalidTxError {
        fn from(value: InvalidAccessKeyError) -> Self {
            Self::InvalidAccessKeyError(value)
        }
    }

    impl ::std::convert::From<ActionsValidationError> for InvalidTxError {
        fn from(value: ActionsValidationError) -> Self {
            Self::ActionsValidation(value)
        }
    }

    impl ::std::convert::From<StorageError> for InvalidTxError {
        fn from(value: StorageError) -> Self {
            Self::StorageError(value)
        }
    }

    ///JsonRpcRequestForBlockMethodNameHelperEnum
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "JsonRpcRequest_for_BlockMethodNameHelperEnum",
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
    ///      "$ref": "#/components/schemas/BlockMethodNameHelperEnum"
    ///    },
    ///    "params": {
    ///      "$ref": "#/components/schemas/RpcBlockRequest"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct JsonRpcRequestForBlockMethodNameHelperEnum {
        pub id: ::std::string::String,
        pub jsonrpc: ::std::string::String,
        pub method: BlockMethodNameHelperEnum,
        pub params: RpcBlockRequest,
    }

    impl ::std::convert::From<&JsonRpcRequestForBlockMethodNameHelperEnum>
        for JsonRpcRequestForBlockMethodNameHelperEnum
    {
        fn from(value: &JsonRpcRequestForBlockMethodNameHelperEnum) -> Self {
            value.clone()
        }
    }

    ///JsonRpcRequestForBroadCastTxAsyncMethodNameHelperEnum
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "JsonRpcRequest_for_BroadCastTxAsyncMethodNameHelperEnum",
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
    ///      "$ref": "#/components/schemas/BroadCastTxAsyncMethodNameHelperEnum"
    ///    },
    ///    "params": {
    ///      "$ref": "#/components/schemas/RpcSendTransactionRequest"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct JsonRpcRequestForBroadCastTxAsyncMethodNameHelperEnum {
        pub id: ::std::string::String,
        pub jsonrpc: ::std::string::String,
        pub method: BroadCastTxAsyncMethodNameHelperEnum,
        pub params: RpcSendTransactionRequest,
    }

    impl ::std::convert::From<&JsonRpcRequestForBroadCastTxAsyncMethodNameHelperEnum>
        for JsonRpcRequestForBroadCastTxAsyncMethodNameHelperEnum
    {
        fn from(value: &JsonRpcRequestForBroadCastTxAsyncMethodNameHelperEnum) -> Self {
            value.clone()
        }
    }

    ///JsonRpcRequestForBroadCastTxCommitMethodNameHelperEnum
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "JsonRpcRequest_for_BroadCastTxCommitMethodNameHelperEnum",
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
    ///      "$ref":
    /// "#/components/schemas/BroadCastTxCommitMethodNameHelperEnum"
    ///    },
    ///    "params": {
    ///      "$ref": "#/components/schemas/RpcSendTransactionRequest"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct JsonRpcRequestForBroadCastTxCommitMethodNameHelperEnum {
        pub id: ::std::string::String,
        pub jsonrpc: ::std::string::String,
        pub method: BroadCastTxCommitMethodNameHelperEnum,
        pub params: RpcSendTransactionRequest,
    }

    impl ::std::convert::From<&JsonRpcRequestForBroadCastTxCommitMethodNameHelperEnum>
        for JsonRpcRequestForBroadCastTxCommitMethodNameHelperEnum
    {
        fn from(value: &JsonRpcRequestForBroadCastTxCommitMethodNameHelperEnum) -> Self {
            value.clone()
        }
    }

    ///JsonRpcRequestForClientConfigMethodNameHelperEnum
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "JsonRpcRequest_for_ClientConfigMethodNameHelperEnum",
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
    ///      "$ref": "#/components/schemas/ClientConfigMethodNameHelperEnum"
    ///    },
    ///    "params": {
    ///      "$ref": "#/components/schemas/RpcClientConfigRequest"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct JsonRpcRequestForClientConfigMethodNameHelperEnum {
        pub id: ::std::string::String,
        pub jsonrpc: ::std::string::String,
        pub method: ClientConfigMethodNameHelperEnum,
        pub params: RpcClientConfigRequest,
    }

    impl ::std::convert::From<&JsonRpcRequestForClientConfigMethodNameHelperEnum>
        for JsonRpcRequestForClientConfigMethodNameHelperEnum
    {
        fn from(value: &JsonRpcRequestForClientConfigMethodNameHelperEnum) -> Self {
            value.clone()
        }
    }

    ///JsonRpcRequestForExpChangeMethodNameHelperEnum
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "JsonRpcRequest_for_ExpChangeMethodNameHelperEnum",
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
    ///      "$ref": "#/components/schemas/ExpChangeMethodNameHelperEnum"
    ///    },
    ///    "params": {
    ///      "$ref": "#/components/schemas/RpcClientConfigRequest"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct JsonRpcRequestForExpChangeMethodNameHelperEnum {
        pub id: ::std::string::String,
        pub jsonrpc: ::std::string::String,
        pub method: ExpChangeMethodNameHelperEnum,
        pub params: RpcClientConfigRequest,
    }

    impl ::std::convert::From<&JsonRpcRequestForExpChangeMethodNameHelperEnum>
        for JsonRpcRequestForExpChangeMethodNameHelperEnum
    {
        fn from(value: &JsonRpcRequestForExpChangeMethodNameHelperEnum) -> Self {
            value.clone()
        }
    }

    ///JsonRpcRequestForExpChangesBlockMethodNameHelperEnum
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "JsonRpcRequest_for_ExpChangesBlockMethodNameHelperEnum",
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
    ///      "$ref": "#/components/schemas/ExpChangesBlockMethodNameHelperEnum"
    ///    },
    ///    "params": {
    ///      "$ref": "#/components/schemas/RpcClientConfigRequest"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct JsonRpcRequestForExpChangesBlockMethodNameHelperEnum {
        pub id: ::std::string::String,
        pub jsonrpc: ::std::string::String,
        pub method: ExpChangesBlockMethodNameHelperEnum,
        pub params: RpcClientConfigRequest,
    }

    impl ::std::convert::From<&JsonRpcRequestForExpChangesBlockMethodNameHelperEnum>
        for JsonRpcRequestForExpChangesBlockMethodNameHelperEnum
    {
        fn from(value: &JsonRpcRequestForExpChangesBlockMethodNameHelperEnum) -> Self {
            value.clone()
        }
    }

    ///JsonRpcRequestForExpGenesisMethodNameHelperEnum
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "JsonRpcRequest_for_ExpGenesisMethodNameHelperEnum",
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
    ///      "$ref": "#/components/schemas/ExpGenesisMethodNameHelperEnum"
    ///    },
    ///    "params": {
    ///      "$ref": "#/components/schemas/RpcClientConfigRequest"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct JsonRpcRequestForExpGenesisMethodNameHelperEnum {
        pub id: ::std::string::String,
        pub jsonrpc: ::std::string::String,
        pub method: ExpGenesisMethodNameHelperEnum,
        pub params: RpcClientConfigRequest,
    }

    impl ::std::convert::From<&JsonRpcRequestForExpGenesisMethodNameHelperEnum>
        for JsonRpcRequestForExpGenesisMethodNameHelperEnum
    {
        fn from(value: &JsonRpcRequestForExpGenesisMethodNameHelperEnum) -> Self {
            value.clone()
        }
    }

    ///JsonRpcRequestForExpGongestionMethodNameHelperEnum
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "JsonRpcRequest_for_ExpGongestionMethodNameHelperEnum",
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
    ///      "$ref": "#/components/schemas/ExpGongestionMethodNameHelperEnum"
    ///    },
    ///    "params": {
    ///      "$ref": "#/components/schemas/RpcClientConfigRequest"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct JsonRpcRequestForExpGongestionMethodNameHelperEnum {
        pub id: ::std::string::String,
        pub jsonrpc: ::std::string::String,
        pub method: ExpGongestionMethodNameHelperEnum,
        pub params: RpcClientConfigRequest,
    }

    impl ::std::convert::From<&JsonRpcRequestForExpGongestionMethodNameHelperEnum>
        for JsonRpcRequestForExpGongestionMethodNameHelperEnum
    {
        fn from(value: &JsonRpcRequestForExpGongestionMethodNameHelperEnum) -> Self {
            value.clone()
        }
    }

    ///JsonRpcRequestForExpLightClientBlockProofMethodNameHelperEnum
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "
    /// JsonRpcRequest_for_ExpLightClientBlockProofMethodNameHelperEnum",
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
    ///      "$ref":
    /// "#/components/schemas/ExpLightClientBlockProofMethodNameHelperEnum"
    ///    },
    ///    "params": {
    ///      "$ref": "#/components/schemas/RpcClientConfigRequest"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct JsonRpcRequestForExpLightClientBlockProofMethodNameHelperEnum {
        pub id: ::std::string::String,
        pub jsonrpc: ::std::string::String,
        pub method: ExpLightClientBlockProofMethodNameHelperEnum,
        pub params: RpcClientConfigRequest,
    }

    impl ::std::convert::From<&JsonRpcRequestForExpLightClientBlockProofMethodNameHelperEnum>
        for JsonRpcRequestForExpLightClientBlockProofMethodNameHelperEnum
    {
        fn from(value: &JsonRpcRequestForExpLightClientBlockProofMethodNameHelperEnum) -> Self {
            value.clone()
        }
    }

    ///JsonRpcRequestForExpLightClientProofMethodNameHelperEnum
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "JsonRpcRequest_for_ExpLightClientProofMethodNameHelperEnum",
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
    ///      "$ref":
    /// "#/components/schemas/ExpLightClientProofMethodNameHelperEnum"
    ///    },
    ///    "params": {
    ///      "$ref": "#/components/schemas/RpcClientConfigRequest"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct JsonRpcRequestForExpLightClientProofMethodNameHelperEnum {
        pub id: ::std::string::String,
        pub jsonrpc: ::std::string::String,
        pub method: ExpLightClientProofMethodNameHelperEnum,
        pub params: RpcClientConfigRequest,
    }

    impl ::std::convert::From<&JsonRpcRequestForExpLightClientProofMethodNameHelperEnum>
        for JsonRpcRequestForExpLightClientProofMethodNameHelperEnum
    {
        fn from(value: &JsonRpcRequestForExpLightClientProofMethodNameHelperEnum) -> Self {
            value.clone()
        }
    }

    ///JsonRpcRequestForExpProtocolConfigMethodNameHelperEnum
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "JsonRpcRequest_for_ExpProtocolConfigMethodNameHelperEnum",
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
    ///      "$ref":
    /// "#/components/schemas/ExpProtocolConfigMethodNameHelperEnum"
    ///    },
    ///    "params": {
    ///      "$ref": "#/components/schemas/RpcClientConfigRequest"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct JsonRpcRequestForExpProtocolConfigMethodNameHelperEnum {
        pub id: ::std::string::String,
        pub jsonrpc: ::std::string::String,
        pub method: ExpProtocolConfigMethodNameHelperEnum,
        pub params: RpcClientConfigRequest,
    }

    impl ::std::convert::From<&JsonRpcRequestForExpProtocolConfigMethodNameHelperEnum>
        for JsonRpcRequestForExpProtocolConfigMethodNameHelperEnum
    {
        fn from(value: &JsonRpcRequestForExpProtocolConfigMethodNameHelperEnum) -> Self {
            value.clone()
        }
    }

    ///JsonRpcRequestForExpReceiptMethodNameHelperEnum
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "JsonRpcRequest_for_ExpReceiptMethodNameHelperEnum",
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
    ///      "$ref": "#/components/schemas/ExpReceiptMethodNameHelperEnum"
    ///    },
    ///    "params": {
    ///      "$ref": "#/components/schemas/RpcClientConfigRequest"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct JsonRpcRequestForExpReceiptMethodNameHelperEnum {
        pub id: ::std::string::String,
        pub jsonrpc: ::std::string::String,
        pub method: ExpReceiptMethodNameHelperEnum,
        pub params: RpcClientConfigRequest,
    }

    impl ::std::convert::From<&JsonRpcRequestForExpReceiptMethodNameHelperEnum>
        for JsonRpcRequestForExpReceiptMethodNameHelperEnum
    {
        fn from(value: &JsonRpcRequestForExpReceiptMethodNameHelperEnum) -> Self {
            value.clone()
        }
    }

    ///JsonRpcRequestForExpTxStatusMethodNameHelperEnum
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "JsonRpcRequest_for_ExpTxStatusMethodNameHelperEnum",
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
    ///      "$ref": "#/components/schemas/ExpTxStatusMethodNameHelperEnum"
    ///    },
    ///    "params": {
    ///      "$ref": "#/components/schemas/RpcClientConfigRequest"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct JsonRpcRequestForExpTxStatusMethodNameHelperEnum {
        pub id: ::std::string::String,
        pub jsonrpc: ::std::string::String,
        pub method: ExpTxStatusMethodNameHelperEnum,
        pub params: RpcClientConfigRequest,
    }

    impl ::std::convert::From<&JsonRpcRequestForExpTxStatusMethodNameHelperEnum>
        for JsonRpcRequestForExpTxStatusMethodNameHelperEnum
    {
        fn from(value: &JsonRpcRequestForExpTxStatusMethodNameHelperEnum) -> Self {
            value.clone()
        }
    }

    ///JsonRpcRequestForExpValidatorsMethodNameHelperEnum
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "JsonRpcRequest_for_ExpValidatorsMethodNameHelperEnum",
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
    ///      "$ref": "#/components/schemas/ExpValidatorsMethodNameHelperEnum"
    ///    },
    ///    "params": {
    ///      "$ref": "#/components/schemas/RpcClientConfigRequest"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct JsonRpcRequestForExpValidatorsMethodNameHelperEnum {
        pub id: ::std::string::String,
        pub jsonrpc: ::std::string::String,
        pub method: ExpValidatorsMethodNameHelperEnum,
        pub params: RpcClientConfigRequest,
    }

    impl ::std::convert::From<&JsonRpcRequestForExpValidatorsMethodNameHelperEnum>
        for JsonRpcRequestForExpValidatorsMethodNameHelperEnum
    {
        fn from(value: &JsonRpcRequestForExpValidatorsMethodNameHelperEnum) -> Self {
            value.clone()
        }
    }

    ///JsonRpcRequestForGasPriceMethodNameHelperEnum
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "JsonRpcRequest_for_GasPriceMethodNameHelperEnum",
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
    ///      "$ref": "#/components/schemas/GasPriceMethodNameHelperEnum"
    ///    },
    ///    "params": {
    ///      "$ref": "#/components/schemas/RpcGasPriceRequest"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct JsonRpcRequestForGasPriceMethodNameHelperEnum {
        pub id: ::std::string::String,
        pub jsonrpc: ::std::string::String,
        pub method: GasPriceMethodNameHelperEnum,
        pub params: RpcGasPriceRequest,
    }

    impl ::std::convert::From<&JsonRpcRequestForGasPriceMethodNameHelperEnum>
        for JsonRpcRequestForGasPriceMethodNameHelperEnum
    {
        fn from(value: &JsonRpcRequestForGasPriceMethodNameHelperEnum) -> Self {
            value.clone()
        }
    }

    ///JsonRpcRequestForHealthMethodNameHelperEnum
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "JsonRpcRequest_for_HealthMethodNameHelperEnum",
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
    ///      "$ref": "#/components/schemas/HealthMethodNameHelperEnum"
    ///    },
    ///    "params": {
    ///      "$ref": "#/components/schemas/RpcHealthRequest"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct JsonRpcRequestForHealthMethodNameHelperEnum {
        pub id: ::std::string::String,
        pub jsonrpc: ::std::string::String,
        pub method: HealthMethodNameHelperEnum,
        pub params: RpcHealthRequest,
    }

    impl ::std::convert::From<&JsonRpcRequestForHealthMethodNameHelperEnum>
        for JsonRpcRequestForHealthMethodNameHelperEnum
    {
        fn from(value: &JsonRpcRequestForHealthMethodNameHelperEnum) -> Self {
            value.clone()
        }
    }

    ///JsonRpcRequestForLightClientProofMethodNameHelperEnum
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "JsonRpcRequest_for_LightClientProofMethodNameHelperEnum",
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
    ///      "$ref": "#/components/schemas/LightClientProofMethodNameHelperEnum"
    ///    },
    ///    "params": {
    ///      "$ref": "#/components/schemas/RpcLightClientExecutionProofRequest"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct JsonRpcRequestForLightClientProofMethodNameHelperEnum {
        pub id: ::std::string::String,
        pub jsonrpc: ::std::string::String,
        pub method: LightClientProofMethodNameHelperEnum,
        pub params: RpcLightClientExecutionProofRequest,
    }

    impl ::std::convert::From<&JsonRpcRequestForLightClientProofMethodNameHelperEnum>
        for JsonRpcRequestForLightClientProofMethodNameHelperEnum
    {
        fn from(value: &JsonRpcRequestForLightClientProofMethodNameHelperEnum) -> Self {
            value.clone()
        }
    }

    ///JsonRpcRequestForNetworkInfoMethodNameHelperEnum
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "JsonRpcRequest_for_NetworkInfoMethodNameHelperEnum",
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
    ///      "$ref": "#/components/schemas/NetworkInfoMethodNameHelperEnum"
    ///    },
    ///    "params": {
    ///      "$ref": "#/components/schemas/RpcNetworkInfoRequest"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct JsonRpcRequestForNetworkInfoMethodNameHelperEnum {
        pub id: ::std::string::String,
        pub jsonrpc: ::std::string::String,
        pub method: NetworkInfoMethodNameHelperEnum,
        pub params: RpcNetworkInfoRequest,
    }

    impl ::std::convert::From<&JsonRpcRequestForNetworkInfoMethodNameHelperEnum>
        for JsonRpcRequestForNetworkInfoMethodNameHelperEnum
    {
        fn from(value: &JsonRpcRequestForNetworkInfoMethodNameHelperEnum) -> Self {
            value.clone()
        }
    }

    ///JsonRpcRequestForNextLightClientBlockMethodNameHelperEnum
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "JsonRpcRequest_for_NextLightClientBlockMethodNameHelperEnum",
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
    ///      "$ref":
    /// "#/components/schemas/NextLightClientBlockMethodNameHelperEnum"
    ///    },
    ///    "params": {
    ///      "$ref": "#/components/schemas/RpcLightClientNextBlockRequest"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct JsonRpcRequestForNextLightClientBlockMethodNameHelperEnum {
        pub id: ::std::string::String,
        pub jsonrpc: ::std::string::String,
        pub method: NextLightClientBlockMethodNameHelperEnum,
        pub params: RpcLightClientNextBlockRequest,
    }

    impl ::std::convert::From<&JsonRpcRequestForNextLightClientBlockMethodNameHelperEnum>
        for JsonRpcRequestForNextLightClientBlockMethodNameHelperEnum
    {
        fn from(value: &JsonRpcRequestForNextLightClientBlockMethodNameHelperEnum) -> Self {
            value.clone()
        }
    }

    ///JsonRpcRequestForSendTxMethodNameHelperEnum
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "JsonRpcRequest_for_SendTxMethodNameHelperEnum",
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
    ///      "$ref": "#/components/schemas/SendTxMethodNameHelperEnum"
    ///    },
    ///    "params": {
    ///      "$ref": "#/components/schemas/RpcSendTransactionRequest"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct JsonRpcRequestForSendTxMethodNameHelperEnum {
        pub id: ::std::string::String,
        pub jsonrpc: ::std::string::String,
        pub method: SendTxMethodNameHelperEnum,
        pub params: RpcSendTransactionRequest,
    }

    impl ::std::convert::From<&JsonRpcRequestForSendTxMethodNameHelperEnum>
        for JsonRpcRequestForSendTxMethodNameHelperEnum
    {
        fn from(value: &JsonRpcRequestForSendTxMethodNameHelperEnum) -> Self {
            value.clone()
        }
    }

    ///JsonRpcRequestForStatusMethodNameHelperEnum
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "JsonRpcRequest_for_StatusMethodNameHelperEnum",
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
    ///      "$ref": "#/components/schemas/StatusMethodNameHelperEnum"
    ///    },
    ///    "params": {
    ///      "$ref": "#/components/schemas/RpcStatusRequest"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct JsonRpcRequestForStatusMethodNameHelperEnum {
        pub id: ::std::string::String,
        pub jsonrpc: ::std::string::String,
        pub method: StatusMethodNameHelperEnum,
        pub params: RpcStatusRequest,
    }

    impl ::std::convert::From<&JsonRpcRequestForStatusMethodNameHelperEnum>
        for JsonRpcRequestForStatusMethodNameHelperEnum
    {
        fn from(value: &JsonRpcRequestForStatusMethodNameHelperEnum) -> Self {
            value.clone()
        }
    }

    ///JsonRpcRequestForTxMethodNameHelperEnum
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "JsonRpcRequest_for_TxMethodNameHelperEnum",
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
    ///      "$ref": "#/components/schemas/TxMethodNameHelperEnum"
    ///    },
    ///    "params": {
    ///      "$ref": "#/components/schemas/RpcTransactionStatusRequest"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct JsonRpcRequestForTxMethodNameHelperEnum {
        pub id: ::std::string::String,
        pub jsonrpc: ::std::string::String,
        pub method: TxMethodNameHelperEnum,
        pub params: RpcTransactionStatusRequest,
    }

    impl ::std::convert::From<&JsonRpcRequestForTxMethodNameHelperEnum>
        for JsonRpcRequestForTxMethodNameHelperEnum
    {
        fn from(value: &JsonRpcRequestForTxMethodNameHelperEnum) -> Self {
            value.clone()
        }
    }

    ///JsonRpcRequestForValidatorsMethodNameHelperEnum
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "JsonRpcRequest_for_ValidatorsMethodNameHelperEnum",
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
    ///      "$ref": "#/components/schemas/ValidatorsMethodNameHelperEnum"
    ///    },
    ///    "params": {
    ///      "$ref": "#/components/schemas/RpcValidatorRequest"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct JsonRpcRequestForValidatorsMethodNameHelperEnum {
        pub id: ::std::string::String,
        pub jsonrpc: ::std::string::String,
        pub method: ValidatorsMethodNameHelperEnum,
        pub params: RpcValidatorRequest,
    }

    impl ::std::convert::From<&JsonRpcRequestForValidatorsMethodNameHelperEnum>
        for JsonRpcRequestForValidatorsMethodNameHelperEnum
    {
        fn from(value: &JsonRpcRequestForValidatorsMethodNameHelperEnum) -> Self {
            value.clone()
        }
    }

    ///JsonRpcResponseForArrayOfValidatorStakeViewAndRpcError
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "JsonRpcResponse_for_Array_of_ValidatorStakeView_and_RpcError"
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
    ///          "type": "array",
    ///          "items": {
    ///            "$ref": "#/components/schemas/ValidatorStakeView"
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
    pub enum JsonRpcResponseForArrayOfValidatorStakeViewAndRpcError {
        Variant0 {
            id: ::std::string::String,
            jsonrpc: ::std::string::String,
            result: ::std::vec::Vec<ValidatorStakeView>,
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

    impl ::std::convert::From<&Self> for JsonRpcResponseForArrayOfValidatorStakeViewAndRpcError {
        fn from(value: &JsonRpcResponseForArrayOfValidatorStakeViewAndRpcError) -> Self {
            value.clone()
        }
    }

    ///JsonRpcResponseForGenesisConfigAndRpcError
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "JsonRpcResponse_for_GenesisConfig_and_RpcError",
    ///  "type": "object",
    ///  "anyOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "result"
    ///      ],
    ///      "properties": {
    ///        "result": {
    ///          "$ref": "#/components/schemas/GenesisConfig"
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
    pub enum JsonRpcResponseForGenesisConfigAndRpcError {
        Variant0 {
            id: ::std::string::String,
            jsonrpc: ::std::string::String,
            result: GenesisConfig,
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

    impl ::std::convert::From<&Self> for JsonRpcResponseForGenesisConfigAndRpcError {
        fn from(value: &JsonRpcResponseForGenesisConfigAndRpcError) -> Self {
            value.clone()
        }
    }

    ///JsonRpcResponseForRpcBlockResponseAndRpcError
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "JsonRpcResponse_for_RpcBlockResponse_and_RpcError",
    ///  "type": "object",
    ///  "anyOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "result"
    ///      ],
    ///      "properties": {
    ///        "result": {
    ///          "$ref": "#/components/schemas/RpcBlockResponse"
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
    pub enum JsonRpcResponseForRpcBlockResponseAndRpcError {
        Variant0 {
            id: ::std::string::String,
            jsonrpc: ::std::string::String,
            result: RpcBlockResponse,
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

    impl ::std::convert::From<&Self> for JsonRpcResponseForRpcBlockResponseAndRpcError {
        fn from(value: &JsonRpcResponseForRpcBlockResponseAndRpcError) -> Self {
            value.clone()
        }
    }

    ///JsonRpcResponseForRpcClientConfigResponseAndRpcError
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "JsonRpcResponse_for_RpcClientConfigResponse_and_RpcError",
    ///  "type": "object",
    ///  "anyOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "result"
    ///      ],
    ///      "properties": {
    ///        "result": {
    ///          "$ref": "#/components/schemas/RpcClientConfigResponse"
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
    pub enum JsonRpcResponseForRpcClientConfigResponseAndRpcError {
        Variant0 {
            id: ::std::string::String,
            jsonrpc: ::std::string::String,
            result: RpcClientConfigResponse,
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

    impl ::std::convert::From<&Self> for JsonRpcResponseForRpcClientConfigResponseAndRpcError {
        fn from(value: &JsonRpcResponseForRpcClientConfigResponseAndRpcError) -> Self {
            value.clone()
        }
    }

    ///JsonRpcResponseForRpcCongestionLevelResponseAndRpcError
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "JsonRpcResponse_for_RpcCongestionLevelResponse_and_RpcError",
    ///  "type": "object",
    ///  "anyOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "result"
    ///      ],
    ///      "properties": {
    ///        "result": {
    ///          "$ref": "#/components/schemas/RpcCongestionLevelResponse"
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
    pub enum JsonRpcResponseForRpcCongestionLevelResponseAndRpcError {
        Variant0 {
            id: ::std::string::String,
            jsonrpc: ::std::string::String,
            result: RpcCongestionLevelResponse,
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

    impl ::std::convert::From<&Self> for JsonRpcResponseForRpcCongestionLevelResponseAndRpcError {
        fn from(value: &JsonRpcResponseForRpcCongestionLevelResponseAndRpcError) -> Self {
            value.clone()
        }
    }

    ///JsonRpcResponseForRpcGasPriceResponseAndRpcError
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "JsonRpcResponse_for_RpcGasPriceResponse_and_RpcError",
    ///  "type": "object",
    ///  "anyOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "result"
    ///      ],
    ///      "properties": {
    ///        "result": {
    ///          "$ref": "#/components/schemas/RpcGasPriceResponse"
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
    pub enum JsonRpcResponseForRpcGasPriceResponseAndRpcError {
        Variant0 {
            id: ::std::string::String,
            jsonrpc: ::std::string::String,
            result: RpcGasPriceResponse,
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

    impl ::std::convert::From<&Self> for JsonRpcResponseForRpcGasPriceResponseAndRpcError {
        fn from(value: &JsonRpcResponseForRpcGasPriceResponseAndRpcError) -> Self {
            value.clone()
        }
    }

    ///JsonRpcResponseForRpcHealthResponseAndRpcError
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "JsonRpcResponse_for_RpcHealthResponse_and_RpcError",
    ///  "type": "object",
    ///  "anyOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "result"
    ///      ],
    ///      "properties": {
    ///        "result": {
    ///          "$ref": "#/components/schemas/RpcHealthResponse"
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
    pub enum JsonRpcResponseForRpcHealthResponseAndRpcError {
        Variant0 {
            id: ::std::string::String,
            jsonrpc: ::std::string::String,
            result: RpcHealthResponse,
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

    impl ::std::convert::From<&Self> for JsonRpcResponseForRpcHealthResponseAndRpcError {
        fn from(value: &JsonRpcResponseForRpcHealthResponseAndRpcError) -> Self {
            value.clone()
        }
    }

    ///JsonRpcResponseForRpcLightClientBlockProofResponseAndRpcError
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "
    /// JsonRpcResponse_for_RpcLightClientBlockProofResponse_and_RpcError",
    ///  "type": "object",
    ///  "anyOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "result"
    ///      ],
    ///      "properties": {
    ///        "result": {
    ///          "$ref": "#/components/schemas/RpcLightClientBlockProofResponse"
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
    pub enum JsonRpcResponseForRpcLightClientBlockProofResponseAndRpcError {
        Variant0 {
            id: ::std::string::String,
            jsonrpc: ::std::string::String,
            result: RpcLightClientBlockProofResponse,
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

    impl ::std::convert::From<&Self> for JsonRpcResponseForRpcLightClientBlockProofResponseAndRpcError {
        fn from(value: &JsonRpcResponseForRpcLightClientBlockProofResponseAndRpcError) -> Self {
            value.clone()
        }
    }

    ///JsonRpcResponseForRpcLightClientExecutionProofResponseAndRpcError
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "JsonRpcResponse_for_RpcLightClientExecutionProofResponse_and_RpcError",
    ///  "type": "object",
    ///  "anyOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "result"
    ///      ],
    ///      "properties": {
    ///        "result": {
    ///          "$ref":
    /// "#/components/schemas/RpcLightClientExecutionProofResponse"
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
    pub enum JsonRpcResponseForRpcLightClientExecutionProofResponseAndRpcError {
        Variant0 {
            id: ::std::string::String,
            jsonrpc: ::std::string::String,
            result: RpcLightClientExecutionProofResponse,
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

    impl ::std::convert::From<&Self>
        for JsonRpcResponseForRpcLightClientExecutionProofResponseAndRpcError
    {
        fn from(value: &JsonRpcResponseForRpcLightClientExecutionProofResponseAndRpcError) -> Self {
            value.clone()
        }
    }

    ///JsonRpcResponseForRpcLightClientNextBlockResponseAndRpcError
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "
    /// JsonRpcResponse_for_RpcLightClientNextBlockResponse_and_RpcError",
    ///  "type": "object",
    ///  "anyOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "result"
    ///      ],
    ///      "properties": {
    ///        "result": {
    ///          "$ref": "#/components/schemas/RpcLightClientNextBlockResponse"
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
    pub enum JsonRpcResponseForRpcLightClientNextBlockResponseAndRpcError {
        Variant0 {
            id: ::std::string::String,
            jsonrpc: ::std::string::String,
            result: RpcLightClientNextBlockResponse,
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

    impl ::std::convert::From<&Self> for JsonRpcResponseForRpcLightClientNextBlockResponseAndRpcError {
        fn from(value: &JsonRpcResponseForRpcLightClientNextBlockResponseAndRpcError) -> Self {
            value.clone()
        }
    }

    ///JsonRpcResponseForRpcNetworkInfoResponseAndRpcError
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "JsonRpcResponse_for_RpcNetworkInfoResponse_and_RpcError",
    ///  "type": "object",
    ///  "anyOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "result"
    ///      ],
    ///      "properties": {
    ///        "result": {
    ///          "$ref": "#/components/schemas/RpcNetworkInfoResponse"
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
    pub enum JsonRpcResponseForRpcNetworkInfoResponseAndRpcError {
        Variant0 {
            id: ::std::string::String,
            jsonrpc: ::std::string::String,
            result: RpcNetworkInfoResponse,
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

    impl ::std::convert::From<&Self> for JsonRpcResponseForRpcNetworkInfoResponseAndRpcError {
        fn from(value: &JsonRpcResponseForRpcNetworkInfoResponseAndRpcError) -> Self {
            value.clone()
        }
    }

    ///JsonRpcResponseForRpcProtocolConfigResponseAndRpcError
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "JsonRpcResponse_for_RpcProtocolConfigResponse_and_RpcError",
    ///  "type": "object",
    ///  "anyOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "result"
    ///      ],
    ///      "properties": {
    ///        "result": {
    ///          "$ref": "#/components/schemas/RpcProtocolConfigResponse"
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
    pub enum JsonRpcResponseForRpcProtocolConfigResponseAndRpcError {
        Variant0 {
            id: ::std::string::String,
            jsonrpc: ::std::string::String,
            result: RpcProtocolConfigResponse,
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

    impl ::std::convert::From<&Self> for JsonRpcResponseForRpcProtocolConfigResponseAndRpcError {
        fn from(value: &JsonRpcResponseForRpcProtocolConfigResponseAndRpcError) -> Self {
            value.clone()
        }
    }

    ///JsonRpcResponseForRpcReceiptResponseAndRpcError
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "JsonRpcResponse_for_RpcReceiptResponse_and_RpcError",
    ///  "type": "object",
    ///  "anyOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "result"
    ///      ],
    ///      "properties": {
    ///        "result": {
    ///          "$ref": "#/components/schemas/RpcReceiptResponse"
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
    pub enum JsonRpcResponseForRpcReceiptResponseAndRpcError {
        Variant0 {
            id: ::std::string::String,
            jsonrpc: ::std::string::String,
            result: RpcReceiptResponse,
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

    impl ::std::convert::From<&Self> for JsonRpcResponseForRpcReceiptResponseAndRpcError {
        fn from(value: &JsonRpcResponseForRpcReceiptResponseAndRpcError) -> Self {
            value.clone()
        }
    }

    ///JsonRpcResponseForRpcStateChangesInBlockByTypeResponseAndRpcError
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "JsonRpcResponse_for_RpcStateChangesInBlockByTypeResponse_and_RpcError",
    ///  "type": "object",
    ///  "anyOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "result"
    ///      ],
    ///      "properties": {
    ///        "result": {
    ///          "$ref":
    /// "#/components/schemas/RpcStateChangesInBlockByTypeResponse"
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
    pub enum JsonRpcResponseForRpcStateChangesInBlockByTypeResponseAndRpcError {
        Variant0 {
            id: ::std::string::String,
            jsonrpc: ::std::string::String,
            result: RpcStateChangesInBlockByTypeResponse,
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

    impl ::std::convert::From<&Self>
        for JsonRpcResponseForRpcStateChangesInBlockByTypeResponseAndRpcError
    {
        fn from(value: &JsonRpcResponseForRpcStateChangesInBlockByTypeResponseAndRpcError) -> Self {
            value.clone()
        }
    }

    ///JsonRpcResponseForRpcStateChangesInBlockResponseAndRpcError
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "
    /// JsonRpcResponse_for_RpcStateChangesInBlockResponse_and_RpcError",
    ///  "type": "object",
    ///  "anyOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "result"
    ///      ],
    ///      "properties": {
    ///        "result": {
    ///          "$ref": "#/components/schemas/RpcStateChangesInBlockResponse"
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
    pub enum JsonRpcResponseForRpcStateChangesInBlockResponseAndRpcError {
        Variant0 {
            id: ::std::string::String,
            jsonrpc: ::std::string::String,
            result: RpcStateChangesInBlockResponse,
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

    impl ::std::convert::From<&Self> for JsonRpcResponseForRpcStateChangesInBlockResponseAndRpcError {
        fn from(value: &JsonRpcResponseForRpcStateChangesInBlockResponseAndRpcError) -> Self {
            value.clone()
        }
    }

    ///JsonRpcResponseForRpcStatusResponseAndRpcError
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "JsonRpcResponse_for_RpcStatusResponse_and_RpcError",
    ///  "type": "object",
    ///  "anyOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "result"
    ///      ],
    ///      "properties": {
    ///        "result": {
    ///          "$ref": "#/components/schemas/RpcStatusResponse"
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
    pub enum JsonRpcResponseForRpcStatusResponseAndRpcError {
        Variant0 {
            id: ::std::string::String,
            jsonrpc: ::std::string::String,
            result: RpcStatusResponse,
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

    impl ::std::convert::From<&Self> for JsonRpcResponseForRpcStatusResponseAndRpcError {
        fn from(value: &JsonRpcResponseForRpcStatusResponseAndRpcError) -> Self {
            value.clone()
        }
    }

    ///JsonRpcResponseForRpcTransactionResponseAndRpcError
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "JsonRpcResponse_for_RpcTransactionResponse_and_RpcError",
    ///  "type": "object",
    ///  "anyOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "result"
    ///      ],
    ///      "properties": {
    ///        "result": {
    ///          "$ref": "#/components/schemas/RpcTransactionResponse"
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
    pub enum JsonRpcResponseForRpcTransactionResponseAndRpcError {
        Variant0 {
            id: ::std::string::String,
            jsonrpc: ::std::string::String,
            result: RpcTransactionResponse,
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

    impl ::std::convert::From<&Self> for JsonRpcResponseForRpcTransactionResponseAndRpcError {
        fn from(value: &JsonRpcResponseForRpcTransactionResponseAndRpcError) -> Self {
            value.clone()
        }
    }

    ///JsonRpcResponseForRpcValidatorResponseAndRpcError
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "JsonRpcResponse_for_RpcValidatorResponse_and_RpcError",
    ///  "type": "object",
    ///  "anyOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "result"
    ///      ],
    ///      "properties": {
    ///        "result": {
    ///          "$ref": "#/components/schemas/RpcValidatorResponse"
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
    pub enum JsonRpcResponseForRpcValidatorResponseAndRpcError {
        Variant0 {
            id: ::std::string::String,
            jsonrpc: ::std::string::String,
            result: RpcValidatorResponse,
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

    impl ::std::convert::From<&Self> for JsonRpcResponseForRpcValidatorResponseAndRpcError {
        fn from(value: &JsonRpcResponseForRpcValidatorResponseAndRpcError) -> Self {
            value.clone()
        }
    }

    ///Information about a Producer: its account name, peer_id and a list of
    /// connected peers that the node can use to send message for this producer.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Information about a Producer: its account name, peer_id
    /// and a list of connected peers that the node can use to send message for
    /// this producer.",
    ///  "type": "object",
    ///  "required": [
    ///    "account_id",
    ///    "peer_id"
    ///  ],
    ///  "properties": {
    ///    "account_id": {
    ///      "$ref": "#/components/schemas/AccountId"
    ///    },
    ///    "next_hops": {
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "$ref": "#/components/schemas/PublicKey"
    ///      }
    ///    },
    ///    "peer_id": {
    ///      "$ref": "#/components/schemas/PublicKey"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct KnownProducerView {
        pub account_id: AccountId,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub next_hops: ::std::option::Option<::std::vec::Vec<PublicKey>>,
        pub peer_id: PublicKey,
    }

    impl ::std::convert::From<&KnownProducerView> for KnownProducerView {
        fn from(value: &KnownProducerView) -> Self {
            value.clone()
        }
    }

    ///LightClientBlockLiteView
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "inner_lite",
    ///    "inner_rest_hash",
    ///    "prev_block_hash"
    ///  ],
    ///  "properties": {
    ///    "inner_lite": {
    ///      "$ref": "#/components/schemas/BlockHeaderInnerLiteView"
    ///    },
    ///    "inner_rest_hash": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "prev_block_hash": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct LightClientBlockLiteView {
        pub inner_lite: BlockHeaderInnerLiteView,
        pub inner_rest_hash: CryptoHash,
        pub prev_block_hash: CryptoHash,
    }

    impl ::std::convert::From<&LightClientBlockLiteView> for LightClientBlockLiteView {
        fn from(value: &LightClientBlockLiteView) -> Self {
            value.clone()
        }
    }

    ///LightClientProofMethodNameHelperEnum
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "light_client_proof"
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
    pub enum LightClientProofMethodNameHelperEnum {
        #[serde(rename = "light_client_proof")]
        LightClientProof,
    }

    impl ::std::convert::From<&Self> for LightClientProofMethodNameHelperEnum {
        fn from(value: &LightClientProofMethodNameHelperEnum) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for LightClientProofMethodNameHelperEnum {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::LightClientProof => write!(f, "light_client_proof"),
            }
        }
    }

    impl ::std::str::FromStr for LightClientProofMethodNameHelperEnum {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "light_client_proof" => Ok(Self::LightClientProof),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for LightClientProofMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for LightClientProofMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for LightClientProofMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///Describes limits for VM and Runtime. TODO #4139: consider switching to
    /// strongly-typed wrappers instead of raw quantities
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Describes limits for VM and Runtime. TODO #4139:
    /// consider switching to strongly-typed wrappers instead of raw
    /// quantities",
    ///  "type": "object",
    ///  "required": [
    ///    "initial_memory_pages",
    ///    "max_actions_per_receipt",
    ///    "max_arguments_length",
    ///    "max_contract_size",
    ///    "max_gas_burnt",
    ///    "max_length_method_name",
    ///    "max_length_returned_data",
    ///    "max_length_storage_key",
    ///    "max_length_storage_value",
    ///    "max_memory_pages",
    ///    "max_number_bytes_method_names",
    ///    "max_number_input_data_dependencies",
    ///    "max_number_logs",
    ///    "max_number_registers",
    ///    "max_promises_per_function_call_action",
    ///    "max_receipt_size",
    ///    "max_register_size",
    ///    "max_stack_height",
    ///    "max_total_log_length",
    ///    "max_total_prepaid_gas",
    ///    "max_transaction_size",
    ///    "max_yield_payload_size",
    ///    "per_receipt_storage_proof_size_limit",
    ///    "registers_memory_limit",
    ///    "yield_timeout_length_in_blocks"
    ///  ],
    ///  "properties": {
    ///    "account_id_validity_rules_version": {
    ///      "description": "Whether to enforce account_id well-formed-ness
    /// where it wasn't enforced historically.",
    ///      "default": 0,
    ///      "type": "integer",
    ///      "format": "uint8",
    ///      "minimum": 0.0
    ///    },
    ///    "contract_prepare_version": {
    ///      "description": "Whether a legacy version of stack limiting should
    /// be used, see [`ContractPrepareVersion`].",
    ///      "default": 0,
    ///      "type": "integer",
    ///      "format": "uint8",
    ///      "minimum": 0.0
    ///    },
    ///    "initial_memory_pages": {
    ///      "description": "The initial number of memory pages. NOTE: It's not a limiter itself, but it's a value we use for initial_memory_pages.",
    ///      "type": "integer",
    ///      "format": "uint32",
    ///      "minimum": 0.0
    ///    },
    ///    "max_actions_per_receipt": {
    ///      "description": "Max number of actions per receipt.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "max_arguments_length": {
    ///      "description": "Max length of arguments in a function call
    /// action.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "max_contract_size": {
    ///      "description": "Max contract size",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "max_functions_number_per_contract": {
    ///      "description": "If present, stores max number of functions in one
    /// contract",
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "max_gas_burnt": {
    ///      "description": "Max amount of gas that can be used, excluding gas
    /// attached to promises.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "max_length_method_name": {
    ///      "description": "Max length of any method name (without terminating
    /// character).",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "max_length_returned_data": {
    ///      "description": "Max length of returned data",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "max_length_storage_key": {
    ///      "description": "Max storage key size",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "max_length_storage_value": {
    ///      "description": "Max storage value size",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "max_locals_per_contract": {
    ///      "description": "If present, stores max number of locals declared
    /// globally in one contract",
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "max_memory_pages": {
    ///      "description": "What is the maximal memory pages amount is allowed
    /// to have for a contract.",
    ///      "type": "integer",
    ///      "format": "uint32",
    ///      "minimum": 0.0
    ///    },
    ///    "max_number_bytes_method_names": {
    ///      "description": "Max total length of all method names (including
    /// terminating character) for a function call permission access key.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "max_number_input_data_dependencies": {
    ///      "description": "Max number of input data dependencies",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "max_number_logs": {
    ///      "description": "Maximum number of log entries.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "max_number_registers": {
    ///      "description": "Maximum number of registers that can be used
    /// simultaneously.\n\nNote that due to an implementation quirk [read: a
    /// bug] in VMLogic, if we have this number of registers, no subsequent
    /// writes to the registers will succeed even if they replace an existing
    /// register.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "max_promises_per_function_call_action": {
    ///      "description": "Max number of promises that a function call can
    /// create",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "max_receipt_size": {
    ///      "description": "Max receipt size",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "max_register_size": {
    ///      "description": "Maximum number of bytes that can be stored in a
    /// single register.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "max_stack_height": {
    ///      "description": "How tall the stack is allowed to grow?\n\nSee <https://wiki.parity.io/WebAssembly-StackHeight> to find out how the stack frame cost is calculated.",
    ///      "type": "integer",
    ///      "format": "uint32",
    ///      "minimum": 0.0
    ///    },
    ///    "max_total_log_length": {
    ///      "description": "Maximum total length in bytes of all log
    /// messages.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "max_total_prepaid_gas": {
    ///      "description": "Max total prepaid gas for all function call actions
    /// per receipt.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "max_transaction_size": {
    ///      "description": "Max transaction size",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "max_yield_payload_size": {
    ///      "description": "Maximum number of bytes for payload passed over a
    /// yield resume.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "per_receipt_storage_proof_size_limit": {
    ///      "description": "Hard limit on the size of storage proof generated
    /// while executing a single receipt.",
    ///      "type": "integer",
    ///      "format": "uint",
    ///      "minimum": 0.0
    ///    },
    ///    "registers_memory_limit": {
    ///      "description": "Limit of memory used by registers.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "wasmer2_stack_limit": {
    ///      "description": "If present, stores the secondary stack limit as implemented by wasmer2.\n\nThis limit should never be hit normally.",
    ///      "default": 102400,
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "yield_timeout_length_in_blocks": {
    ///      "description": "Number of blocks after which a yielded promise
    /// times out.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct LimitConfig {
        ///Whether to enforce account_id well-formed-ness where it wasn't
        /// enforced historically.
        #[serde(default)]
        pub account_id_validity_rules_version: u8,
        ///Whether a legacy version of stack limiting should be used, see
        /// [`ContractPrepareVersion`].
        #[serde(default)]
        pub contract_prepare_version: u8,
        ///The initial number of memory pages. NOTE: It's not a limiter itself,
        /// but it's a value we use for initial_memory_pages.
        pub initial_memory_pages: u32,
        ///Max number of actions per receipt.
        pub max_actions_per_receipt: u64,
        ///Max length of arguments in a function call action.
        pub max_arguments_length: u64,
        ///Max contract size
        pub max_contract_size: u64,
        ///If present, stores max number of functions in one contract
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub max_functions_number_per_contract: ::std::option::Option<u64>,
        ///Max amount of gas that can be used, excluding gas attached to
        /// promises.
        pub max_gas_burnt: u64,
        ///Max length of any method name (without terminating character).
        pub max_length_method_name: u64,
        ///Max length of returned data
        pub max_length_returned_data: u64,
        ///Max storage key size
        pub max_length_storage_key: u64,
        ///Max storage value size
        pub max_length_storage_value: u64,
        ///If present, stores max number of locals declared globally in one
        /// contract
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub max_locals_per_contract: ::std::option::Option<u64>,
        ///What is the maximal memory pages amount is allowed to have for a
        /// contract.
        pub max_memory_pages: u32,
        ///Max total length of all method names (including terminating
        /// character) for a function call permission access key.
        pub max_number_bytes_method_names: u64,
        ///Max number of input data dependencies
        pub max_number_input_data_dependencies: u64,
        ///Maximum number of log entries.
        pub max_number_logs: u64,
        ///Maximum number of registers that can be used simultaneously.
        ///
        ///Note that due to an implementation quirk [read: a bug] in VMLogic,
        /// if we have this number of registers, no subsequent writes to the
        /// registers will succeed even if they replace an existing register.
        pub max_number_registers: u64,
        ///Max number of promises that a function call can create
        pub max_promises_per_function_call_action: u64,
        ///Max receipt size
        pub max_receipt_size: u64,
        ///Maximum number of bytes that can be stored in a single register.
        pub max_register_size: u64,
        ///How tall the stack is allowed to grow?
        ///
        ///See <https://wiki.parity.io/WebAssembly-StackHeight> to find out how the stack frame cost is calculated.
        pub max_stack_height: u32,
        ///Maximum total length in bytes of all log messages.
        pub max_total_log_length: u64,
        ///Max total prepaid gas for all function call actions per receipt.
        pub max_total_prepaid_gas: u64,
        ///Max transaction size
        pub max_transaction_size: u64,
        ///Maximum number of bytes for payload passed over a yield resume.
        pub max_yield_payload_size: u64,
        ///Hard limit on the size of storage proof generated while executing a
        /// single receipt.
        pub per_receipt_storage_proof_size_limit: u32,
        ///Limit of memory used by registers.
        pub registers_memory_limit: u64,
        ///If present, stores the secondary stack limit as implemented by
        /// wasmer2.
        ///
        ///This limit should never be hit normally.
        #[serde(default = "defaults::default_u64::<i32, 102400>")]
        pub wasmer2_stack_limit: i32,
        ///Number of blocks after which a yielded promise times out.
        pub yield_timeout_length_in_blocks: u64,
    }

    impl ::std::convert::From<&LimitConfig> for LimitConfig {
        fn from(value: &LimitConfig) -> Self {
            value.clone()
        }
    }

    ///LogSummaryStyle
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "plain",
    ///    "colored"
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
    pub enum LogSummaryStyle {
        #[serde(rename = "plain")]
        Plain,
        #[serde(rename = "colored")]
        Colored,
    }

    impl ::std::convert::From<&Self> for LogSummaryStyle {
        fn from(value: &LogSummaryStyle) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for LogSummaryStyle {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Plain => write!(f, "plain"),
                Self::Colored => write!(f, "colored"),
            }
        }
    }

    impl ::std::str::FromStr for LogSummaryStyle {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "plain" => Ok(Self::Plain),
                "colored" => Ok(Self::Colored),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for LogSummaryStyle {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for LogSummaryStyle {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for LogSummaryStyle {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///MerklePathItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "direction",
    ///    "hash"
    ///  ],
    ///  "properties": {
    ///    "direction": {
    ///      "$ref": "#/components/schemas/Direction"
    ///    },
    ///    "hash": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct MerklePathItem {
        pub direction: Direction,
        pub hash: CryptoHash,
    }

    impl ::std::convert::From<&MerklePathItem> for MerklePathItem {
        fn from(value: &MerklePathItem) -> Self {
            value.clone()
        }
    }

    ///MethodResolveError
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "MethodEmptyName",
    ///    "MethodNotFound",
    ///    "MethodInvalidSignature"
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
    pub enum MethodResolveError {
        MethodEmptyName,
        MethodNotFound,
        MethodInvalidSignature,
    }

    impl ::std::convert::From<&Self> for MethodResolveError {
        fn from(value: &MethodResolveError) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for MethodResolveError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::MethodEmptyName => write!(f, "MethodEmptyName"),
                Self::MethodNotFound => write!(f, "MethodNotFound"),
                Self::MethodInvalidSignature => write!(f, "MethodInvalidSignature"),
            }
        }
    }

    impl ::std::str::FromStr for MethodResolveError {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "MethodEmptyName" => Ok(Self::MethodEmptyName),
                "MethodNotFound" => Ok(Self::MethodNotFound),
                "MethodInvalidSignature" => Ok(Self::MethodInvalidSignature),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for MethodResolveError {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for MethodResolveError {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for MethodResolveError {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///MissingTrieValue
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "context",
    ///    "hash"
    ///  ],
    ///  "properties": {
    ///    "context": {
    ///      "$ref": "#/components/schemas/MissingTrieValueContext"
    ///    },
    ///    "hash": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct MissingTrieValue {
        pub context: MissingTrieValueContext,
        pub hash: CryptoHash,
    }

    impl ::std::convert::From<&MissingTrieValue> for MissingTrieValue {
        fn from(value: &MissingTrieValue) -> Self {
            value.clone()
        }
    }

    ///Contexts in which `StorageError::MissingTrieValue` error might occur.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Contexts in which `StorageError::MissingTrieValue`
    /// error might occur.",
    ///  "oneOf": [
    ///    {
    ///      "description": "Missing trie value when reading from
    /// TrieIterator.",
    ///      "type": "string",
    ///      "enum": [
    ///        "TrieIterator"
    ///      ]
    ///    },
    ///    {
    ///      "description": "Missing trie value when reading from
    /// TriePrefetchingStorage.",
    ///      "type": "string",
    ///      "enum": [
    ///        "TriePrefetchingStorage"
    ///      ]
    ///    },
    ///    {
    ///      "description": "Missing trie value when reading from
    /// TrieMemoryPartialStorage.",
    ///      "type": "string",
    ///      "enum": [
    ///        "TrieMemoryPartialStorage"
    ///      ]
    ///    },
    ///    {
    ///      "description": "Missing trie value when reading from TrieStorage.",
    ///      "type": "string",
    ///      "enum": [
    ///        "TrieStorage"
    ///      ]
    ///    }
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
    pub enum MissingTrieValueContext {
        ///Missing trie value when reading from TrieIterator.
        TrieIterator,
        ///Missing trie value when reading from TriePrefetchingStorage.
        TriePrefetchingStorage,
        ///Missing trie value when reading from TrieMemoryPartialStorage.
        TrieMemoryPartialStorage,
        ///Missing trie value when reading from TrieStorage.
        TrieStorage,
    }

    impl ::std::convert::From<&Self> for MissingTrieValueContext {
        fn from(value: &MissingTrieValueContext) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for MissingTrieValueContext {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::TrieIterator => write!(f, "TrieIterator"),
                Self::TriePrefetchingStorage => write!(f, "TriePrefetchingStorage"),
                Self::TrieMemoryPartialStorage => write!(f, "TrieMemoryPartialStorage"),
                Self::TrieStorage => write!(f, "TrieStorage"),
            }
        }
    }

    impl ::std::str::FromStr for MissingTrieValueContext {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "TrieIterator" => Ok(Self::TrieIterator),
                "TriePrefetchingStorage" => Ok(Self::TriePrefetchingStorage),
                "TrieMemoryPartialStorage" => Ok(Self::TrieMemoryPartialStorage),
                "TrieStorage" => Ok(Self::TrieStorage),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for MissingTrieValueContext {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for MissingTrieValueContext {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for MissingTrieValueContext {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///MutableConfigValue
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
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
    pub struct MutableConfigValue(pub ::std::string::String);
    impl ::std::ops::Deref for MutableConfigValue {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<MutableConfigValue> for ::std::string::String {
        fn from(value: MutableConfigValue) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&MutableConfigValue> for MutableConfigValue {
        fn from(value: &MutableConfigValue) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<::std::string::String> for MutableConfigValue {
        fn from(value: ::std::string::String) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for MutableConfigValue {
        type Err = ::std::convert::Infallible;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::fmt::Display for MutableConfigValue {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
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

    ///NetworkInfoMethodNameHelperEnum
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "network_info"
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
    pub enum NetworkInfoMethodNameHelperEnum {
        #[serde(rename = "network_info")]
        NetworkInfo,
    }

    impl ::std::convert::From<&Self> for NetworkInfoMethodNameHelperEnum {
        fn from(value: &NetworkInfoMethodNameHelperEnum) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for NetworkInfoMethodNameHelperEnum {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::NetworkInfo => write!(f, "network_info"),
            }
        }
    }

    impl ::std::str::FromStr for NetworkInfoMethodNameHelperEnum {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "network_info" => Ok(Self::NetworkInfo),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for NetworkInfoMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for NetworkInfoMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for NetworkInfoMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///NetworkInfoView
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "connected_peers",
    ///    "known_producers",
    ///    "num_connected_peers",
    ///    "peer_max_count",
    ///    "tier1_accounts_data",
    ///    "tier1_accounts_keys",
    ///    "tier1_connections"
    ///  ],
    ///  "properties": {
    ///    "connected_peers": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/PeerInfoView"
    ///      }
    ///    },
    ///    "known_producers": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/KnownProducerView"
    ///      }
    ///    },
    ///    "num_connected_peers": {
    ///      "type": "integer",
    ///      "format": "uint",
    ///      "minimum": 0.0
    ///    },
    ///    "peer_max_count": {
    ///      "type": "integer",
    ///      "format": "uint32",
    ///      "minimum": 0.0
    ///    },
    ///    "tier1_accounts_data": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/AccountDataView"
    ///      }
    ///    },
    ///    "tier1_accounts_keys": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/PublicKey"
    ///      }
    ///    },
    ///    "tier1_connections": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/PeerInfoView"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct NetworkInfoView {
        pub connected_peers: ::std::vec::Vec<PeerInfoView>,
        pub known_producers: ::std::vec::Vec<KnownProducerView>,
        pub num_connected_peers: u32,
        pub peer_max_count: u32,
        pub tier1_accounts_data: ::std::vec::Vec<AccountDataView>,
        pub tier1_accounts_keys: ::std::vec::Vec<PublicKey>,
        pub tier1_connections: ::std::vec::Vec<PeerInfoView>,
    }

    impl ::std::convert::From<&NetworkInfoView> for NetworkInfoView {
        fn from(value: &NetworkInfoView) -> Self {
            value.clone()
        }
    }

    ///NextEpochValidatorInfo
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "account_id",
    ///    "public_key",
    ///    "shards",
    ///    "stake"
    ///  ],
    ///  "properties": {
    ///    "account_id": {
    ///      "$ref": "#/components/schemas/AccountId"
    ///    },
    ///    "public_key": {
    ///      "$ref": "#/components/schemas/PublicKey"
    ///    },
    ///    "shards": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ShardId"
    ///      }
    ///    },
    ///    "stake": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct NextEpochValidatorInfo {
        pub account_id: AccountId,
        pub public_key: PublicKey,
        pub shards: ::std::vec::Vec<ShardId>,
        pub stake: ::std::string::String,
    }

    impl ::std::convert::From<&NextEpochValidatorInfo> for NextEpochValidatorInfo {
        fn from(value: &NextEpochValidatorInfo) -> Self {
            value.clone()
        }
    }

    ///NextLightClientBlockMethodNameHelperEnum
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "next_light_client_block"
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
    pub enum NextLightClientBlockMethodNameHelperEnum {
        #[serde(rename = "next_light_client_block")]
        NextLightClientBlock,
    }

    impl ::std::convert::From<&Self> for NextLightClientBlockMethodNameHelperEnum {
        fn from(value: &NextLightClientBlockMethodNameHelperEnum) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for NextLightClientBlockMethodNameHelperEnum {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::NextLightClientBlock => write!(f, "next_light_client_block"),
            }
        }
    }

    impl ::std::str::FromStr for NextLightClientBlockMethodNameHelperEnum {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "next_light_client_block" => Ok(Self::NextLightClientBlock),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for NextLightClientBlockMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for NextLightClientBlockMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for NextLightClientBlockMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///This is Action which mustn't contain DelegateAction.
    ///
    ///This struct is needed to avoid the recursion when Action/DelegateAction
    /// is deserialized.
    ///
    ///Important: Don't make the inner Action public, this must only be
    /// constructed through the correct interface that ensures the inner Action
    /// is actually not a delegate action. That would break an assumption of
    /// this type, which we use in several places. For example, borsh
    /// de-/serialization relies on it. If the invariant is broken, we may end
    /// up with a `Transaction` or `Receipt` that we can serialize but
    /// deserializing it back causes a parsing error.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "This is Action which mustn't contain
    /// DelegateAction.\n\nThis struct is needed to avoid the recursion when
    /// Action/DelegateAction is deserialized.\n\nImportant: Don't make the
    /// inner Action public, this must only be constructed through the correct
    /// interface that ensures the inner Action is actually not a delegate
    /// action. That would break an assumption of this type, which we use in
    /// several places. For example, borsh de-/serialization relies on it. If
    /// the invariant is broken, we may end up with a `Transaction` or `Receipt`
    /// that we can serialize but deserializing it back causes a parsing
    /// error.",
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/Action"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct NonDelegateAction(pub Action);
    impl ::std::ops::Deref for NonDelegateAction {
        type Target = Action;
        fn deref(&self) -> &Action {
            &self.0
        }
    }

    impl ::std::convert::From<NonDelegateAction> for Action {
        fn from(value: NonDelegateAction) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&NonDelegateAction> for NonDelegateAction {
        fn from(value: &NonDelegateAction) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<Action> for NonDelegateAction {
        fn from(value: Action) -> Self {
            Self(value)
        }
    }

    ///Peer id is the public key.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Peer id is the public key.",
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/PublicKey"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct PeerId(pub PublicKey);
    impl ::std::ops::Deref for PeerId {
        type Target = PublicKey;
        fn deref(&self) -> &PublicKey {
            &self.0
        }
    }

    impl ::std::convert::From<PeerId> for PublicKey {
        fn from(value: PeerId) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&PeerId> for PeerId {
        fn from(value: &PeerId) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<PublicKey> for PeerId {
        fn from(value: PublicKey) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for PeerId {
        type Err = <PublicKey as ::std::str::FromStr>::Err;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl ::std::convert::TryFrom<&str> for PeerId {
        type Error = <PublicKey as ::std::str::FromStr>::Err;
        fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&String> for PeerId {
        type Error = <PublicKey as ::std::str::FromStr>::Err;
        fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for PeerId {
        type Error = <PublicKey as ::std::str::FromStr>::Err;
        fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::fmt::Display for PeerId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    ///PeerInfoView
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "addr",
    ///    "archival",
    ///    "connection_established_time_millis",
    ///    "is_highest_block_invalid",
    ///    "is_outbound_peer",
    ///    "last_time_peer_requested_millis",
    ///    "last_time_received_message_millis",
    ///    "nonce",
    ///    "peer_id",
    ///    "received_bytes_per_sec",
    ///    "sent_bytes_per_sec",
    ///    "tracked_shards"
    ///  ],
    ///  "properties": {
    ///    "account_id": {
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/AccountId"
    ///            }
    ///          ]
    ///        }
    ///      ]
    ///    },
    ///    "addr": {
    ///      "type": "string"
    ///    },
    ///    "archival": {
    ///      "type": "boolean"
    ///    },
    ///    "block_hash": {
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/CryptoHash"
    ///            }
    ///          ]
    ///        }
    ///      ]
    ///    },
    ///    "connection_established_time_millis": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "height": {
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "is_highest_block_invalid": {
    ///      "type": "boolean"
    ///    },
    ///    "is_outbound_peer": {
    ///      "type": "boolean"
    ///    },
    ///    "last_time_peer_requested_millis": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "last_time_received_message_millis": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "nonce": {
    ///      "description": "Connection nonce.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "peer_id": {
    ///      "$ref": "#/components/schemas/PublicKey"
    ///    },
    ///    "received_bytes_per_sec": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "sent_bytes_per_sec": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "tracked_shards": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ShardId"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PeerInfoView {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub account_id: ::std::option::Option<AccountId>,
        pub addr: ::std::string::String,
        pub archival: bool,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub block_hash: ::std::option::Option<CryptoHash>,
        pub connection_established_time_millis: u64,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub height: ::std::option::Option<u64>,
        pub is_highest_block_invalid: bool,
        pub is_outbound_peer: bool,
        pub last_time_peer_requested_millis: u64,
        pub last_time_received_message_millis: u64,
        ///Connection nonce.
        pub nonce: u64,
        pub peer_id: PublicKey,
        pub received_bytes_per_sec: u64,
        pub sent_bytes_per_sec: u64,
        pub tracked_shards: ::std::vec::Vec<ShardId>,
    }

    impl ::std::convert::From<&PeerInfoView> for PeerInfoView {
        fn from(value: &PeerInfoView) -> Self {
            value.clone()
        }
    }

    ///Error that can occur while preparing or executing Wasm smart-contract.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Error that can occur while preparing or executing Wasm
    /// smart-contract.",
    ///  "oneOf": [
    ///    {
    ///      "description": "Error happened while serializing the module.",
    ///      "type": "string",
    ///      "enum": [
    ///        "Serialization"
    ///      ]
    ///    },
    ///    {
    ///      "description": "Error happened while deserializing the module.",
    ///      "type": "string",
    ///      "enum": [
    ///        "Deserialization"
    ///      ]
    ///    },
    ///    {
    ///      "description": "Internal memory declaration has been found in the
    /// module.",
    ///      "type": "string",
    ///      "enum": [
    ///        "InternalMemoryDeclared"
    ///      ]
    ///    },
    ///    {
    ///      "description": "Gas instrumentation failed.\n\nThis most likely
    /// indicates the module isn't valid.",
    ///      "type": "string",
    ///      "enum": [
    ///        "GasInstrumentation"
    ///      ]
    ///    },
    ///    {
    ///      "description": "Stack instrumentation failed.\n\nThis  most likely
    /// indicates the module isn't valid.",
    ///      "type": "string",
    ///      "enum": [
    ///        "StackHeightInstrumentation"
    ///      ]
    ///    },
    ///    {
    ///      "description": "Error happened during instantiation.\n\nThis might
    /// indicate that `start` function trapped, or module isn't instantiable
    /// and/or un-linkable.",
    ///      "type": "string",
    ///      "enum": [
    ///        "Instantiate"
    ///      ]
    ///    },
    ///    {
    ///      "description": "Error creating memory.",
    ///      "type": "string",
    ///      "enum": [
    ///        "Memory"
    ///      ]
    ///    },
    ///    {
    ///      "description": "Contract contains too many functions.",
    ///      "type": "string",
    ///      "enum": [
    ///        "TooManyFunctions"
    ///      ]
    ///    },
    ///    {
    ///      "description": "Contract contains too many locals.",
    ///      "type": "string",
    ///      "enum": [
    ///        "TooManyLocals"
    ///      ]
    ///    }
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
    pub enum PrepareError {
        ///Error happened while serializing the module.
        Serialization,
        ///Error happened while deserializing the module.
        Deserialization,
        ///Internal memory declaration has been found in the module.
        InternalMemoryDeclared,
        ///Gas instrumentation failed.
        ///
        ///This most likely indicates the module isn't valid.
        GasInstrumentation,
        ///Stack instrumentation failed.
        ///
        ///This  most likely indicates the module isn't valid.
        StackHeightInstrumentation,
        ///Error happened during instantiation.
        ///
        ///This might indicate that `start` function trapped, or module isn't
        /// instantiable and/or un-linkable.
        Instantiate,
        ///Error creating memory.
        Memory,
        ///Contract contains too many functions.
        TooManyFunctions,
        ///Contract contains too many locals.
        TooManyLocals,
    }

    impl ::std::convert::From<&Self> for PrepareError {
        fn from(value: &PrepareError) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for PrepareError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Serialization => write!(f, "Serialization"),
                Self::Deserialization => write!(f, "Deserialization"),
                Self::InternalMemoryDeclared => write!(f, "InternalMemoryDeclared"),
                Self::GasInstrumentation => write!(f, "GasInstrumentation"),
                Self::StackHeightInstrumentation => write!(f, "StackHeightInstrumentation"),
                Self::Instantiate => write!(f, "Instantiate"),
                Self::Memory => write!(f, "Memory"),
                Self::TooManyFunctions => write!(f, "TooManyFunctions"),
                Self::TooManyLocals => write!(f, "TooManyLocals"),
            }
        }
    }

    impl ::std::str::FromStr for PrepareError {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "Serialization" => Ok(Self::Serialization),
                "Deserialization" => Ok(Self::Deserialization),
                "InternalMemoryDeclared" => Ok(Self::InternalMemoryDeclared),
                "GasInstrumentation" => Ok(Self::GasInstrumentation),
                "StackHeightInstrumentation" => Ok(Self::StackHeightInstrumentation),
                "Instantiate" => Ok(Self::Instantiate),
                "Memory" => Ok(Self::Memory),
                "TooManyFunctions" => Ok(Self::TooManyFunctions),
                "TooManyLocals" => Ok(Self::TooManyLocals),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for PrepareError {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for PrepareError {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for PrepareError {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///PublicKey
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
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
    pub struct PublicKey(pub ::std::string::String);
    impl ::std::ops::Deref for PublicKey {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<PublicKey> for ::std::string::String {
        fn from(value: PublicKey) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&PublicKey> for PublicKey {
        fn from(value: &PublicKey) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<::std::string::String> for PublicKey {
        fn from(value: ::std::string::String) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for PublicKey {
        type Err = ::std::convert::Infallible;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::fmt::Display for PublicKey {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    ///Rational32SchemaProvider
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "denom",
    ///    "numer"
    ///  ],
    ///  "properties": {
    ///    "denom": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "numer": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Rational32SchemaProvider {
        pub denom: i32,
        pub numer: i32,
    }

    impl ::std::convert::From<&Rational32SchemaProvider> for Rational32SchemaProvider {
        fn from(value: &Rational32SchemaProvider) -> Self {
            value.clone()
        }
    }

    ///ReceiptEnumView
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "Action"
    ///      ],
    ///      "properties": {
    ///        "Action": {
    ///          "type": "object",
    ///          "required": [
    ///            "actions",
    ///            "gas_price",
    ///            "input_data_ids",
    ///            "output_data_receivers",
    ///            "signer_id",
    ///            "signer_public_key"
    ///          ],
    ///          "properties": {
    ///            "actions": {
    ///              "type": "array",
    ///              "items": {
    ///                "$ref": "#/components/schemas/ActionView"
    ///              }
    ///            },
    ///            "gas_price": {
    ///              "type": "string"
    ///            },
    ///            "input_data_ids": {
    ///              "type": "array",
    ///              "items": {
    ///                "$ref": "#/components/schemas/CryptoHash"
    ///              }
    ///            },
    ///            "is_promise_yield": {
    ///              "default": false,
    ///              "type": "boolean"
    ///            },
    ///            "output_data_receivers": {
    ///              "type": "array",
    ///              "items": {
    ///                "$ref": "#/components/schemas/DataReceiverView"
    ///              }
    ///            },
    ///            "signer_id": {
    ///              "$ref": "#/components/schemas/AccountId"
    ///            },
    ///            "signer_public_key": {
    ///              "$ref": "#/components/schemas/PublicKey"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "Data"
    ///      ],
    ///      "properties": {
    ///        "Data": {
    ///          "type": "object",
    ///          "required": [
    ///            "data_id"
    ///          ],
    ///          "properties": {
    ///            "data": {
    ///              "type": [
    ///                "string",
    ///                "null"
    ///              ]
    ///            },
    ///            "data_id": {
    ///              "$ref": "#/components/schemas/CryptoHash"
    ///            },
    ///            "is_promise_resume": {
    ///              "default": false,
    ///              "type": "boolean"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "GlobalContractDistribution"
    ///      ],
    ///      "properties": {
    ///        "GlobalContractDistribution": {
    ///          "type": "object",
    ///          "required": [
    ///            "data"
    ///          ],
    ///          "properties": {
    ///            "data": {
    ///              "$ref": "#/components/schemas/GlobalContractData"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub enum ReceiptEnumView {
        Action {
            actions: ::std::vec::Vec<ActionView>,
            gas_price: ::std::string::String,
            input_data_ids: ::std::vec::Vec<CryptoHash>,
            #[serde(default)]
            is_promise_yield: bool,
            output_data_receivers: ::std::vec::Vec<DataReceiverView>,
            signer_id: AccountId,
            signer_public_key: PublicKey,
        },
        Data {
            #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
            data: ::std::option::Option<::std::string::String>,
            data_id: CryptoHash,
            #[serde(default)]
            is_promise_resume: bool,
        },
        GlobalContractDistribution {
            data: GlobalContractData,
        },
    }

    impl ::std::convert::From<&Self> for ReceiptEnumView {
        fn from(value: &ReceiptEnumView) -> Self {
            value.clone()
        }
    }

    ///Describes the error for validating a receipt.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Describes the error for validating a receipt.",
    ///  "oneOf": [
    ///    {
    ///      "description": "The `predecessor_id` of a Receipt is not valid.",
    ///      "type": "object",
    ///      "required": [
    ///        "InvalidPredecessorId"
    ///      ],
    ///      "properties": {
    ///        "InvalidPredecessorId": {
    ///          "type": "object",
    ///          "required": [
    ///            "account_id"
    ///          ],
    ///          "properties": {
    ///            "account_id": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "The `receiver_id` of a Receipt is not valid.",
    ///      "type": "object",
    ///      "required": [
    ///        "InvalidReceiverId"
    ///      ],
    ///      "properties": {
    ///        "InvalidReceiverId": {
    ///          "type": "object",
    ///          "required": [
    ///            "account_id"
    ///          ],
    ///          "properties": {
    ///            "account_id": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "The `signer_id` of an ActionReceipt is not valid.",
    ///      "type": "object",
    ///      "required": [
    ///        "InvalidSignerId"
    ///      ],
    ///      "properties": {
    ///        "InvalidSignerId": {
    ///          "type": "object",
    ///          "required": [
    ///            "account_id"
    ///          ],
    ///          "properties": {
    ///            "account_id": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "The `receiver_id` of a DataReceiver within an
    /// ActionReceipt is not valid.",
    ///      "type": "object",
    ///      "required": [
    ///        "InvalidDataReceiverId"
    ///      ],
    ///      "properties": {
    ///        "InvalidDataReceiverId": {
    ///          "type": "object",
    ///          "required": [
    ///            "account_id"
    ///          ],
    ///          "properties": {
    ///            "account_id": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "The length of the returned data exceeded the limit
    /// in a DataReceipt.",
    ///      "type": "object",
    ///      "required": [
    ///        "ReturnedValueLengthExceeded"
    ///      ],
    ///      "properties": {
    ///        "ReturnedValueLengthExceeded": {
    ///          "type": "object",
    ///          "required": [
    ///            "length",
    ///            "limit"
    ///          ],
    ///          "properties": {
    ///            "length": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            },
    ///            "limit": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "The number of input data dependencies exceeds the
    /// limit in an ActionReceipt.",
    ///      "type": "object",
    ///      "required": [
    ///        "NumberInputDataDependenciesExceeded"
    ///      ],
    ///      "properties": {
    ///        "NumberInputDataDependenciesExceeded": {
    ///          "type": "object",
    ///          "required": [
    ///            "limit",
    ///            "number_of_input_data_dependencies"
    ///          ],
    ///          "properties": {
    ///            "limit": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            },
    ///            "number_of_input_data_dependencies": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "An error occurred while validating actions of an
    /// ActionReceipt.",
    ///      "type": "object",
    ///      "required": [
    ///        "ActionsValidation"
    ///      ],
    ///      "properties": {
    ///        "ActionsValidation": {
    ///          "$ref": "#/components/schemas/ActionsValidationError"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "Receipt is bigger than the limit.",
    ///      "type": "object",
    ///      "required": [
    ///        "ReceiptSizeExceeded"
    ///      ],
    ///      "properties": {
    ///        "ReceiptSizeExceeded": {
    ///          "type": "object",
    ///          "required": [
    ///            "limit",
    ///            "size"
    ///          ],
    ///          "properties": {
    ///            "limit": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            },
    ///            "size": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub enum ReceiptValidationError {
        ///The `predecessor_id` of a Receipt is not valid.
        InvalidPredecessorId { account_id: ::std::string::String },
        ///The `receiver_id` of a Receipt is not valid.
        InvalidReceiverId { account_id: ::std::string::String },
        ///The `signer_id` of an ActionReceipt is not valid.
        InvalidSignerId { account_id: ::std::string::String },
        ///The `receiver_id` of a DataReceiver within an ActionReceipt is not
        /// valid.
        InvalidDataReceiverId { account_id: ::std::string::String },
        ///The length of the returned data exceeded the limit in a DataReceipt.
        ReturnedValueLengthExceeded { length: u64, limit: u64 },
        ///The number of input data dependencies exceeds the limit in an
        /// ActionReceipt.
        NumberInputDataDependenciesExceeded {
            limit: u64,
            number_of_input_data_dependencies: u64,
        },
        ///An error occurred while validating actions of an ActionReceipt.
        ActionsValidation(ActionsValidationError),
        ///Receipt is bigger than the limit.
        ReceiptSizeExceeded { limit: u64, size: u64 },
    }

    impl ::std::convert::From<&Self> for ReceiptValidationError {
        fn from(value: &ReceiptValidationError) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<ActionsValidationError> for ReceiptValidationError {
        fn from(value: ActionsValidationError) -> Self {
            Self::ActionsValidation(value)
        }
    }

    ///ReceiptView
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "predecessor_id",
    ///    "receipt",
    ///    "receipt_id",
    ///    "receiver_id"
    ///  ],
    ///  "properties": {
    ///    "predecessor_id": {
    ///      "$ref": "#/components/schemas/AccountId"
    ///    },
    ///    "priority": {
    ///      "default": 0,
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "receipt": {
    ///      "$ref": "#/components/schemas/ReceiptEnumView"
    ///    },
    ///    "receipt_id": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "receiver_id": {
    ///      "$ref": "#/components/schemas/AccountId"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ReceiptView {
        pub predecessor_id: AccountId,
        #[serde(default)]
        pub priority: u64,
        pub receipt: ReceiptEnumView,
        pub receipt_id: CryptoHash,
        pub receiver_id: AccountId,
    }

    impl ::std::convert::From<&ReceiptView> for ReceiptView {
        fn from(value: &ReceiptView) -> Self {
            value.clone()
        }
    }

    ///RpcBlockRequest
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "oneOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "block_id"
    ///      ],
    ///      "properties": {
    ///        "block_id": {
    ///          "$ref": "#/components/schemas/BlockId"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "finality"
    ///      ],
    ///      "properties": {
    ///        "finality": {
    ///          "$ref": "#/components/schemas/Finality"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "sync_checkpoint"
    ///      ],
    ///      "properties": {
    ///        "sync_checkpoint": {
    ///          "$ref": "#/components/schemas/SyncCheckpoint"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub enum RpcBlockRequest {
        #[serde(rename = "block_id")]
        BlockId(BlockId),
        #[serde(rename = "finality")]
        Finality(Finality),
        #[serde(rename = "sync_checkpoint")]
        SyncCheckpoint(SyncCheckpoint),
    }

    impl ::std::convert::From<&Self> for RpcBlockRequest {
        fn from(value: &RpcBlockRequest) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<BlockId> for RpcBlockRequest {
        fn from(value: BlockId) -> Self {
            Self::BlockId(value)
        }
    }

    impl ::std::convert::From<Finality> for RpcBlockRequest {
        fn from(value: Finality) -> Self {
            Self::Finality(value)
        }
    }

    impl ::std::convert::From<SyncCheckpoint> for RpcBlockRequest {
        fn from(value: SyncCheckpoint) -> Self {
            Self::SyncCheckpoint(value)
        }
    }

    ///RpcBlockResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "author",
    ///    "chunks",
    ///    "header"
    ///  ],
    ///  "properties": {
    ///    "author": {
    ///      "$ref": "#/components/schemas/AccountId"
    ///    },
    ///    "chunks": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ChunkHeaderView"
    ///      }
    ///    },
    ///    "header": {
    ///      "$ref": "#/components/schemas/BlockHeaderView"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RpcBlockResponse {
        pub author: AccountId,
        pub chunks: ::std::vec::Vec<ChunkHeaderView>,
        pub header: BlockHeaderView,
    }

    impl ::std::convert::From<&RpcBlockResponse> for RpcBlockResponse {
        fn from(value: &RpcBlockResponse) -> Self {
            value.clone()
        }
    }

    ///RpcClientConfigRequest
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object"
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct RpcClientConfigRequest(
        pub ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    );
    impl ::std::ops::Deref for RpcClientConfigRequest {
        type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
        fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
            &self.0
        }
    }

    impl ::std::convert::From<RpcClientConfigRequest>
        for ::serde_json::Map<::std::string::String, ::serde_json::Value>
    {
        fn from(value: RpcClientConfigRequest) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&RpcClientConfigRequest> for RpcClientConfigRequest {
        fn from(value: &RpcClientConfigRequest) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
        for RpcClientConfigRequest
    {
        fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
            Self(value)
        }
    }

    ///ClientConfig where some fields can be updated at runtime.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "ClientConfig where some fields can be updated at
    /// runtime.",
    ///  "type": "object",
    ///  "required": [
    ///    "archive",
    ///    "block_fetch_horizon",
    ///    "block_header_fetch_horizon",
    ///    "block_production_tracking_delay",
    ///    "catchup_step_period",
    ///    "chain_id",
    ///    "chunk_request_retry_period",
    ///    "client_background_migration_threads",
    ///    "doomslug_step_period",
    ///    "enable_multiline_logging",
    ///    "enable_statistics_export",
    ///    "epoch_length",
    ///    "epoch_sync",
    ///    "expected_shutdown",
    ///    "gc",
    ///    "header_sync_expected_height_per_second",
    ///    "header_sync_initial_timeout",
    ///    "header_sync_progress_timeout",
    ///    "header_sync_stall_ban_timeout",
    ///    "log_summary_period",
    ///    "log_summary_style",
    ///    "max_block_production_delay",
    ///    "max_block_wait_delay",
    ///    "min_block_production_delay",
    ///    "min_num_peers",
    ///    "num_block_producer_seats",
    ///    "orphan_state_witness_max_size",
    ///    "orphan_state_witness_pool_size",
    ///    "produce_chunk_add_transactions_time_limit",
    ///    "produce_empty_blocks",
    ///    "resharding_config",
    ///    "save_latest_witnesses",
    ///    "save_trie_changes",
    ///    "skip_sync_wait",
    ///    "state_sync",
    ///    "state_sync_enabled",
    ///    "state_sync_external_backoff",
    ///    "state_sync_external_timeout",
    ///    "state_sync_p2p_timeout",
    ///    "state_sync_retry_backoff",
    ///    "sync_check_period",
    ///    "sync_height_threshold",
    ///    "sync_max_block_requests",
    ///    "sync_step_period",
    ///    "tracked_accounts",
    ///    "tracked_shard_schedule",
    ///    "tracked_shards",
    ///    "ttl_account_id_router",
    ///    "tx_routing_height_horizon",
    ///    "version",
    ///    "view_client_threads",
    ///    "view_client_throttle_period"
    ///  ],
    ///  "properties": {
    ///    "archive": {
    ///      "description": "Not clear old data, set `true` for archive nodes.",
    ///      "type": "boolean"
    ///    },
    ///    "block_fetch_horizon": {
    ///      "description": "Horizon at which instead of fetching block, fetch
    /// full state.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "block_header_fetch_horizon": {
    ///      "description": "Behind this horizon header fetch kicks in.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "block_production_tracking_delay": {
    ///      "description": "Duration to check for producing / skipping block.",
    ///      "type": "string"
    ///    },
    ///    "catchup_step_period": {
    ///      "description": "Time between check to perform catchup.",
    ///      "type": "string"
    ///    },
    ///    "chain_id": {
    ///      "description": "Chain id for status.",
    ///      "type": "string"
    ///    },
    ///    "chunk_distribution_network": {
    ///      "description": "Optional config for the Chunk Distribution Network
    /// feature. If set to `None` then this node does not participate in the
    /// Chunk Distribution Network. Nodes not participating will still function
    /// fine, but possibly with higher latency due to the need of requesting
    /// chunks over the peer-to-peer network.",
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref":
    /// "#/components/schemas/ChunkDistributionNetworkConfig"
    ///            }
    ///          ]
    ///        }
    ///      ]
    ///    },
    ///    "chunk_request_retry_period": {
    ///      "description": "Time between checking to re-request chunks.",
    ///      "type": "string"
    ///    },
    ///    "client_background_migration_threads": {
    ///      "description": "Number of threads to execute background migration
    /// work in client.",
    ///      "type": "integer",
    ///      "format": "uint",
    ///      "minimum": 0.0
    ///    },
    ///    "doomslug_step_period": {
    ///      "description": "Time between running doomslug timer.",
    ///      "type": "string"
    ///    },
    ///    "enable_multiline_logging": {
    ///      "type": "boolean"
    ///    },
    ///    "enable_statistics_export": {
    ///      "description": "Re-export storage layer statistics as prometheus
    /// metrics.",
    ///      "type": "boolean"
    ///    },
    ///    "epoch_length": {
    ///      "description": "Epoch length.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "epoch_sync": {
    ///      "description": "Options for epoch sync.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/EpochSyncConfig"
    ///        }
    ///      ]
    ///    },
    ///    "expected_shutdown": {
    ///      "description": "Graceful shutdown at expected block height.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/MutableConfigValue"
    ///        }
    ///      ]
    ///    },
    ///    "gc": {
    ///      "description": "Garbage collection configuration.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/GCConfig"
    ///        }
    ///      ]
    ///    },
    ///    "header_sync_expected_height_per_second": {
    ///      "description": "Expected increase of header head height per second
    /// during header sync",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "header_sync_initial_timeout": {
    ///      "description": "How much time to wait after initial header sync",
    ///      "type": "string"
    ///    },
    ///    "header_sync_progress_timeout": {
    ///      "description": "How much time to wait after some progress is made
    /// in header sync",
    ///      "type": "string"
    ///    },
    ///    "header_sync_stall_ban_timeout": {
    ///      "description": "How much time to wait before banning a peer in
    /// header sync if sync is too slow",
    ///      "type": "string"
    ///    },
    ///    "log_summary_period": {
    ///      "description": "Period between logging summary information.",
    ///      "type": "string"
    ///    },
    ///    "log_summary_style": {
    ///      "description": "Enable coloring of the logs",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/LogSummaryStyle"
    ///        }
    ///      ]
    ///    },
    ///    "max_block_production_delay": {
    ///      "description": "Maximum wait for approvals before producing
    /// block.",
    ///      "type": "string"
    ///    },
    ///    "max_block_wait_delay": {
    ///      "description": "Maximum duration before skipping given height.",
    ///      "type": "string"
    ///    },
    ///    "max_gas_burnt_view": {
    ///      "description": "Max burnt gas per view method.  If present,
    /// overrides value stored in genesis file.  The value only affects the RPCs
    /// without influencing the protocol thus changing it per-node doesn’t
    /// affect the blockchain.",
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "min_block_production_delay": {
    ///      "description": "Minimum duration before producing block.",
    ///      "type": "string"
    ///    },
    ///    "min_num_peers": {
    ///      "description": "Minimum number of peers to start syncing.",
    ///      "type": "integer",
    ///      "format": "uint",
    ///      "minimum": 0.0
    ///    },
    ///    "num_block_producer_seats": {
    ///      "description": "Number of block producer seats",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "orphan_state_witness_max_size": {
    ///      "description": "Maximum size of state witnesses in the
    /// OrphanStateWitnessPool.\n\nWe keep only orphan witnesses which are
    /// smaller than this size. This limits the maximum memory usage of
    /// OrphanStateWitnessPool.",
    ///      "type": "string"
    ///    },
    ///    "orphan_state_witness_pool_size": {
    ///      "description": "OrphanStateWitnessPool keeps instances of
    /// ChunkStateWitness which can't be processed because the previous block
    /// isn't available. The witnesses wait in the pool until the required block
    /// appears. This variable controls how many witnesses can be stored in the
    /// pool.",
    ///      "type": "integer",
    ///      "format": "uint",
    ///      "minimum": 0.0
    ///    },
    ///    "produce_chunk_add_transactions_time_limit": {
    ///      "description": "Limit the time of adding transactions to a chunk. A
    /// node produces a chunk by adding transactions from the transaction pool
    /// until some limit is reached. This time limit ensures that adding
    /// transactions won't take longer than the specified duration, which helps
    /// to produce the chunk quickly.",
    ///      "type": "string"
    ///    },
    ///    "produce_empty_blocks": {
    ///      "description": "Produce empty blocks, use `false` for testing.",
    ///      "type": "boolean"
    ///    },
    ///    "resharding_config": {
    ///      "$ref": "#/components/schemas/MutableConfigValue"
    ///    },
    ///    "rpc_addr": {
    ///      "description": "Listening rpc port for status.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "save_latest_witnesses": {
    ///      "description": "Save observed instances of ChunkStateWitness to the
    /// database in DBCol::LatestChunkStateWitnesses. Saving the latest
    /// witnesses is useful for analysis and debugging. When this option is
    /// enabled, the node will save ALL witnesses it observes, even invalid
    /// ones, which can cause extra load on the database. This option is not
    /// recommended for production use, as a large number of incoming witnesses
    /// could cause denial of service.",
    ///      "type": "boolean"
    ///    },
    ///    "save_trie_changes": {
    ///      "description": "save_trie_changes should be set to true iff -
    /// archive if false - non-archival nodes need trie changes to perform
    /// garbage collection - archive is true, cold_store is configured and
    /// migration to split_storage is finished - node working in split storage
    /// mode needs trie changes in order to do garbage collection on hot.",
    ///      "type": "boolean"
    ///    },
    ///    "skip_sync_wait": {
    ///      "description": "Skip waiting for sync (for testing or single node
    /// testnet).",
    ///      "type": "boolean"
    ///    },
    ///    "state_sync": {
    ///      "description": "Options for syncing state.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/StateSyncConfig"
    ///        }
    ///      ]
    ///    },
    ///    "state_sync_enabled": {
    ///      "description": "Whether to use the State Sync mechanism. If
    /// disabled, the node will do Block Sync instead of State Sync.",
    ///      "type": "boolean"
    ///    },
    ///    "state_sync_external_backoff": {
    ///      "description": "Additional waiting period after a failed request to
    /// external storage",
    ///      "type": "string"
    ///    },
    ///    "state_sync_external_timeout": {
    ///      "description": "How long to wait for a response from centralized
    /// state sync",
    ///      "type": "string"
    ///    },
    ///    "state_sync_p2p_timeout": {
    ///      "description": "How long to wait for a response from p2p state
    /// sync",
    ///      "type": "string"
    ///    },
    ///    "state_sync_retry_backoff": {
    ///      "description": "How long to wait after a failed state sync
    /// request",
    ///      "type": "string"
    ///    },
    ///    "sync_check_period": {
    ///      "description": "How often to check that we are not out of sync.",
    ///      "type": "string"
    ///    },
    ///    "sync_height_threshold": {
    ///      "description": "Sync height threshold: below this difference in
    /// height don't start syncing.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "sync_max_block_requests": {
    ///      "description": "Maximum number of block requests to send to peers
    /// to sync",
    ///      "type": "integer",
    ///      "format": "uint",
    ///      "minimum": 0.0
    ///    },
    ///    "sync_step_period": {
    ///      "description": "While syncing, how long to check for each step.",
    ///      "type": "string"
    ///    },
    ///    "tracked_accounts": {
    ///      "description": "Accounts that this client tracks.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/AccountId"
    ///      }
    ///    },
    ///    "tracked_shadow_validator": {
    ///      "description": "Track shards that should be tracked by given
    /// validator.",
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/AccountId"
    ///            }
    ///          ]
    ///        }
    ///      ]
    ///    },
    ///    "tracked_shard_schedule": {
    ///      "description": "Rotate between these sets of tracked shards. Used
    /// to simulate the behavior of chunk only producers without staking tokens.
    /// This field is only used if `tracked_shards` is empty.",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "array",
    ///        "items": {
    ///          "$ref": "#/components/schemas/ShardId"
    ///        }
    ///      }
    ///    },
    ///    "tracked_shards": {
    ///      "description": "Shards that this client tracks.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ShardId"
    ///      }
    ///    },
    ///    "transaction_pool_size_limit": {
    ///      "description": "Limit of the size of per-shard transaction pool
    /// measured in bytes. If not set, the size will be unbounded.",
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "trie_viewer_state_size_limit": {
    ///      "description": "Upper bound of the byte size of contract state that
    /// is still viewable. None is no limit",
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "ttl_account_id_router": {
    ///      "description": "Time to persist Accounts Id in the router without
    /// removing them.",
    ///      "type": "string"
    ///    },
    ///    "tx_routing_height_horizon": {
    ///      "description": "If the node is not a chunk producer within that
    /// many blocks, then route to upcoming chunk producers.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "version": {
    ///      "description": "Version of the binary.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Version"
    ///        }
    ///      ]
    ///    },
    ///    "view_client_threads": {
    ///      "description": "Number of threads for ViewClientActor pool.",
    ///      "type": "integer",
    ///      "format": "uint",
    ///      "minimum": 0.0
    ///    },
    ///    "view_client_throttle_period": {
    ///      "description": "Number of seconds between state requests for view
    /// client.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RpcClientConfigResponse {
        ///Not clear old data, set `true` for archive nodes.
        pub archive: bool,
        ///Horizon at which instead of fetching block, fetch full state.
        pub block_fetch_horizon: u64,
        ///Behind this horizon header fetch kicks in.
        pub block_header_fetch_horizon: u64,
        ///Duration to check for producing / skipping block.
        pub block_production_tracking_delay: ::std::string::String,
        ///Time between check to perform catchup.
        pub catchup_step_period: ::std::string::String,
        ///Chain id for status.
        pub chain_id: ::std::string::String,
        ///Optional config for the Chunk Distribution Network feature. If set
        /// to `None` then this node does not participate in the Chunk
        /// Distribution Network. Nodes not participating will still function
        /// fine, but possibly with higher latency due to the need of requesting
        /// chunks over the peer-to-peer network.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub chunk_distribution_network: ::std::option::Option<ChunkDistributionNetworkConfig>,
        ///Time between checking to re-request chunks.
        pub chunk_request_retry_period: ::std::string::String,
        ///Number of threads to execute background migration work in client.
        pub client_background_migration_threads: u32,
        ///Time between running doomslug timer.
        pub doomslug_step_period: ::std::string::String,
        pub enable_multiline_logging: bool,
        ///Re-export storage layer statistics as prometheus metrics.
        pub enable_statistics_export: bool,
        ///Epoch length.
        pub epoch_length: u64,
        ///Options for epoch sync.
        pub epoch_sync: EpochSyncConfig,
        ///Graceful shutdown at expected block height.
        pub expected_shutdown: MutableConfigValue,
        ///Garbage collection configuration.
        pub gc: GcConfig,
        ///Expected increase of header head height per second during header
        /// sync
        pub header_sync_expected_height_per_second: u64,
        ///How much time to wait after initial header sync
        pub header_sync_initial_timeout: ::std::string::String,
        ///How much time to wait after some progress is made in header sync
        pub header_sync_progress_timeout: ::std::string::String,
        ///How much time to wait before banning a peer in header sync if sync
        /// is too slow
        pub header_sync_stall_ban_timeout: ::std::string::String,
        ///Period between logging summary information.
        pub log_summary_period: ::std::string::String,
        ///Enable coloring of the logs
        pub log_summary_style: LogSummaryStyle,
        ///Maximum wait for approvals before producing block.
        pub max_block_production_delay: ::std::string::String,
        ///Maximum duration before skipping given height.
        pub max_block_wait_delay: ::std::string::String,
        ///Max burnt gas per view method.  If present, overrides value stored
        /// in genesis file.  The value only affects the RPCs without
        /// influencing the protocol thus changing it per-node doesn’t affect
        /// the blockchain.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub max_gas_burnt_view: ::std::option::Option<u64>,
        ///Minimum duration before producing block.
        pub min_block_production_delay: ::std::string::String,
        ///Minimum number of peers to start syncing.
        pub min_num_peers: u32,
        ///Number of block producer seats
        pub num_block_producer_seats: u64,
        ///Maximum size of state witnesses in the OrphanStateWitnessPool.
        ///
        ///We keep only orphan witnesses which are smaller than this size. This
        /// limits the maximum memory usage of OrphanStateWitnessPool.
        pub orphan_state_witness_max_size: ::std::string::String,
        ///OrphanStateWitnessPool keeps instances of ChunkStateWitness which
        /// can't be processed because the previous block isn't available. The
        /// witnesses wait in the pool until the required block appears. This
        /// variable controls how many witnesses can be stored in the pool.
        pub orphan_state_witness_pool_size: u32,
        ///Limit the time of adding transactions to a chunk. A node produces a
        /// chunk by adding transactions from the transaction pool until some
        /// limit is reached. This time limit ensures that adding transactions
        /// won't take longer than the specified duration, which helps to
        /// produce the chunk quickly.
        pub produce_chunk_add_transactions_time_limit: ::std::string::String,
        ///Produce empty blocks, use `false` for testing.
        pub produce_empty_blocks: bool,
        pub resharding_config: MutableConfigValue,
        ///Listening rpc port for status.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub rpc_addr: ::std::option::Option<::std::string::String>,
        ///Save observed instances of ChunkStateWitness to the database in
        /// DBCol::LatestChunkStateWitnesses. Saving the latest witnesses is
        /// useful for analysis and debugging. When this option is enabled, the
        /// node will save ALL witnesses it observes, even invalid ones, which
        /// can cause extra load on the database. This option is not recommended
        /// for production use, as a large number of incoming witnesses could
        /// cause denial of service.
        pub save_latest_witnesses: bool,
        ///save_trie_changes should be set to true iff - archive if false -
        /// non-archival nodes need trie changes to perform garbage collection -
        /// archive is true, cold_store is configured and migration to
        /// split_storage is finished - node working in split storage mode needs
        /// trie changes in order to do garbage collection on hot.
        pub save_trie_changes: bool,
        ///Skip waiting for sync (for testing or single node testnet).
        pub skip_sync_wait: bool,
        ///Options for syncing state.
        pub state_sync: StateSyncConfig,
        ///Whether to use the State Sync mechanism. If disabled, the node will
        /// do Block Sync instead of State Sync.
        pub state_sync_enabled: bool,
        ///Additional waiting period after a failed request to external storage
        pub state_sync_external_backoff: ::std::string::String,
        ///How long to wait for a response from centralized state sync
        pub state_sync_external_timeout: ::std::string::String,
        ///How long to wait for a response from p2p state sync
        pub state_sync_p2p_timeout: ::std::string::String,
        ///How long to wait after a failed state sync request
        pub state_sync_retry_backoff: ::std::string::String,
        ///How often to check that we are not out of sync.
        pub sync_check_period: ::std::string::String,
        ///Sync height threshold: below this difference in height don't start
        /// syncing.
        pub sync_height_threshold: u64,
        ///Maximum number of block requests to send to peers to sync
        pub sync_max_block_requests: u32,
        ///While syncing, how long to check for each step.
        pub sync_step_period: ::std::string::String,
        ///Accounts that this client tracks.
        pub tracked_accounts: ::std::vec::Vec<AccountId>,
        ///Track shards that should be tracked by given validator.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub tracked_shadow_validator: ::std::option::Option<AccountId>,
        ///Rotate between these sets of tracked shards. Used to simulate the
        /// behavior of chunk only producers without staking tokens. This field
        /// is only used if `tracked_shards` is empty.
        pub tracked_shard_schedule: ::std::vec::Vec<::std::vec::Vec<ShardId>>,
        ///Shards that this client tracks.
        pub tracked_shards: ::std::vec::Vec<ShardId>,
        ///Limit of the size of per-shard transaction pool measured in bytes.
        /// If not set, the size will be unbounded.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub transaction_pool_size_limit: ::std::option::Option<u64>,
        ///Upper bound of the byte size of contract state that is still
        /// viewable. None is no limit
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub trie_viewer_state_size_limit: ::std::option::Option<u64>,
        ///Time to persist Accounts Id in the router without removing them.
        pub ttl_account_id_router: ::std::string::String,
        ///If the node is not a chunk producer within that many blocks, then
        /// route to upcoming chunk producers.
        pub tx_routing_height_horizon: u64,
        ///Version of the binary.
        pub version: Version,
        ///Number of threads for ViewClientActor pool.
        pub view_client_threads: u32,
        ///Number of seconds between state requests for view client.
        pub view_client_throttle_period: ::std::string::String,
    }

    impl ::std::convert::From<&RpcClientConfigResponse> for RpcClientConfigResponse {
        fn from(value: &RpcClientConfigResponse) -> Self {
            value.clone()
        }
    }

    ///RpcCongestionLevelResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "congestion_level"
    ///  ],
    ///  "properties": {
    ///    "congestion_level": {
    ///      "type": "number",
    ///      "format": "double"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RpcCongestionLevelResponse {
        pub congestion_level: f64,
    }

    impl ::std::convert::From<&RpcCongestionLevelResponse> for RpcCongestionLevelResponse {
        fn from(value: &RpcCongestionLevelResponse) -> Self {
            value.clone()
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
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/CauseRpcErrorKind"
    ///            }
    ///          ]
    ///        }
    ///      ]
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
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/NameRpcErrorKind"
    ///            }
    ///          ]
    ///        }
    ///      ]
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

    ///RpcGasPriceRequest
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "block_id": {
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/BlockId"
    ///            }
    ///          ]
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RpcGasPriceRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub block_id: ::std::option::Option<BlockId>,
    }

    impl ::std::convert::From<&RpcGasPriceRequest> for RpcGasPriceRequest {
        fn from(value: &RpcGasPriceRequest) -> Self {
            value.clone()
        }
    }

    impl ::std::default::Default for RpcGasPriceRequest {
        fn default() -> Self {
            Self {
                block_id: Default::default(),
            }
        }
    }

    ///RpcGasPriceResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "gas_price"
    ///  ],
    ///  "properties": {
    ///    "gas_price": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RpcGasPriceResponse {
        pub gas_price: ::std::string::String,
    }

    impl ::std::convert::From<&RpcGasPriceResponse> for RpcGasPriceResponse {
        fn from(value: &RpcGasPriceResponse) -> Self {
            value.clone()
        }
    }

    ///RpcHealthRequest
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object"
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct RpcHealthRequest(pub ::serde_json::Map<::std::string::String, ::serde_json::Value>);
    impl ::std::ops::Deref for RpcHealthRequest {
        type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
        fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
            &self.0
        }
    }

    impl ::std::convert::From<RpcHealthRequest>
        for ::serde_json::Map<::std::string::String, ::serde_json::Value>
    {
        fn from(value: RpcHealthRequest) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&RpcHealthRequest> for RpcHealthRequest {
        fn from(value: &RpcHealthRequest) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
        for RpcHealthRequest
    {
        fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
            Self(value)
        }
    }

    ///RpcHealthResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object"
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct RpcHealthResponse(pub ::serde_json::Map<::std::string::String, ::serde_json::Value>);
    impl ::std::ops::Deref for RpcHealthResponse {
        type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
        fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
            &self.0
        }
    }

    impl ::std::convert::From<RpcHealthResponse>
        for ::serde_json::Map<::std::string::String, ::serde_json::Value>
    {
        fn from(value: RpcHealthResponse) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&RpcHealthResponse> for RpcHealthResponse {
        fn from(value: &RpcHealthResponse) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
        for RpcHealthResponse
    {
        fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
            Self(value)
        }
    }

    ///RpcKnownProducer
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "account_id",
    ///    "peer_id"
    ///  ],
    ///  "properties": {
    ///    "account_id": {
    ///      "$ref": "#/components/schemas/AccountId"
    ///    },
    ///    "addr": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "peer_id": {
    ///      "$ref": "#/components/schemas/PeerId"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RpcKnownProducer {
        pub account_id: AccountId,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub addr: ::std::option::Option<::std::string::String>,
        pub peer_id: PeerId,
    }

    impl ::std::convert::From<&RpcKnownProducer> for RpcKnownProducer {
        fn from(value: &RpcKnownProducer) -> Self {
            value.clone()
        }
    }

    ///RpcLightClientBlockProofResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "block_header_lite",
    ///    "block_proof"
    ///  ],
    ///  "properties": {
    ///    "block_header_lite": {
    ///      "$ref": "#/components/schemas/LightClientBlockLiteView"
    ///    },
    ///    "block_proof": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/MerklePathItem"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RpcLightClientBlockProofResponse {
        pub block_header_lite: LightClientBlockLiteView,
        pub block_proof: ::std::vec::Vec<MerklePathItem>,
    }

    impl ::std::convert::From<&RpcLightClientBlockProofResponse> for RpcLightClientBlockProofResponse {
        fn from(value: &RpcLightClientBlockProofResponse) -> Self {
            value.clone()
        }
    }

    ///RpcLightClientExecutionProofRequest
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "oneOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "sender_id",
    ///        "transaction_hash",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "sender_id": {
    ///          "$ref": "#/components/schemas/AccountId"
    ///        },
    ///        "transaction_hash": {
    ///          "$ref": "#/components/schemas/CryptoHash"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "transaction"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "receipt_id",
    ///        "receiver_id",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "receipt_id": {
    ///          "$ref": "#/components/schemas/CryptoHash"
    ///        },
    ///        "receiver_id": {
    ///          "$ref": "#/components/schemas/AccountId"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "receipt"
    ///          ]
    ///        }
    ///      }
    ///    }
    ///  ],
    ///  "required": [
    ///    "light_client_head"
    ///  ],
    ///  "properties": {
    ///    "light_client_head": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum RpcLightClientExecutionProofRequest {
        Variant0(RpcLightClientExecutionProofRequestVariant0),
        Variant1(RpcLightClientExecutionProofRequestVariant1),
    }

    impl ::std::convert::From<&Self> for RpcLightClientExecutionProofRequest {
        fn from(value: &RpcLightClientExecutionProofRequest) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<RpcLightClientExecutionProofRequestVariant0>
        for RpcLightClientExecutionProofRequest
    {
        fn from(value: RpcLightClientExecutionProofRequestVariant0) -> Self {
            Self::Variant0(value)
        }
    }

    impl ::std::convert::From<RpcLightClientExecutionProofRequestVariant1>
        for RpcLightClientExecutionProofRequest
    {
        fn from(value: RpcLightClientExecutionProofRequestVariant1) -> Self {
            Self::Variant1(value)
        }
    }

    ///RpcLightClientExecutionProofRequestVariant0
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "light_client_head"
    ///      ],
    ///      "properties": {
    ///        "light_client_head": {
    ///          "$ref": "#/components/schemas/CryptoHash"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "sender_id",
    ///        "transaction_hash",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "sender_id": {
    ///          "$ref": "#/components/schemas/AccountId"
    ///        },
    ///        "transaction_hash": {
    ///          "$ref": "#/components/schemas/CryptoHash"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "transaction"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "receipt_id",
    ///          "receiver_id",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "receipt_id": {
    ///            "$ref": "#/components/schemas/CryptoHash"
    ///          },
    ///          "receiver_id": {
    ///            "$ref": "#/components/schemas/AccountId"
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "receipt"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    }
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
    #[serde(deny_unknown_fields)]
    pub enum RpcLightClientExecutionProofRequestVariant0 {}
    impl ::std::convert::From<&Self> for RpcLightClientExecutionProofRequestVariant0 {
        fn from(value: &RpcLightClientExecutionProofRequestVariant0) -> Self {
            value.clone()
        }
    }

    ///RpcLightClientExecutionProofRequestVariant1
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "light_client_head"
    ///      ],
    ///      "properties": {
    ///        "light_client_head": {
    ///          "$ref": "#/components/schemas/CryptoHash"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "receipt_id",
    ///        "receiver_id",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "receipt_id": {
    ///          "$ref": "#/components/schemas/CryptoHash"
    ///        },
    ///        "receiver_id": {
    ///          "$ref": "#/components/schemas/AccountId"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "receipt"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "sender_id",
    ///          "transaction_hash",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "sender_id": {
    ///            "$ref": "#/components/schemas/AccountId"
    ///          },
    ///          "transaction_hash": {
    ///            "$ref": "#/components/schemas/CryptoHash"
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "transaction"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    }
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
    #[serde(deny_unknown_fields)]
    pub enum RpcLightClientExecutionProofRequestVariant1 {}
    impl ::std::convert::From<&Self> for RpcLightClientExecutionProofRequestVariant1 {
        fn from(value: &RpcLightClientExecutionProofRequestVariant1) -> Self {
            value.clone()
        }
    }

    ///RpcLightClientExecutionProofResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "block_header_lite",
    ///    "block_proof",
    ///    "outcome_proof",
    ///    "outcome_root_proof"
    ///  ],
    ///  "properties": {
    ///    "block_header_lite": {
    ///      "$ref": "#/components/schemas/LightClientBlockLiteView"
    ///    },
    ///    "block_proof": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/MerklePathItem"
    ///      }
    ///    },
    ///    "outcome_proof": {
    ///      "$ref": "#/components/schemas/ExecutionOutcomeWithIdView"
    ///    },
    ///    "outcome_root_proof": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/MerklePathItem"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RpcLightClientExecutionProofResponse {
        pub block_header_lite: LightClientBlockLiteView,
        pub block_proof: ::std::vec::Vec<MerklePathItem>,
        pub outcome_proof: ExecutionOutcomeWithIdView,
        pub outcome_root_proof: ::std::vec::Vec<MerklePathItem>,
    }

    impl ::std::convert::From<&RpcLightClientExecutionProofResponse>
        for RpcLightClientExecutionProofResponse
    {
        fn from(value: &RpcLightClientExecutionProofResponse) -> Self {
            value.clone()
        }
    }

    ///RpcLightClientNextBlockRequest
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "last_block_hash"
    ///  ],
    ///  "properties": {
    ///    "last_block_hash": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RpcLightClientNextBlockRequest {
        pub last_block_hash: CryptoHash,
    }

    impl ::std::convert::From<&RpcLightClientNextBlockRequest> for RpcLightClientNextBlockRequest {
        fn from(value: &RpcLightClientNextBlockRequest) -> Self {
            value.clone()
        }
    }

    ///RpcLightClientNextBlockResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "approvals_after_next": {
    ///      "type": "array",
    ///      "items": {
    ///        "oneOf": [
    ///          {
    ///            "type": "null"
    ///          },
    ///          {
    ///            "allOf": [
    ///              {
    ///                "$ref": "#/components/schemas/Signature"
    ///              }
    ///            ]
    ///          }
    ///        ]
    ///      }
    ///    },
    ///    "inner_lite": {
    ///      "$ref": "#/components/schemas/BlockHeaderInnerLiteView"
    ///    },
    ///    "inner_rest_hash": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "next_block_inner_hash": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "next_bps": {
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "$ref": "#/components/schemas/ValidatorStakeView"
    ///      }
    ///    },
    ///    "prev_block_hash": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RpcLightClientNextBlockResponse {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub approvals_after_next: ::std::vec::Vec<::std::option::Option<Signature>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub inner_lite: ::std::option::Option<BlockHeaderInnerLiteView>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub inner_rest_hash: ::std::option::Option<CryptoHash>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub next_block_inner_hash: ::std::option::Option<CryptoHash>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub next_bps: ::std::option::Option<::std::vec::Vec<ValidatorStakeView>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub prev_block_hash: ::std::option::Option<CryptoHash>,
    }

    impl ::std::convert::From<&RpcLightClientNextBlockResponse> for RpcLightClientNextBlockResponse {
        fn from(value: &RpcLightClientNextBlockResponse) -> Self {
            value.clone()
        }
    }

    impl ::std::default::Default for RpcLightClientNextBlockResponse {
        fn default() -> Self {
            Self {
                approvals_after_next: Default::default(),
                inner_lite: Default::default(),
                inner_rest_hash: Default::default(),
                next_block_inner_hash: Default::default(),
                next_bps: Default::default(),
                prev_block_hash: Default::default(),
            }
        }
    }

    ///RpcNetworkInfoRequest
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object"
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct RpcNetworkInfoRequest(
        pub ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    );
    impl ::std::ops::Deref for RpcNetworkInfoRequest {
        type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
        fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
            &self.0
        }
    }

    impl ::std::convert::From<RpcNetworkInfoRequest>
        for ::serde_json::Map<::std::string::String, ::serde_json::Value>
    {
        fn from(value: RpcNetworkInfoRequest) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&RpcNetworkInfoRequest> for RpcNetworkInfoRequest {
        fn from(value: &RpcNetworkInfoRequest) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
        for RpcNetworkInfoRequest
    {
        fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
            Self(value)
        }
    }

    ///RpcNetworkInfoResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "active_peers",
    ///    "known_producers",
    ///    "num_active_peers",
    ///    "peer_max_count",
    ///    "received_bytes_per_sec",
    ///    "sent_bytes_per_sec"
    ///  ],
    ///  "properties": {
    ///    "active_peers": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/RpcPeerInfo"
    ///      }
    ///    },
    ///    "known_producers": {
    ///      "description": "Accounts of known block and chunk producers from
    /// routing table.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/RpcKnownProducer"
    ///      }
    ///    },
    ///    "num_active_peers": {
    ///      "type": "integer",
    ///      "format": "uint",
    ///      "minimum": 0.0
    ///    },
    ///    "peer_max_count": {
    ///      "type": "integer",
    ///      "format": "uint32",
    ///      "minimum": 0.0
    ///    },
    ///    "received_bytes_per_sec": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "sent_bytes_per_sec": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RpcNetworkInfoResponse {
        pub active_peers: ::std::vec::Vec<RpcPeerInfo>,
        ///Accounts of known block and chunk producers from routing table.
        pub known_producers: ::std::vec::Vec<RpcKnownProducer>,
        pub num_active_peers: u32,
        pub peer_max_count: u32,
        pub received_bytes_per_sec: u64,
        pub sent_bytes_per_sec: u64,
    }

    impl ::std::convert::From<&RpcNetworkInfoResponse> for RpcNetworkInfoResponse {
        fn from(value: &RpcNetworkInfoResponse) -> Self {
            value.clone()
        }
    }

    ///RpcPeerInfo
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id"
    ///  ],
    ///  "properties": {
    ///    "account_id": {
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/AccountId"
    ///            }
    ///          ]
    ///        }
    ///      ]
    ///    },
    ///    "addr": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "id": {
    ///      "$ref": "#/components/schemas/PeerId"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RpcPeerInfo {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub account_id: ::std::option::Option<AccountId>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub addr: ::std::option::Option<::std::string::String>,
        pub id: PeerId,
    }

    impl ::std::convert::From<&RpcPeerInfo> for RpcPeerInfo {
        fn from(value: &RpcPeerInfo) -> Self {
            value.clone()
        }
    }

    ///RpcProtocolConfigResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "avg_hidden_validator_seats_per_shard",
    ///    "block_producer_kickout_threshold",
    ///    "chain_id",
    ///    "chunk_producer_kickout_threshold",
    ///    "chunk_validator_only_kickout_threshold",
    ///    "dynamic_resharding",
    ///    "epoch_length",
    ///    "fishermen_threshold",
    ///    "gas_limit",
    ///    "gas_price_adjustment_rate",
    ///    "genesis_height",
    ///    "genesis_time",
    ///    "max_gas_price",
    ///    "max_inflation_rate",
    ///    "max_kickout_stake_perc",
    ///    "min_gas_price",
    ///    "minimum_stake_divisor",
    ///    "minimum_stake_ratio",
    ///    "minimum_validators_per_shard",
    ///    "num_block_producer_seats",
    ///    "num_block_producer_seats_per_shard",
    ///    "num_blocks_per_year",
    ///    "num_chunk_only_producer_seats",
    ///    "online_max_threshold",
    ///    "online_min_threshold",
    ///    "protocol_reward_rate",
    ///    "protocol_treasury_account",
    ///    "protocol_upgrade_stake_threshold",
    ///    "protocol_version",
    ///    "runtime_config",
    ///    "shard_layout",
    ///    "shuffle_shard_assignment_for_chunk_producers",
    ///    "target_validator_mandates_per_shard",
    ///    "transaction_validity_period"
    ///  ],
    ///  "properties": {
    ///    "avg_hidden_validator_seats_per_shard": {
    ///      "description": "Expected number of hidden validators per shard.",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "integer",
    ///        "format": "uint64",
    ///        "minimum": 0.0
    ///      }
    ///    },
    ///    "block_producer_kickout_threshold": {
    ///      "description": "Threshold for kicking out block producers, between
    /// 0 and 100.",
    ///      "type": "integer",
    ///      "format": "uint8",
    ///      "minimum": 0.0
    ///    },
    ///    "chain_id": {
    ///      "description": "ID of the blockchain. This must be unique for every
    /// blockchain. If your testnet blockchains do not have unique chain IDs,
    /// you will have a bad time.",
    ///      "type": "string"
    ///    },
    ///    "chunk_producer_kickout_threshold": {
    ///      "description": "Threshold for kicking out chunk producers, between
    /// 0 and 100.",
    ///      "type": "integer",
    ///      "format": "uint8",
    ///      "minimum": 0.0
    ///    },
    ///    "chunk_validator_only_kickout_threshold": {
    ///      "description": "Threshold for kicking out nodes which are only
    /// chunk validators, between 0 and 100.",
    ///      "type": "integer",
    ///      "format": "uint8",
    ///      "minimum": 0.0
    ///    },
    ///    "dynamic_resharding": {
    ///      "description": "Enable dynamic re-sharding.",
    ///      "type": "boolean"
    ///    },
    ///    "epoch_length": {
    ///      "description": "Epoch length counted in block heights.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "fishermen_threshold": {
    ///      "description": "Fishermen stake threshold.",
    ///      "type": "string"
    ///    },
    ///    "gas_limit": {
    ///      "description": "Initial gas limit.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "gas_price_adjustment_rate": {
    ///      "description": "Gas price adjustment rate",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Rational32SchemaProvider"
    ///        }
    ///      ]
    ///    },
    ///    "genesis_height": {
    ///      "description": "Height of genesis block.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "genesis_time": {
    ///      "description": "Official time of blockchain start.",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "max_gas_price": {
    ///      "description": "Maximum gas price.",
    ///      "type": "string"
    ///    },
    ///    "max_inflation_rate": {
    ///      "description": "Maximum inflation on the total supply every
    /// epoch.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Rational32SchemaProvider"
    ///        }
    ///      ]
    ///    },
    ///    "max_kickout_stake_perc": {
    ///      "description": "Max stake percentage of the validators we will kick
    /// out.",
    ///      "type": "integer",
    ///      "format": "uint8",
    ///      "minimum": 0.0
    ///    },
    ///    "min_gas_price": {
    ///      "description": "Minimum gas price. It is also the initial gas
    /// price.",
    ///      "type": "string"
    ///    },
    ///    "minimum_stake_divisor": {
    ///      "description": "The minimum stake required for staking is last seat
    /// price divided by this number.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "minimum_stake_ratio": {
    ///      "description": "The lowest ratio s/s_total any block producer can have. See <https://github.com/near/NEPs/pull/167> for details",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Rational32SchemaProvider"
    ///        }
    ///      ]
    ///    },
    ///    "minimum_validators_per_shard": {
    ///      "description": "The minimum number of validators each shard must
    /// have",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "num_block_producer_seats": {
    ///      "description": "Number of block producer seats at genesis.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "num_block_producer_seats_per_shard": {
    ///      "description": "Defines number of shards and number of block
    /// producer seats per each shard at genesis.",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "integer",
    ///        "format": "uint64",
    ///        "minimum": 0.0
    ///      }
    ///    },
    ///    "num_blocks_per_year": {
    ///      "description": "Expected number of blocks per year",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "num_chunk_only_producer_seats": {
    ///      "description": "Number of validator seats for chunk only
    /// producers.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "online_max_threshold": {
    ///      "description": "Online maximum threshold above which validator gets
    /// full reward.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Rational32SchemaProvider"
    ///        }
    ///      ]
    ///    },
    ///    "online_min_threshold": {
    ///      "description": "Online minimum threshold below which validator
    /// doesn't receive reward.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Rational32SchemaProvider"
    ///        }
    ///      ]
    ///    },
    ///    "protocol_reward_rate": {
    ///      "description": "Protocol treasury rate",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Rational32SchemaProvider"
    ///        }
    ///      ]
    ///    },
    ///    "protocol_treasury_account": {
    ///      "description": "Protocol treasury account",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/AccountId"
    ///        }
    ///      ]
    ///    },
    ///    "protocol_upgrade_stake_threshold": {
    ///      "description": "Threshold of stake that needs to indicate that they
    /// ready for upgrade.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Rational32SchemaProvider"
    ///        }
    ///      ]
    ///    },
    ///    "protocol_version": {
    ///      "description": "Current Protocol Version",
    ///      "type": "integer",
    ///      "format": "uint32",
    ///      "minimum": 0.0
    ///    },
    ///    "runtime_config": {
    ///      "description": "Runtime configuration (mostly economics
    /// constants).",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/RuntimeConfigView"
    ///        }
    ///      ]
    ///    },
    ///    "shard_layout": {
    ///      "description": "Layout information regarding how to split accounts
    /// to shards",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/ShardLayout"
    ///        }
    ///      ]
    ///    },
    ///    "shuffle_shard_assignment_for_chunk_producers": {
    ///      "description": "If true, shuffle the chunk producers across shards.
    /// In other words, if the shard assignments were `[S_0, S_1, S_2, S_3]`
    /// where `S_i` represents the set of chunk producers for shard `i`, if this
    /// flag were true, the shard assignments might become, for example, `[S_2,
    /// S_0, S_3, S_1]`.",
    ///      "type": "boolean"
    ///    },
    ///    "target_validator_mandates_per_shard": {
    ///      "description": "Number of target chunk validator mandates for each
    /// shard.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "transaction_validity_period": {
    ///      "description": "Number of blocks for which a given transaction is
    /// valid",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RpcProtocolConfigResponse {
        ///Expected number of hidden validators per shard.
        pub avg_hidden_validator_seats_per_shard: ::std::vec::Vec<u64>,
        ///Threshold for kicking out block producers, between 0 and 100.
        pub block_producer_kickout_threshold: u8,
        ///ID of the blockchain. This must be unique for every blockchain. If
        /// your testnet blockchains do not have unique chain IDs, you will have
        /// a bad time.
        pub chain_id: ::std::string::String,
        ///Threshold for kicking out chunk producers, between 0 and 100.
        pub chunk_producer_kickout_threshold: u8,
        ///Threshold for kicking out nodes which are only chunk validators,
        /// between 0 and 100.
        pub chunk_validator_only_kickout_threshold: u8,
        ///Enable dynamic re-sharding.
        pub dynamic_resharding: bool,
        ///Epoch length counted in block heights.
        pub epoch_length: u64,
        ///Fishermen stake threshold.
        pub fishermen_threshold: ::std::string::String,
        ///Initial gas limit.
        pub gas_limit: u64,
        ///Gas price adjustment rate
        pub gas_price_adjustment_rate: Rational32SchemaProvider,
        ///Height of genesis block.
        pub genesis_height: u64,
        ///Official time of blockchain start.
        pub genesis_time: chrono::DateTime<chrono::offset::Utc>,
        ///Maximum gas price.
        pub max_gas_price: ::std::string::String,
        ///Maximum inflation on the total supply every epoch.
        pub max_inflation_rate: Rational32SchemaProvider,
        ///Max stake percentage of the validators we will kick out.
        pub max_kickout_stake_perc: u8,
        ///Minimum gas price. It is also the initial gas price.
        pub min_gas_price: ::std::string::String,
        ///The minimum stake required for staking is last seat price divided by
        /// this number.
        pub minimum_stake_divisor: u64,
        ///The lowest ratio s/s_total any block producer can have. See <https://github.com/near/NEPs/pull/167> for details
        pub minimum_stake_ratio: Rational32SchemaProvider,
        ///The minimum number of validators each shard must have
        pub minimum_validators_per_shard: u64,
        ///Number of block producer seats at genesis.
        pub num_block_producer_seats: u64,
        ///Defines number of shards and number of block producer seats per each
        /// shard at genesis.
        pub num_block_producer_seats_per_shard: ::std::vec::Vec<u64>,
        ///Expected number of blocks per year
        pub num_blocks_per_year: u64,
        ///Number of validator seats for chunk only producers.
        pub num_chunk_only_producer_seats: u64,
        ///Online maximum threshold above which validator gets full reward.
        pub online_max_threshold: Rational32SchemaProvider,
        ///Online minimum threshold below which validator doesn't receive
        /// reward.
        pub online_min_threshold: Rational32SchemaProvider,
        ///Protocol treasury rate
        pub protocol_reward_rate: Rational32SchemaProvider,
        ///Protocol treasury account
        pub protocol_treasury_account: AccountId,
        ///Threshold of stake that needs to indicate that they ready for
        /// upgrade.
        pub protocol_upgrade_stake_threshold: Rational32SchemaProvider,
        ///Current Protocol Version
        pub protocol_version: u32,
        ///Runtime configuration (mostly economics constants).
        pub runtime_config: RuntimeConfigView,
        ///Layout information regarding how to split accounts to shards
        pub shard_layout: ShardLayout,
        ///If true, shuffle the chunk producers across shards. In other words,
        /// if the shard assignments were `[S_0, S_1, S_2, S_3]` where `S_i`
        /// represents the set of chunk producers for shard `i`, if this flag
        /// were true, the shard assignments might become, for example, `[S_2,
        /// S_0, S_3, S_1]`.
        pub shuffle_shard_assignment_for_chunk_producers: bool,
        ///Number of target chunk validator mandates for each shard.
        pub target_validator_mandates_per_shard: u64,
        ///Number of blocks for which a given transaction is valid
        pub transaction_validity_period: u64,
    }

    impl ::std::convert::From<&RpcProtocolConfigResponse> for RpcProtocolConfigResponse {
        fn from(value: &RpcProtocolConfigResponse) -> Self {
            value.clone()
        }
    }

    ///RpcReceiptResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "predecessor_id",
    ///    "receipt",
    ///    "receipt_id",
    ///    "receiver_id"
    ///  ],
    ///  "properties": {
    ///    "predecessor_id": {
    ///      "$ref": "#/components/schemas/AccountId"
    ///    },
    ///    "priority": {
    ///      "default": 0,
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "receipt": {
    ///      "$ref": "#/components/schemas/ReceiptEnumView"
    ///    },
    ///    "receipt_id": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "receiver_id": {
    ///      "$ref": "#/components/schemas/AccountId"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RpcReceiptResponse {
        pub predecessor_id: AccountId,
        #[serde(default)]
        pub priority: u64,
        pub receipt: ReceiptEnumView,
        pub receipt_id: CryptoHash,
        pub receiver_id: AccountId,
    }

    impl ::std::convert::From<&RpcReceiptResponse> for RpcReceiptResponse {
        fn from(value: &RpcReceiptResponse) -> Self {
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

    ///RpcSendTransactionRequest
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "signed_tx_base64"
    ///  ],
    ///  "properties": {
    ///    "signed_tx_base64": {
    ///      "type": "string"
    ///    },
    ///    "wait_until": {
    ///      "default": "EXECUTED_OPTIMISTIC",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/TxExecutionStatus"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RpcSendTransactionRequest {
        pub signed_tx_base64: ::std::string::String,
        #[serde(default = "defaults::rpc_send_transaction_request_wait_until")]
        pub wait_until: TxExecutionStatus,
    }

    impl ::std::convert::From<&RpcSendTransactionRequest> for RpcSendTransactionRequest {
        fn from(value: &RpcSendTransactionRequest) -> Self {
            value.clone()
        }
    }

    ///RpcStateChangesInBlockByTypeResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "block_hash",
    ///    "changes"
    ///  ],
    ///  "properties": {
    ///    "block_hash": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "changes": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/StateChangeKindView"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RpcStateChangesInBlockByTypeResponse {
        pub block_hash: CryptoHash,
        pub changes: ::std::vec::Vec<StateChangeKindView>,
    }

    impl ::std::convert::From<&RpcStateChangesInBlockByTypeResponse>
        for RpcStateChangesInBlockByTypeResponse
    {
        fn from(value: &RpcStateChangesInBlockByTypeResponse) -> Self {
            value.clone()
        }
    }

    ///RpcStateChangesInBlockResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "block_hash",
    ///    "changes"
    ///  ],
    ///  "properties": {
    ///    "block_hash": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "changes": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/StateChangeWithCauseView"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RpcStateChangesInBlockResponse {
        pub block_hash: CryptoHash,
        pub changes: ::std::vec::Vec<StateChangeWithCauseView>,
    }

    impl ::std::convert::From<&RpcStateChangesInBlockResponse> for RpcStateChangesInBlockResponse {
        fn from(value: &RpcStateChangesInBlockResponse) -> Self {
            value.clone()
        }
    }

    ///RpcStatusRequest
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object"
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct RpcStatusRequest(pub ::serde_json::Map<::std::string::String, ::serde_json::Value>);
    impl ::std::ops::Deref for RpcStatusRequest {
        type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
        fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
            &self.0
        }
    }

    impl ::std::convert::From<RpcStatusRequest>
        for ::serde_json::Map<::std::string::String, ::serde_json::Value>
    {
        fn from(value: RpcStatusRequest) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&RpcStatusRequest> for RpcStatusRequest {
        fn from(value: &RpcStatusRequest) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
        for RpcStatusRequest
    {
        fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
            Self(value)
        }
    }

    ///RpcStatusResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "chain_id",
    ///    "genesis_hash",
    ///    "latest_protocol_version",
    ///    "node_public_key",
    ///    "protocol_version",
    ///    "sync_info",
    ///    "uptime_sec",
    ///    "validators",
    ///    "version"
    ///  ],
    ///  "properties": {
    ///    "chain_id": {
    ///      "description": "Unique chain id.",
    ///      "type": "string"
    ///    },
    ///    "detailed_debug_status": {
    ///      "description": "Information about last blocks, network, epoch and
    /// chain & chunk info.",
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/DetailedDebugStatus"
    ///            }
    ///          ]
    ///        }
    ///      ]
    ///    },
    ///    "genesis_hash": {
    ///      "description": "Genesis hash of the chain.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/CryptoHash"
    ///        }
    ///      ]
    ///    },
    ///    "latest_protocol_version": {
    ///      "description": "Latest protocol version that this client
    /// supports.",
    ///      "type": "integer",
    ///      "format": "uint32",
    ///      "minimum": 0.0
    ///    },
    ///    "node_key": {
    ///      "description": "Deprecated; same as `validator_public_key` which
    /// you should use instead.",
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/PublicKey"
    ///            }
    ///          ]
    ///        }
    ///      ]
    ///    },
    ///    "node_public_key": {
    ///      "description": "Public key of the node.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/PublicKey"
    ///        }
    ///      ]
    ///    },
    ///    "protocol_version": {
    ///      "description": "Currently active protocol version.",
    ///      "type": "integer",
    ///      "format": "uint32",
    ///      "minimum": 0.0
    ///    },
    ///    "rpc_addr": {
    ///      "description": "Address for RPC server.  None if node doesn’t have
    /// RPC endpoint enabled.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "sync_info": {
    ///      "description": "Sync status of the node.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/StatusSyncInfo"
    ///        }
    ///      ]
    ///    },
    ///    "uptime_sec": {
    ///      "description": "Uptime of the node.",
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "validator_account_id": {
    ///      "description": "Validator id of the node",
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/AccountId"
    ///            }
    ///          ]
    ///        }
    ///      ]
    ///    },
    ///    "validator_public_key": {
    ///      "description": "Public key of the validator.",
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/PublicKey"
    ///            }
    ///          ]
    ///        }
    ///      ]
    ///    },
    ///    "validators": {
    ///      "description": "Current epoch validators.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ValidatorInfo"
    ///      }
    ///    },
    ///    "version": {
    ///      "description": "Binary version.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Version"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RpcStatusResponse {
        ///Unique chain id.
        pub chain_id: ::std::string::String,
        ///Information about last blocks, network, epoch and chain & chunk
        /// info.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub detailed_debug_status: ::std::option::Option<DetailedDebugStatus>,
        ///Genesis hash of the chain.
        pub genesis_hash: CryptoHash,
        ///Latest protocol version that this client supports.
        pub latest_protocol_version: u32,
        ///Deprecated; same as `validator_public_key` which you should use
        /// instead.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub node_key: ::std::option::Option<PublicKey>,
        ///Public key of the node.
        pub node_public_key: PublicKey,
        ///Currently active protocol version.
        pub protocol_version: u32,
        ///Address for RPC server.  None if node doesn’t have RPC endpoint
        /// enabled.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub rpc_addr: ::std::option::Option<::std::string::String>,
        ///Sync status of the node.
        pub sync_info: StatusSyncInfo,
        ///Uptime of the node.
        pub uptime_sec: i64,
        ///Validator id of the node
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub validator_account_id: ::std::option::Option<AccountId>,
        ///Public key of the validator.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub validator_public_key: ::std::option::Option<PublicKey>,
        ///Current epoch validators.
        pub validators: ::std::vec::Vec<ValidatorInfo>,
        ///Binary version.
        pub version: Version,
    }

    impl ::std::convert::From<&RpcStatusResponse> for RpcStatusResponse {
        fn from(value: &RpcStatusResponse) -> Self {
            value.clone()
        }
    }

    ///RpcTransactionResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "anyOf": [
    ///    {
    ///      "$ref": "#/components/schemas/FinalExecutionOutcomeWithReceiptView"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/FinalExecutionOutcomeView"
    ///    }
    ///  ],
    ///  "required": [
    ///    "final_execution_status"
    ///  ],
    ///  "properties": {
    ///    "final_execution_status": {
    ///      "$ref": "#/components/schemas/TxExecutionStatus"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum RpcTransactionResponse {
        Variant0 {
            final_execution_status: TxExecutionStatus,
            ///Receipts generated from the transaction
            receipts: ::std::vec::Vec<ReceiptView>,
            ///The execution outcome of receipts.
            receipts_outcome: ::std::vec::Vec<ExecutionOutcomeWithIdView>,
            ///Execution status defined by
            /// chain.rs:get_final_transaction_result
            /// FinalExecutionStatus::NotStarted - the tx is not converted to
            /// the receipt yet FinalExecutionStatus::Started - we have at least
            /// 1 receipt, but the first leaf receipt_id (using dfs) hasn't
            /// finished the execution FinalExecutionStatus::Failure - the
            /// result of the first leaf receipt_id
            /// FinalExecutionStatus::SuccessValue - the result of the first
            /// leaf receipt_id
            status: FinalExecutionStatus,
            ///Signed Transaction
            transaction: SignedTransactionView,
            ///The execution outcome of the signed transaction.
            transaction_outcome: ExecutionOutcomeWithIdView,
        },
        Variant1 {
            final_execution_status: TxExecutionStatus,
            ///The execution outcome of receipts.
            receipts_outcome: ::std::vec::Vec<ExecutionOutcomeWithIdView>,
            ///Execution status defined by
            /// chain.rs:get_final_transaction_result
            /// FinalExecutionStatus::NotStarted - the tx is not converted to
            /// the receipt yet FinalExecutionStatus::Started - we have at least
            /// 1 receipt, but the first leaf receipt_id (using dfs) hasn't
            /// finished the execution FinalExecutionStatus::Failure - the
            /// result of the first leaf receipt_id
            /// FinalExecutionStatus::SuccessValue - the result of the first
            /// leaf receipt_id
            status: FinalExecutionStatus,
            ///Signed Transaction
            transaction: SignedTransactionView,
            ///The execution outcome of the signed transaction.
            transaction_outcome: ExecutionOutcomeWithIdView,
        },
    }

    impl ::std::convert::From<&Self> for RpcTransactionResponse {
        fn from(value: &RpcTransactionResponse) -> Self {
            value.clone()
        }
    }

    ///RpcTransactionStatusRequest
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "anyOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SignedTransaction"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "sender_account_id",
    ///        "tx_hash"
    ///      ],
    ///      "properties": {
    ///        "sender_account_id": {
    ///          "$ref": "#/components/schemas/AccountId"
    ///        },
    ///        "tx_hash": {
    ///          "$ref": "#/components/schemas/CryptoHash"
    ///        }
    ///      }
    ///    }
    ///  ],
    ///  "properties": {
    ///    "wait_until": {
    ///      "default": "EXECUTED_OPTIMISTIC",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/TxExecutionStatus"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged, deny_unknown_fields)]
    pub enum RpcTransactionStatusRequest {
        Variant0 {
            signed_tx_base64: ::std::string::String,
        },
        Variant1 {
            sender_account_id: AccountId,
            tx_hash: CryptoHash,
            #[serde(default = "defaults::rpc_transaction_status_request_variant1_wait_until")]
            wait_until: TxExecutionStatus,
        },
    }

    impl ::std::convert::From<&Self> for RpcTransactionStatusRequest {
        fn from(value: &RpcTransactionStatusRequest) -> Self {
            value.clone()
        }
    }

    ///RpcValidatorRequest
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "oneOf": [
    ///    {
    ///      "type": "string",
    ///      "enum": [
    ///        "latest"
    ///      ]
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "epoch_id"
    ///      ],
    ///      "properties": {
    ///        "epoch_id": {
    ///          "$ref": "#/components/schemas/EpochId"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "block_id"
    ///      ],
    ///      "properties": {
    ///        "block_id": {
    ///          "$ref": "#/components/schemas/BlockId"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub enum RpcValidatorRequest {
        #[serde(rename = "latest")]
        Latest,
        #[serde(rename = "epoch_id")]
        EpochId(EpochId),
        #[serde(rename = "block_id")]
        BlockId(BlockId),
    }

    impl ::std::convert::From<&Self> for RpcValidatorRequest {
        fn from(value: &RpcValidatorRequest) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<EpochId> for RpcValidatorRequest {
        fn from(value: EpochId) -> Self {
            Self::EpochId(value)
        }
    }

    impl ::std::convert::From<BlockId> for RpcValidatorRequest {
        fn from(value: BlockId) -> Self {
            Self::BlockId(value)
        }
    }

    ///Information about this epoch validators and next epoch validators
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Information about this epoch validators and next epoch
    /// validators",
    ///  "type": "object",
    ///  "required": [
    ///    "current_fishermen",
    ///    "current_proposals",
    ///    "current_validators",
    ///    "epoch_height",
    ///    "epoch_start_height",
    ///    "next_fishermen",
    ///    "next_validators",
    ///    "prev_epoch_kickout"
    ///  ],
    ///  "properties": {
    ///    "current_fishermen": {
    ///      "description": "Fishermen for the current epoch",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ValidatorStakeView"
    ///      }
    ///    },
    ///    "current_proposals": {
    ///      "description": "Proposals in the current epoch",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ValidatorStakeView"
    ///      }
    ///    },
    ///    "current_validators": {
    ///      "description": "Validators for the current epoch",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/CurrentEpochValidatorInfo"
    ///      }
    ///    },
    ///    "epoch_height": {
    ///      "description": "Epoch height",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "epoch_start_height": {
    ///      "description": "Epoch start block height",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "next_fishermen": {
    ///      "description": "Fishermen for the next epoch",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ValidatorStakeView"
    ///      }
    ///    },
    ///    "next_validators": {
    ///      "description": "Validators for the next epoch",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/NextEpochValidatorInfo"
    ///      }
    ///    },
    ///    "prev_epoch_kickout": {
    ///      "description": "Kickout in the previous epoch",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ValidatorKickoutView"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RpcValidatorResponse {
        ///Fishermen for the current epoch
        pub current_fishermen: ::std::vec::Vec<ValidatorStakeView>,
        ///Proposals in the current epoch
        pub current_proposals: ::std::vec::Vec<ValidatorStakeView>,
        ///Validators for the current epoch
        pub current_validators: ::std::vec::Vec<CurrentEpochValidatorInfo>,
        ///Epoch height
        pub epoch_height: u64,
        ///Epoch start block height
        pub epoch_start_height: u64,
        ///Fishermen for the next epoch
        pub next_fishermen: ::std::vec::Vec<ValidatorStakeView>,
        ///Validators for the next epoch
        pub next_validators: ::std::vec::Vec<NextEpochValidatorInfo>,
        ///Kickout in the previous epoch
        pub prev_epoch_kickout: ::std::vec::Vec<ValidatorKickoutView>,
    }

    impl ::std::convert::From<&RpcValidatorResponse> for RpcValidatorResponse {
        fn from(value: &RpcValidatorResponse) -> Self {
            value.clone()
        }
    }

    ///View that preserves JSON format of the runtime config.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "View that preserves JSON format of the runtime
    /// config.",
    ///  "type": "object",
    ///  "required": [
    ///    "account_creation_config",
    ///    "congestion_control_config",
    ///    "storage_amount_per_byte",
    ///    "transaction_costs",
    ///    "wasm_config",
    ///    "witness_config"
    ///  ],
    ///  "properties": {
    ///    "account_creation_config": {
    ///      "description": "Config that defines rules for account creation.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/AccountCreationConfigView"
    ///        }
    ///      ]
    ///    },
    ///    "congestion_control_config": {
    ///      "description": "The configuration for congestion control.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/CongestionControlConfigView"
    ///        }
    ///      ]
    ///    },
    ///    "storage_amount_per_byte": {
    ///      "description": "Amount of yN per byte required to have on the account.  See <https://nomicon.io/Economics/Economic#state-stake> for details.",
    ///      "type": "string"
    ///    },
    ///    "transaction_costs": {
    ///      "description": "Costs of different actions that need to be
    /// performed when sending and processing transaction and receipts.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/RuntimeFeesConfigView"
    ///        }
    ///      ]
    ///    },
    ///    "wasm_config": {
    ///      "description": "Config of wasm operations.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/VMConfigView"
    ///        }
    ///      ]
    ///    },
    ///    "witness_config": {
    ///      "description": "Configuration specific to ChunkStateWitness.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/WitnessConfigView"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RuntimeConfigView {
        ///Config that defines rules for account creation.
        pub account_creation_config: AccountCreationConfigView,
        ///The configuration for congestion control.
        pub congestion_control_config: CongestionControlConfigView,
        ///Amount of yN per byte required to have on the account.  See <https://nomicon.io/Economics/Economic#state-stake> for details.
        pub storage_amount_per_byte: ::std::string::String,
        ///Costs of different actions that need to be performed when sending
        /// and processing transaction and receipts.
        pub transaction_costs: RuntimeFeesConfigView,
        ///Config of wasm operations.
        pub wasm_config: VmConfigView,
        ///Configuration specific to ChunkStateWitness.
        pub witness_config: WitnessConfigView,
    }

    impl ::std::convert::From<&RuntimeConfigView> for RuntimeConfigView {
        fn from(value: &RuntimeConfigView) -> Self {
            value.clone()
        }
    }

    ///RuntimeFeesConfigView
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "action_creation_config",
    ///    "action_receipt_creation_config",
    ///    "burnt_gas_reward",
    ///    "data_receipt_creation_config",
    ///    "pessimistic_gas_price_inflation_ratio",
    ///    "storage_usage_config"
    ///  ],
    ///  "properties": {
    ///    "action_creation_config": {
    ///      "description": "Describes the cost of creating a certain action,
    /// `Action`. Includes all variants.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/ActionCreationConfigView"
    ///        }
    ///      ]
    ///    },
    ///    "action_receipt_creation_config": {
    ///      "description": "Describes the cost of creating an action receipt,
    /// `ActionReceipt`, excluding the actual cost of actions. - `send` cost is
    /// burned when a receipt is created using `promise_create` or
    /// `promise_batch_create` - `exec` cost is burned when the receipt is being
    /// executed.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Fee"
    ///        }
    ///      ]
    ///    },
    ///    "burnt_gas_reward": {
    ///      "description": "Fraction of the burnt gas to reward to the contract
    /// account for execution.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Rational32SchemaProvider"
    ///        }
    ///      ]
    ///    },
    ///    "data_receipt_creation_config": {
    ///      "description": "Describes the cost of creating a data receipt,
    /// `DataReceipt`.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/DataReceiptCreationConfigView"
    ///        }
    ///      ]
    ///    },
    ///    "pessimistic_gas_price_inflation_ratio": {
    ///      "description": "Pessimistic gas price inflation ratio.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Rational32SchemaProvider"
    ///        }
    ///      ]
    ///    },
    ///    "storage_usage_config": {
    ///      "description": "Describes fees for storage.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/StorageUsageConfigView"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RuntimeFeesConfigView {
        ///Describes the cost of creating a certain action, `Action`. Includes
        /// all variants.
        pub action_creation_config: ActionCreationConfigView,
        ///Describes the cost of creating an action receipt, `ActionReceipt`,
        /// excluding the actual cost of actions. - `send` cost is burned when a
        /// receipt is created using `promise_create` or `promise_batch_create`
        /// - `exec` cost is burned when the receipt is being executed.
        pub action_receipt_creation_config: Fee,
        ///Fraction of the burnt gas to reward to the contract account for
        /// execution.
        pub burnt_gas_reward: Rational32SchemaProvider,
        ///Describes the cost of creating a data receipt, `DataReceipt`.
        pub data_receipt_creation_config: DataReceiptCreationConfigView,
        ///Pessimistic gas price inflation ratio.
        pub pessimistic_gas_price_inflation_ratio: Rational32SchemaProvider,
        ///Describes fees for storage.
        pub storage_usage_config: StorageUsageConfigView,
    }

    impl ::std::convert::From<&RuntimeFeesConfigView> for RuntimeFeesConfigView {
        fn from(value: &RuntimeFeesConfigView) -> Self {
            value.clone()
        }
    }

    ///SendTxMethodNameHelperEnum
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "send_tx"
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
    pub enum SendTxMethodNameHelperEnum {
        #[serde(rename = "send_tx")]
        SendTx,
    }

    impl ::std::convert::From<&Self> for SendTxMethodNameHelperEnum {
        fn from(value: &SendTxMethodNameHelperEnum) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for SendTxMethodNameHelperEnum {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::SendTx => write!(f, "send_tx"),
            }
        }
    }

    impl ::std::str::FromStr for SendTxMethodNameHelperEnum {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "send_tx" => Ok(Self::SendTx),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for SendTxMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for SendTxMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for SendTxMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///The shard identifier. It may be an arbitrary number - it does not need
    /// to be a number in the range 0..NUM_SHARDS. The shard ids do not need to
    /// be sequential or contiguous.
    ///
    ///The shard id is wrapped in a new type to prevent the old pattern of
    /// using indices in range 0..NUM_SHARDS and casting to ShardId. Once the
    /// transition if fully complete it potentially may be simplified to a
    /// regular type alias.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The shard identifier. It may be an arbitrary number -
    /// it does not need to be a number in the range 0..NUM_SHARDS. The shard
    /// ids do not need to be sequential or contiguous.\n\nThe shard id is
    /// wrapped in a new type to prevent the old pattern of using indices in
    /// range 0..NUM_SHARDS and casting to ShardId. Once the transition if fully
    /// complete it potentially may be simplified to a regular type alias.",
    ///  "type": "integer",
    ///  "format": "uint64",
    ///  "minimum": 0.0
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct ShardId(pub u64);
    impl ::std::ops::Deref for ShardId {
        type Target = u64;
        fn deref(&self) -> &u64 {
            &self.0
        }
    }

    impl ::std::convert::From<ShardId> for u64 {
        fn from(value: ShardId) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&ShardId> for ShardId {
        fn from(value: &ShardId) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<u64> for ShardId {
        fn from(value: u64) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for ShardId {
        type Err = <u64 as ::std::str::FromStr>::Err;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl ::std::convert::TryFrom<&str> for ShardId {
        type Error = <u64 as ::std::str::FromStr>::Err;
        fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&String> for ShardId {
        type Error = <u64 as ::std::str::FromStr>::Err;
        fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for ShardId {
        type Error = <u64 as ::std::str::FromStr>::Err;
        fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::fmt::Display for ShardId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    ///A versioned struct that contains all information needed to assign
    /// accounts to shards.
    ///
    ///Because of re-sharding, the chain may use different shard layout to
    /// split shards at different times. Currently, `ShardLayout` is stored as
    /// part of `EpochConfig`, which is generated each epoch given the epoch
    /// protocol version. In mainnet/testnet, we use two shard layouts since
    /// re-sharding has only happened once. It is stored as part of genesis
    /// config, see default_simple_nightshade_shard_layout() Below is an
    /// overview for some important functionalities of ShardLayout interface.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A versioned struct that contains all information needed
    /// to assign accounts to shards.\n\nBecause of re-sharding, the chain may
    /// use different shard layout to split shards at different times.
    /// Currently, `ShardLayout` is stored as part of `EpochConfig`, which is
    /// generated each epoch given the epoch protocol version. In
    /// mainnet/testnet, we use two shard layouts since re-sharding has only
    /// happened once. It is stored as part of genesis config, see
    /// default_simple_nightshade_shard_layout() Below is an overview for some
    /// important functionalities of ShardLayout interface.",
    ///  "oneOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "V0"
    ///      ],
    ///      "properties": {
    ///        "V0": {
    ///          "$ref": "#/components/schemas/ShardLayoutV0"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "V1"
    ///      ],
    ///      "properties": {
    ///        "V1": {
    ///          "$ref": "#/components/schemas/ShardLayoutV1"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "V2"
    ///      ],
    ///      "properties": {
    ///        "V2": {
    ///          "$ref": "#/components/schemas/ShardLayoutV2"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub enum ShardLayout {
        V0(ShardLayoutV0),
        V1(ShardLayoutV1),
        V2(ShardLayoutV2),
    }

    impl ::std::convert::From<&Self> for ShardLayout {
        fn from(value: &ShardLayout) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<ShardLayoutV0> for ShardLayout {
        fn from(value: ShardLayoutV0) -> Self {
            Self::V0(value)
        }
    }

    impl ::std::convert::From<ShardLayoutV1> for ShardLayout {
        fn from(value: ShardLayoutV1) -> Self {
            Self::V1(value)
        }
    }

    impl ::std::convert::From<ShardLayoutV2> for ShardLayout {
        fn from(value: ShardLayoutV2) -> Self {
            Self::V2(value)
        }
    }

    ///A shard layout that maps accounts evenly across all shards -- by
    /// calculate the hash of account id and mod number of shards. This is added
    /// to capture the old `account_id_to_shard_id` algorithm, to keep backward
    /// compatibility for some existing tests. `parent_shards` for
    /// `ShardLayoutV1` is always `None`, meaning it can only be the first shard
    /// layout a chain uses.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A shard layout that maps accounts evenly across all
    /// shards -- by calculate the hash of account id and mod number of shards.
    /// This is added to capture the old `account_id_to_shard_id` algorithm, to
    /// keep backward compatibility for some existing tests. `parent_shards` for
    /// `ShardLayoutV1` is always `None`, meaning it can only be the first shard
    /// layout a chain uses.",
    ///  "type": "object",
    ///  "required": [
    ///    "num_shards",
    ///    "version"
    ///  ],
    ///  "properties": {
    ///    "num_shards": {
    ///      "description": "Map accounts evenly across all shards",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "version": {
    ///      "description": "Version of the shard layout, this is useful for
    /// uniquely identify the shard layout",
    ///      "type": "integer",
    ///      "format": "uint32",
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ShardLayoutV0 {
        ///Map accounts evenly across all shards
        pub num_shards: u64,
        ///Version of the shard layout, this is useful for uniquely identify
        /// the shard layout
        pub version: u32,
    }

    impl ::std::convert::From<&ShardLayoutV0> for ShardLayoutV0 {
        fn from(value: &ShardLayoutV0) -> Self {
            value.clone()
        }
    }

    ///ShardLayoutV1
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "boundary_accounts",
    ///    "version"
    ///  ],
    ///  "properties": {
    ///    "boundary_accounts": {
    ///      "description": "The boundary accounts are the accounts on
    /// boundaries between shards. Each shard contains a range of accounts from
    /// one boundary account to another - or the smallest or largest account
    /// possible. The total number of shards is equal to the number of boundary
    /// accounts plus 1.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/AccountId"
    ///      }
    ///    },
    ///    "shards_split_map": {
    ///      "description": "Maps shards from the last shard layout to shards
    /// that it splits to in this shard layout, Useful for constructing states
    /// for the shards. None for the genesis shard layout",
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "array",
    ///        "items": {
    ///          "$ref": "#/components/schemas/ShardId"
    ///        }
    ///      }
    ///    },
    ///    "to_parent_shard_map": {
    ///      "description": "Maps shard in this shard layout to their parent
    /// shard Since shard_ids always range from 0 to num_shards - 1, we use vec
    /// instead of a hashmap",
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "$ref": "#/components/schemas/ShardId"
    ///      }
    ///    },
    ///    "version": {
    ///      "description": "Version of the shard layout, this is useful for
    /// uniquely identify the shard layout",
    ///      "type": "integer",
    ///      "format": "uint32",
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ShardLayoutV1 {
        ///The boundary accounts are the accounts on boundaries between shards.
        /// Each shard contains a range of accounts from one boundary account to
        /// another - or the smallest or largest account possible. The total
        /// number of shards is equal to the number of boundary accounts plus 1.
        pub boundary_accounts: ::std::vec::Vec<AccountId>,
        ///Maps shards from the last shard layout to shards that it splits to
        /// in this shard layout, Useful for constructing states for the shards.
        /// None for the genesis shard layout
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub shards_split_map: ::std::option::Option<::std::vec::Vec<::std::vec::Vec<ShardId>>>,
        ///Maps shard in this shard layout to their parent shard Since
        /// shard_ids always range from 0 to num_shards - 1, we use vec instead
        /// of a hashmap
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub to_parent_shard_map: ::std::option::Option<::std::vec::Vec<ShardId>>,
        ///Version of the shard layout, this is useful for uniquely identify
        /// the shard layout
        pub version: u32,
    }

    impl ::std::convert::From<&ShardLayoutV1> for ShardLayoutV1 {
        fn from(value: &ShardLayoutV1) -> Self {
            value.clone()
        }
    }

    ///Counterpart to `ShardLayoutV2` composed of maps with string keys to aid
    /// serde serialization.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Counterpart to `ShardLayoutV2` composed of maps with
    /// string keys to aid serde serialization.",
    ///  "type": "object",
    ///  "required": [
    ///    "boundary_accounts",
    ///    "id_to_index_map",
    ///    "index_to_id_map",
    ///    "shard_ids",
    ///    "version"
    ///  ],
    ///  "properties": {
    ///    "boundary_accounts": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/AccountId"
    ///      }
    ///    },
    ///    "id_to_index_map": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "integer",
    ///        "format": "uint",
    ///        "minimum": 0.0
    ///      }
    ///    },
    ///    "index_to_id_map": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "$ref": "#/components/schemas/ShardId"
    ///      }
    ///    },
    ///    "shard_ids": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ShardId"
    ///      }
    ///    },
    ///    "shards_parent_map": {
    ///      "type": [
    ///        "object",
    ///        "null"
    ///      ],
    ///      "additionalProperties": {
    ///        "$ref": "#/components/schemas/ShardId"
    ///      }
    ///    },
    ///    "shards_split_map": {
    ///      "type": [
    ///        "object",
    ///        "null"
    ///      ],
    ///      "additionalProperties": {
    ///        "type": "array",
    ///        "items": {
    ///          "$ref": "#/components/schemas/ShardId"
    ///        }
    ///      }
    ///    },
    ///    "version": {
    ///      "type": "integer",
    ///      "format": "uint32",
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ShardLayoutV2 {
        pub boundary_accounts: ::std::vec::Vec<AccountId>,
        pub id_to_index_map: ::std::collections::HashMap<::std::string::String, u32>,
        pub index_to_id_map: ::std::collections::HashMap<::std::string::String, ShardId>,
        pub shard_ids: ::std::vec::Vec<ShardId>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub shards_parent_map:
            ::std::option::Option<::std::collections::HashMap<::std::string::String, ShardId>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub shards_split_map: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::vec::Vec<ShardId>>,
        >,
        pub version: u32,
    }

    impl ::std::convert::From<&ShardLayoutV2> for ShardLayoutV2 {
        fn from(value: &ShardLayoutV2) -> Self {
            value.clone()
        }
    }

    ///Signature
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
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
    pub struct Signature(pub ::std::string::String);
    impl ::std::ops::Deref for Signature {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<Signature> for ::std::string::String {
        fn from(value: Signature) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&Signature> for Signature {
        fn from(value: &Signature) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<::std::string::String> for Signature {
        fn from(value: ::std::string::String) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for Signature {
        type Err = ::std::convert::Infallible;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::fmt::Display for Signature {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    ///SignedDelegateAction
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "delegate_action",
    ///    "signature"
    ///  ],
    ///  "properties": {
    ///    "delegate_action": {
    ///      "$ref": "#/components/schemas/DelegateAction"
    ///    },
    ///    "signature": {
    ///      "$ref": "#/components/schemas/Signature"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SignedDelegateAction {
        pub delegate_action: DelegateAction,
        pub signature: Signature,
    }

    impl ::std::convert::From<&SignedDelegateAction> for SignedDelegateAction {
        fn from(value: &SignedDelegateAction) -> Self {
            value.clone()
        }
    }

    ///SignedTransaction
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "signed_tx_base64"
    ///      ],
    ///      "properties": {
    ///        "signed_tx_base64": {
    ///          "type": "string"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub enum SignedTransaction {
        #[serde(rename = "signed_tx_base64")]
        SignedTxBase64(::std::string::String),
    }

    impl ::std::convert::From<&Self> for SignedTransaction {
        fn from(value: &SignedTransaction) -> Self {
            value.clone()
        }
    }

    ///SignedTransactionView
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "actions",
    ///    "hash",
    ///    "nonce",
    ///    "public_key",
    ///    "receiver_id",
    ///    "signature",
    ///    "signer_id"
    ///  ],
    ///  "properties": {
    ///    "actions": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ActionView"
    ///      }
    ///    },
    ///    "hash": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "nonce": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "priority_fee": {
    ///      "default": 0,
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "public_key": {
    ///      "$ref": "#/components/schemas/PublicKey"
    ///    },
    ///    "receiver_id": {
    ///      "$ref": "#/components/schemas/AccountId"
    ///    },
    ///    "signature": {
    ///      "$ref": "#/components/schemas/Signature"
    ///    },
    ///    "signer_id": {
    ///      "$ref": "#/components/schemas/AccountId"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SignedTransactionView {
        pub actions: ::std::vec::Vec<ActionView>,
        pub hash: CryptoHash,
        pub nonce: u64,
        #[serde(default)]
        pub priority_fee: u64,
        pub public_key: PublicKey,
        pub receiver_id: AccountId,
        pub signature: Signature,
        pub signer_id: AccountId,
    }

    impl ::std::convert::From<&SignedTransactionView> for SignedTransactionView {
        fn from(value: &SignedTransactionView) -> Self {
            value.clone()
        }
    }

    ///SlashedValidator
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "account_id",
    ///    "is_double_sign"
    ///  ],
    ///  "properties": {
    ///    "account_id": {
    ///      "$ref": "#/components/schemas/AccountId"
    ///    },
    ///    "is_double_sign": {
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SlashedValidator {
        pub account_id: AccountId,
        pub is_double_sign: bool,
    }

    impl ::std::convert::From<&SlashedValidator> for SlashedValidator {
        fn from(value: &SlashedValidator) -> Self {
            value.clone()
        }
    }

    ///An action which stakes signer_id tokens and setup's validator public key
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "An action which stakes signer_id tokens and setup's
    /// validator public key",
    ///  "type": "object",
    ///  "required": [
    ///    "public_key",
    ///    "stake"
    ///  ],
    ///  "properties": {
    ///    "public_key": {
    ///      "description": "Validator key which will be used to sign
    /// transactions on behalf of signer_id",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/PublicKey"
    ///        }
    ///      ]
    ///    },
    ///    "stake": {
    ///      "description": "Amount of tokens to stake.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct StakeAction {
        ///Validator key which will be used to sign transactions on behalf of
        /// signer_id
        pub public_key: PublicKey,
        ///Amount of tokens to stake.
        pub stake: ::std::string::String,
    }

    impl ::std::convert::From<&StakeAction> for StakeAction {
        fn from(value: &StakeAction) -> Self {
            value.clone()
        }
    }

    ///See crate::types::StateChangeCause for details.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "See crate::types::StateChangeCause for details.",
    ///  "oneOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "not_writable_to_disk"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "initial_state"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "tx_hash",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "tx_hash": {
    ///          "$ref": "#/components/schemas/CryptoHash"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "transaction_processing"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "receipt_hash",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "receipt_hash": {
    ///          "$ref": "#/components/schemas/CryptoHash"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "action_receipt_processing_started"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "receipt_hash",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "receipt_hash": {
    ///          "$ref": "#/components/schemas/CryptoHash"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "action_receipt_gas_reward"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "receipt_hash",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "receipt_hash": {
    ///          "$ref": "#/components/schemas/CryptoHash"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "receipt_processing"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "receipt_hash",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "receipt_hash": {
    ///          "$ref": "#/components/schemas/CryptoHash"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "postponed_receipt"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "updated_delayed_receipts"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "validator_accounts_update"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "migration"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "resharding_v2"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "bandwidth_scheduler_state_update"
    ///          ]
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(tag = "type")]
    pub enum StateChangeCauseView {
        #[serde(rename = "not_writable_to_disk")]
        NotWritableToDisk,
        #[serde(rename = "initial_state")]
        InitialState,
        #[serde(rename = "transaction_processing")]
        TransactionProcessing { tx_hash: CryptoHash },
        #[serde(rename = "action_receipt_processing_started")]
        ActionReceiptProcessingStarted { receipt_hash: CryptoHash },
        #[serde(rename = "action_receipt_gas_reward")]
        ActionReceiptGasReward { receipt_hash: CryptoHash },
        #[serde(rename = "receipt_processing")]
        ReceiptProcessing { receipt_hash: CryptoHash },
        #[serde(rename = "postponed_receipt")]
        PostponedReceipt { receipt_hash: CryptoHash },
        #[serde(rename = "updated_delayed_receipts")]
        UpdatedDelayedReceipts,
        #[serde(rename = "validator_accounts_update")]
        ValidatorAccountsUpdate,
        #[serde(rename = "migration")]
        Migration,
        #[serde(rename = "resharding_v2")]
        ReshardingV2,
        #[serde(rename = "bandwidth_scheduler_state_update")]
        BandwidthSchedulerStateUpdate,
    }

    impl ::std::convert::From<&Self> for StateChangeCauseView {
        fn from(value: &StateChangeCauseView) -> Self {
            value.clone()
        }
    }

    ///It is a [serializable view] of [`StateChangeKind`].
    ///
    ///[serializable view]: ./index.html [`StateChangeKind`]: ../types/struct.StateChangeKind.html
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "It is a [serializable view] of
    /// [`StateChangeKind`].\n\n[serializable view]: ./index.html
    /// [`StateChangeKind`]: ../types/struct.StateChangeKind.html",
    ///  "oneOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "account_id",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "account_id": {
    ///          "$ref": "#/components/schemas/AccountId"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "account_touched"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "account_id",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "account_id": {
    ///          "$ref": "#/components/schemas/AccountId"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "access_key_touched"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "account_id",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "account_id": {
    ///          "$ref": "#/components/schemas/AccountId"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "data_touched"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "account_id",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "account_id": {
    ///          "$ref": "#/components/schemas/AccountId"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "contract_code_touched"
    ///          ]
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(tag = "type", content = "account_id")]
    pub enum StateChangeKindView {
        #[serde(rename = "account_touched")]
        AccountTouched(AccountId),
        #[serde(rename = "access_key_touched")]
        AccessKeyTouched(AccountId),
        #[serde(rename = "data_touched")]
        DataTouched(AccountId),
        #[serde(rename = "contract_code_touched")]
        ContractCodeTouched(AccountId),
    }

    impl ::std::convert::From<&Self> for StateChangeKindView {
        fn from(value: &StateChangeKindView) -> Self {
            value.clone()
        }
    }

    ///StateChangeWithCauseView
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "oneOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "change",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "change": {
    ///          "description": "A view of the account",
    ///          "type": "object",
    ///          "required": [
    ///            "account_id",
    ///            "amount",
    ///            "code_hash",
    ///            "locked",
    ///            "storage_usage"
    ///          ],
    ///          "properties": {
    ///            "account_id": {
    ///              "$ref": "#/components/schemas/AccountId"
    ///            },
    ///            "amount": {
    ///              "type": "string"
    ///            },
    ///            "code_hash": {
    ///              "$ref": "#/components/schemas/CryptoHash"
    ///            },
    ///            "global_contract_account_id": {
    ///              "oneOf": [
    ///                {
    ///                  "type": "null"
    ///                },
    ///                {
    ///                  "allOf": [
    ///                    {
    ///                      "$ref": "#/components/schemas/AccountId"
    ///                    }
    ///                  ]
    ///                }
    ///              ]
    ///            },
    ///            "global_contract_hash": {
    ///              "oneOf": [
    ///                {
    ///                  "type": "null"
    ///                },
    ///                {
    ///                  "allOf": [
    ///                    {
    ///                      "$ref": "#/components/schemas/CryptoHash"
    ///                    }
    ///                  ]
    ///                }
    ///              ]
    ///            },
    ///            "locked": {
    ///              "type": "string"
    ///            },
    ///            "storage_paid_at": {
    ///              "description": "TODO(2271): deprecated.",
    ///              "default": 0,
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            },
    ///            "storage_usage": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            }
    ///          }
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "account_update"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "change",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "change": {
    ///          "type": "object",
    ///          "required": [
    ///            "account_id"
    ///          ],
    ///          "properties": {
    ///            "account_id": {
    ///              "$ref": "#/components/schemas/AccountId"
    ///            }
    ///          }
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "account_deletion"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "change",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "change": {
    ///          "type": "object",
    ///          "required": [
    ///            "access_key",
    ///            "account_id",
    ///            "public_key"
    ///          ],
    ///          "properties": {
    ///            "access_key": {
    ///              "$ref": "#/components/schemas/AccessKeyView"
    ///            },
    ///            "account_id": {
    ///              "$ref": "#/components/schemas/AccountId"
    ///            },
    ///            "public_key": {
    ///              "$ref": "#/components/schemas/PublicKey"
    ///            }
    ///          }
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "access_key_update"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "change",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "change": {
    ///          "type": "object",
    ///          "required": [
    ///            "account_id",
    ///            "public_key"
    ///          ],
    ///          "properties": {
    ///            "account_id": {
    ///              "$ref": "#/components/schemas/AccountId"
    ///            },
    ///            "public_key": {
    ///              "$ref": "#/components/schemas/PublicKey"
    ///            }
    ///          }
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "access_key_deletion"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "change",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "change": {
    ///          "type": "object",
    ///          "required": [
    ///            "account_id",
    ///            "key_base64",
    ///            "value_base64"
    ///          ],
    ///          "properties": {
    ///            "account_id": {
    ///              "$ref": "#/components/schemas/AccountId"
    ///            },
    ///            "key_base64": {
    ///              "type": "string"
    ///            },
    ///            "value_base64": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "data_update"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "change",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "change": {
    ///          "type": "object",
    ///          "required": [
    ///            "account_id",
    ///            "key_base64"
    ///          ],
    ///          "properties": {
    ///            "account_id": {
    ///              "$ref": "#/components/schemas/AccountId"
    ///            },
    ///            "key_base64": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "data_deletion"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "change",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "change": {
    ///          "type": "object",
    ///          "required": [
    ///            "account_id",
    ///            "code_base64"
    ///          ],
    ///          "properties": {
    ///            "account_id": {
    ///              "$ref": "#/components/schemas/AccountId"
    ///            },
    ///            "code_base64": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "contract_code_update"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "change",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "change": {
    ///          "type": "object",
    ///          "required": [
    ///            "account_id"
    ///          ],
    ///          "properties": {
    ///            "account_id": {
    ///              "$ref": "#/components/schemas/AccountId"
    ///            }
    ///          }
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "contract_code_deletion"
    ///          ]
    ///        }
    ///      }
    ///    }
    ///  ],
    ///  "required": [
    ///    "cause"
    ///  ],
    ///  "properties": {
    ///    "cause": {
    ///      "$ref": "#/components/schemas/StateChangeCauseView"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum StateChangeWithCauseView {
        Variant0(StateChangeWithCauseViewVariant0),
        Variant1(StateChangeWithCauseViewVariant1),
        Variant2(StateChangeWithCauseViewVariant2),
        Variant3(StateChangeWithCauseViewVariant3),
        Variant4(StateChangeWithCauseViewVariant4),
        Variant5(StateChangeWithCauseViewVariant5),
        Variant6(StateChangeWithCauseViewVariant6),
        Variant7(StateChangeWithCauseViewVariant7),
    }

    impl ::std::convert::From<&Self> for StateChangeWithCauseView {
        fn from(value: &StateChangeWithCauseView) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<StateChangeWithCauseViewVariant0> for StateChangeWithCauseView {
        fn from(value: StateChangeWithCauseViewVariant0) -> Self {
            Self::Variant0(value)
        }
    }

    impl ::std::convert::From<StateChangeWithCauseViewVariant1> for StateChangeWithCauseView {
        fn from(value: StateChangeWithCauseViewVariant1) -> Self {
            Self::Variant1(value)
        }
    }

    impl ::std::convert::From<StateChangeWithCauseViewVariant2> for StateChangeWithCauseView {
        fn from(value: StateChangeWithCauseViewVariant2) -> Self {
            Self::Variant2(value)
        }
    }

    impl ::std::convert::From<StateChangeWithCauseViewVariant3> for StateChangeWithCauseView {
        fn from(value: StateChangeWithCauseViewVariant3) -> Self {
            Self::Variant3(value)
        }
    }

    impl ::std::convert::From<StateChangeWithCauseViewVariant4> for StateChangeWithCauseView {
        fn from(value: StateChangeWithCauseViewVariant4) -> Self {
            Self::Variant4(value)
        }
    }

    impl ::std::convert::From<StateChangeWithCauseViewVariant5> for StateChangeWithCauseView {
        fn from(value: StateChangeWithCauseViewVariant5) -> Self {
            Self::Variant5(value)
        }
    }

    impl ::std::convert::From<StateChangeWithCauseViewVariant6> for StateChangeWithCauseView {
        fn from(value: StateChangeWithCauseViewVariant6) -> Self {
            Self::Variant6(value)
        }
    }

    impl ::std::convert::From<StateChangeWithCauseViewVariant7> for StateChangeWithCauseView {
        fn from(value: StateChangeWithCauseViewVariant7) -> Self {
            Self::Variant7(value)
        }
    }

    ///StateChangeWithCauseViewVariant0
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "cause"
    ///      ],
    ///      "properties": {
    ///        "cause": {
    ///          "$ref": "#/components/schemas/StateChangeCauseView"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "change",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "change": {
    ///          "description": "A view of the account",
    ///          "type": "object",
    ///          "required": [
    ///            "account_id",
    ///            "amount",
    ///            "code_hash",
    ///            "locked",
    ///            "storage_usage"
    ///          ],
    ///          "properties": {
    ///            "account_id": {
    ///              "$ref": "#/components/schemas/AccountId"
    ///            },
    ///            "amount": {
    ///              "type": "string"
    ///            },
    ///            "code_hash": {
    ///              "$ref": "#/components/schemas/CryptoHash"
    ///            },
    ///            "global_contract_account_id": {
    ///              "oneOf": [
    ///                {
    ///                  "type": "null"
    ///                },
    ///                {
    ///                  "allOf": [
    ///                    {
    ///                      "$ref": "#/components/schemas/AccountId"
    ///                    }
    ///                  ]
    ///                }
    ///              ]
    ///            },
    ///            "global_contract_hash": {
    ///              "oneOf": [
    ///                {
    ///                  "type": "null"
    ///                },
    ///                {
    ///                  "allOf": [
    ///                    {
    ///                      "$ref": "#/components/schemas/CryptoHash"
    ///                    }
    ///                  ]
    ///                }
    ///              ]
    ///            },
    ///            "locked": {
    ///              "type": "string"
    ///            },
    ///            "storage_paid_at": {
    ///              "description": "TODO(2271): deprecated.",
    ///              "default": 0,
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            },
    ///            "storage_usage": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            }
    ///          }
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "account_update"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "change",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "change": {
    ///            "type": "object",
    ///            "required": [
    ///              "account_id"
    ///            ],
    ///            "properties": {
    ///              "account_id": {
    ///                "$ref": "#/components/schemas/AccountId"
    ///              }
    ///            }
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "account_deletion"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "change",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "change": {
    ///            "type": "object",
    ///            "required": [
    ///              "access_key",
    ///              "account_id",
    ///              "public_key"
    ///            ],
    ///            "properties": {
    ///              "access_key": {
    ///                "$ref": "#/components/schemas/AccessKeyView"
    ///              },
    ///              "account_id": {
    ///                "$ref": "#/components/schemas/AccountId"
    ///              },
    ///              "public_key": {
    ///                "$ref": "#/components/schemas/PublicKey"
    ///              }
    ///            }
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "access_key_update"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "change",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "change": {
    ///            "type": "object",
    ///            "required": [
    ///              "account_id",
    ///              "public_key"
    ///            ],
    ///            "properties": {
    ///              "account_id": {
    ///                "$ref": "#/components/schemas/AccountId"
    ///              },
    ///              "public_key": {
    ///                "$ref": "#/components/schemas/PublicKey"
    ///              }
    ///            }
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "access_key_deletion"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "change",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "change": {
    ///            "type": "object",
    ///            "required": [
    ///              "account_id",
    ///              "key_base64",
    ///              "value_base64"
    ///            ],
    ///            "properties": {
    ///              "account_id": {
    ///                "$ref": "#/components/schemas/AccountId"
    ///              },
    ///              "key_base64": {
    ///                "type": "string"
    ///              },
    ///              "value_base64": {
    ///                "type": "string"
    ///              }
    ///            }
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "data_update"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "change",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "change": {
    ///            "type": "object",
    ///            "required": [
    ///              "account_id",
    ///              "key_base64"
    ///            ],
    ///            "properties": {
    ///              "account_id": {
    ///                "$ref": "#/components/schemas/AccountId"
    ///              },
    ///              "key_base64": {
    ///                "type": "string"
    ///              }
    ///            }
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "data_deletion"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "change",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "change": {
    ///            "type": "object",
    ///            "required": [
    ///              "account_id",
    ///              "code_base64"
    ///            ],
    ///            "properties": {
    ///              "account_id": {
    ///                "$ref": "#/components/schemas/AccountId"
    ///              },
    ///              "code_base64": {
    ///                "type": "string"
    ///              }
    ///            }
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "contract_code_update"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "change",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "change": {
    ///            "type": "object",
    ///            "required": [
    ///              "account_id"
    ///            ],
    ///            "properties": {
    ///              "account_id": {
    ///                "$ref": "#/components/schemas/AccountId"
    ///              }
    ///            }
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "contract_code_deletion"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    }
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
    #[serde(deny_unknown_fields)]
    pub enum StateChangeWithCauseViewVariant0 {}
    impl ::std::convert::From<&Self> for StateChangeWithCauseViewVariant0 {
        fn from(value: &StateChangeWithCauseViewVariant0) -> Self {
            value.clone()
        }
    }

    ///StateChangeWithCauseViewVariant1
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "cause"
    ///      ],
    ///      "properties": {
    ///        "cause": {
    ///          "$ref": "#/components/schemas/StateChangeCauseView"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "change",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "change": {
    ///          "type": "object",
    ///          "required": [
    ///            "account_id"
    ///          ],
    ///          "properties": {
    ///            "account_id": {
    ///              "$ref": "#/components/schemas/AccountId"
    ///            }
    ///          }
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "account_deletion"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "change",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "change": {
    ///            "description": "A view of the account",
    ///            "type": "object",
    ///            "required": [
    ///              "account_id",
    ///              "amount",
    ///              "code_hash",
    ///              "locked",
    ///              "storage_usage"
    ///            ],
    ///            "properties": {
    ///              "account_id": {
    ///                "$ref": "#/components/schemas/AccountId"
    ///              },
    ///              "amount": {
    ///                "type": "string"
    ///              },
    ///              "code_hash": {
    ///                "$ref": "#/components/schemas/CryptoHash"
    ///              },
    ///              "global_contract_account_id": {
    ///                "oneOf": [
    ///                  {
    ///                    "type": "null"
    ///                  },
    ///                  {
    ///                    "allOf": [
    ///                      {
    ///                        "$ref": "#/components/schemas/AccountId"
    ///                      }
    ///                    ]
    ///                  }
    ///                ]
    ///              },
    ///              "global_contract_hash": {
    ///                "oneOf": [
    ///                  {
    ///                    "type": "null"
    ///                  },
    ///                  {
    ///                    "allOf": [
    ///                      {
    ///                        "$ref": "#/components/schemas/CryptoHash"
    ///                      }
    ///                    ]
    ///                  }
    ///                ]
    ///              },
    ///              "locked": {
    ///                "type": "string"
    ///              },
    ///              "storage_paid_at": {
    ///                "description": "TODO(2271): deprecated.",
    ///                "default": 0,
    ///                "type": "integer",
    ///                "format": "uint64",
    ///                "minimum": 0.0
    ///              },
    ///              "storage_usage": {
    ///                "type": "integer",
    ///                "format": "uint64",
    ///                "minimum": 0.0
    ///              }
    ///            }
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "account_update"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "change",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "change": {
    ///            "type": "object",
    ///            "required": [
    ///              "access_key",
    ///              "account_id",
    ///              "public_key"
    ///            ],
    ///            "properties": {
    ///              "access_key": {
    ///                "$ref": "#/components/schemas/AccessKeyView"
    ///              },
    ///              "account_id": {
    ///                "$ref": "#/components/schemas/AccountId"
    ///              },
    ///              "public_key": {
    ///                "$ref": "#/components/schemas/PublicKey"
    ///              }
    ///            }
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "access_key_update"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "change",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "change": {
    ///            "type": "object",
    ///            "required": [
    ///              "account_id",
    ///              "public_key"
    ///            ],
    ///            "properties": {
    ///              "account_id": {
    ///                "$ref": "#/components/schemas/AccountId"
    ///              },
    ///              "public_key": {
    ///                "$ref": "#/components/schemas/PublicKey"
    ///              }
    ///            }
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "access_key_deletion"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "change",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "change": {
    ///            "type": "object",
    ///            "required": [
    ///              "account_id",
    ///              "key_base64",
    ///              "value_base64"
    ///            ],
    ///            "properties": {
    ///              "account_id": {
    ///                "$ref": "#/components/schemas/AccountId"
    ///              },
    ///              "key_base64": {
    ///                "type": "string"
    ///              },
    ///              "value_base64": {
    ///                "type": "string"
    ///              }
    ///            }
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "data_update"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "change",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "change": {
    ///            "type": "object",
    ///            "required": [
    ///              "account_id",
    ///              "key_base64"
    ///            ],
    ///            "properties": {
    ///              "account_id": {
    ///                "$ref": "#/components/schemas/AccountId"
    ///              },
    ///              "key_base64": {
    ///                "type": "string"
    ///              }
    ///            }
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "data_deletion"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "change",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "change": {
    ///            "type": "object",
    ///            "required": [
    ///              "account_id",
    ///              "code_base64"
    ///            ],
    ///            "properties": {
    ///              "account_id": {
    ///                "$ref": "#/components/schemas/AccountId"
    ///              },
    ///              "code_base64": {
    ///                "type": "string"
    ///              }
    ///            }
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "contract_code_update"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "change",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "change": {
    ///            "type": "object",
    ///            "required": [
    ///              "account_id"
    ///            ],
    ///            "properties": {
    ///              "account_id": {
    ///                "$ref": "#/components/schemas/AccountId"
    ///              }
    ///            }
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "contract_code_deletion"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    }
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
    #[serde(deny_unknown_fields)]
    pub enum StateChangeWithCauseViewVariant1 {}
    impl ::std::convert::From<&Self> for StateChangeWithCauseViewVariant1 {
        fn from(value: &StateChangeWithCauseViewVariant1) -> Self {
            value.clone()
        }
    }

    ///StateChangeWithCauseViewVariant2
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "cause"
    ///      ],
    ///      "properties": {
    ///        "cause": {
    ///          "$ref": "#/components/schemas/StateChangeCauseView"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "change",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "change": {
    ///          "type": "object",
    ///          "required": [
    ///            "access_key",
    ///            "account_id",
    ///            "public_key"
    ///          ],
    ///          "properties": {
    ///            "access_key": {
    ///              "$ref": "#/components/schemas/AccessKeyView"
    ///            },
    ///            "account_id": {
    ///              "$ref": "#/components/schemas/AccountId"
    ///            },
    ///            "public_key": {
    ///              "$ref": "#/components/schemas/PublicKey"
    ///            }
    ///          }
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "access_key_update"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "change",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "change": {
    ///            "description": "A view of the account",
    ///            "type": "object",
    ///            "required": [
    ///              "account_id",
    ///              "amount",
    ///              "code_hash",
    ///              "locked",
    ///              "storage_usage"
    ///            ],
    ///            "properties": {
    ///              "account_id": {
    ///                "$ref": "#/components/schemas/AccountId"
    ///              },
    ///              "amount": {
    ///                "type": "string"
    ///              },
    ///              "code_hash": {
    ///                "$ref": "#/components/schemas/CryptoHash"
    ///              },
    ///              "global_contract_account_id": {
    ///                "oneOf": [
    ///                  {
    ///                    "type": "null"
    ///                  },
    ///                  {
    ///                    "allOf": [
    ///                      {
    ///                        "$ref": "#/components/schemas/AccountId"
    ///                      }
    ///                    ]
    ///                  }
    ///                ]
    ///              },
    ///              "global_contract_hash": {
    ///                "oneOf": [
    ///                  {
    ///                    "type": "null"
    ///                  },
    ///                  {
    ///                    "allOf": [
    ///                      {
    ///                        "$ref": "#/components/schemas/CryptoHash"
    ///                      }
    ///                    ]
    ///                  }
    ///                ]
    ///              },
    ///              "locked": {
    ///                "type": "string"
    ///              },
    ///              "storage_paid_at": {
    ///                "description": "TODO(2271): deprecated.",
    ///                "default": 0,
    ///                "type": "integer",
    ///                "format": "uint64",
    ///                "minimum": 0.0
    ///              },
    ///              "storage_usage": {
    ///                "type": "integer",
    ///                "format": "uint64",
    ///                "minimum": 0.0
    ///              }
    ///            }
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "account_update"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "change",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "change": {
    ///            "type": "object",
    ///            "required": [
    ///              "account_id"
    ///            ],
    ///            "properties": {
    ///              "account_id": {
    ///                "$ref": "#/components/schemas/AccountId"
    ///              }
    ///            }
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "account_deletion"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "change",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "change": {
    ///            "type": "object",
    ///            "required": [
    ///              "account_id",
    ///              "public_key"
    ///            ],
    ///            "properties": {
    ///              "account_id": {
    ///                "$ref": "#/components/schemas/AccountId"
    ///              },
    ///              "public_key": {
    ///                "$ref": "#/components/schemas/PublicKey"
    ///              }
    ///            }
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "access_key_deletion"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "change",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "change": {
    ///            "type": "object",
    ///            "required": [
    ///              "account_id",
    ///              "key_base64",
    ///              "value_base64"
    ///            ],
    ///            "properties": {
    ///              "account_id": {
    ///                "$ref": "#/components/schemas/AccountId"
    ///              },
    ///              "key_base64": {
    ///                "type": "string"
    ///              },
    ///              "value_base64": {
    ///                "type": "string"
    ///              }
    ///            }
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "data_update"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "change",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "change": {
    ///            "type": "object",
    ///            "required": [
    ///              "account_id",
    ///              "key_base64"
    ///            ],
    ///            "properties": {
    ///              "account_id": {
    ///                "$ref": "#/components/schemas/AccountId"
    ///              },
    ///              "key_base64": {
    ///                "type": "string"
    ///              }
    ///            }
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "data_deletion"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "change",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "change": {
    ///            "type": "object",
    ///            "required": [
    ///              "account_id",
    ///              "code_base64"
    ///            ],
    ///            "properties": {
    ///              "account_id": {
    ///                "$ref": "#/components/schemas/AccountId"
    ///              },
    ///              "code_base64": {
    ///                "type": "string"
    ///              }
    ///            }
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "contract_code_update"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "change",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "change": {
    ///            "type": "object",
    ///            "required": [
    ///              "account_id"
    ///            ],
    ///            "properties": {
    ///              "account_id": {
    ///                "$ref": "#/components/schemas/AccountId"
    ///              }
    ///            }
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "contract_code_deletion"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    }
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
    #[serde(deny_unknown_fields)]
    pub enum StateChangeWithCauseViewVariant2 {}
    impl ::std::convert::From<&Self> for StateChangeWithCauseViewVariant2 {
        fn from(value: &StateChangeWithCauseViewVariant2) -> Self {
            value.clone()
        }
    }

    ///StateChangeWithCauseViewVariant3
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "cause"
    ///      ],
    ///      "properties": {
    ///        "cause": {
    ///          "$ref": "#/components/schemas/StateChangeCauseView"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "change",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "change": {
    ///          "type": "object",
    ///          "required": [
    ///            "account_id",
    ///            "public_key"
    ///          ],
    ///          "properties": {
    ///            "account_id": {
    ///              "$ref": "#/components/schemas/AccountId"
    ///            },
    ///            "public_key": {
    ///              "$ref": "#/components/schemas/PublicKey"
    ///            }
    ///          }
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "access_key_deletion"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "change",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "change": {
    ///            "description": "A view of the account",
    ///            "type": "object",
    ///            "required": [
    ///              "account_id",
    ///              "amount",
    ///              "code_hash",
    ///              "locked",
    ///              "storage_usage"
    ///            ],
    ///            "properties": {
    ///              "account_id": {
    ///                "$ref": "#/components/schemas/AccountId"
    ///              },
    ///              "amount": {
    ///                "type": "string"
    ///              },
    ///              "code_hash": {
    ///                "$ref": "#/components/schemas/CryptoHash"
    ///              },
    ///              "global_contract_account_id": {
    ///                "oneOf": [
    ///                  {
    ///                    "type": "null"
    ///                  },
    ///                  {
    ///                    "allOf": [
    ///                      {
    ///                        "$ref": "#/components/schemas/AccountId"
    ///                      }
    ///                    ]
    ///                  }
    ///                ]
    ///              },
    ///              "global_contract_hash": {
    ///                "oneOf": [
    ///                  {
    ///                    "type": "null"
    ///                  },
    ///                  {
    ///                    "allOf": [
    ///                      {
    ///                        "$ref": "#/components/schemas/CryptoHash"
    ///                      }
    ///                    ]
    ///                  }
    ///                ]
    ///              },
    ///              "locked": {
    ///                "type": "string"
    ///              },
    ///              "storage_paid_at": {
    ///                "description": "TODO(2271): deprecated.",
    ///                "default": 0,
    ///                "type": "integer",
    ///                "format": "uint64",
    ///                "minimum": 0.0
    ///              },
    ///              "storage_usage": {
    ///                "type": "integer",
    ///                "format": "uint64",
    ///                "minimum": 0.0
    ///              }
    ///            }
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "account_update"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "change",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "change": {
    ///            "type": "object",
    ///            "required": [
    ///              "account_id"
    ///            ],
    ///            "properties": {
    ///              "account_id": {
    ///                "$ref": "#/components/schemas/AccountId"
    ///              }
    ///            }
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "account_deletion"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "change",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "change": {
    ///            "type": "object",
    ///            "required": [
    ///              "access_key",
    ///              "account_id",
    ///              "public_key"
    ///            ],
    ///            "properties": {
    ///              "access_key": {
    ///                "$ref": "#/components/schemas/AccessKeyView"
    ///              },
    ///              "account_id": {
    ///                "$ref": "#/components/schemas/AccountId"
    ///              },
    ///              "public_key": {
    ///                "$ref": "#/components/schemas/PublicKey"
    ///              }
    ///            }
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "access_key_update"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "change",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "change": {
    ///            "type": "object",
    ///            "required": [
    ///              "account_id",
    ///              "key_base64",
    ///              "value_base64"
    ///            ],
    ///            "properties": {
    ///              "account_id": {
    ///                "$ref": "#/components/schemas/AccountId"
    ///              },
    ///              "key_base64": {
    ///                "type": "string"
    ///              },
    ///              "value_base64": {
    ///                "type": "string"
    ///              }
    ///            }
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "data_update"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "change",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "change": {
    ///            "type": "object",
    ///            "required": [
    ///              "account_id",
    ///              "key_base64"
    ///            ],
    ///            "properties": {
    ///              "account_id": {
    ///                "$ref": "#/components/schemas/AccountId"
    ///              },
    ///              "key_base64": {
    ///                "type": "string"
    ///              }
    ///            }
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "data_deletion"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "change",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "change": {
    ///            "type": "object",
    ///            "required": [
    ///              "account_id",
    ///              "code_base64"
    ///            ],
    ///            "properties": {
    ///              "account_id": {
    ///                "$ref": "#/components/schemas/AccountId"
    ///              },
    ///              "code_base64": {
    ///                "type": "string"
    ///              }
    ///            }
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "contract_code_update"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "change",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "change": {
    ///            "type": "object",
    ///            "required": [
    ///              "account_id"
    ///            ],
    ///            "properties": {
    ///              "account_id": {
    ///                "$ref": "#/components/schemas/AccountId"
    ///              }
    ///            }
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "contract_code_deletion"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    }
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
    #[serde(deny_unknown_fields)]
    pub enum StateChangeWithCauseViewVariant3 {}
    impl ::std::convert::From<&Self> for StateChangeWithCauseViewVariant3 {
        fn from(value: &StateChangeWithCauseViewVariant3) -> Self {
            value.clone()
        }
    }

    ///StateChangeWithCauseViewVariant4
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "cause"
    ///      ],
    ///      "properties": {
    ///        "cause": {
    ///          "$ref": "#/components/schemas/StateChangeCauseView"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "change",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "change": {
    ///          "type": "object",
    ///          "required": [
    ///            "account_id",
    ///            "key_base64",
    ///            "value_base64"
    ///          ],
    ///          "properties": {
    ///            "account_id": {
    ///              "$ref": "#/components/schemas/AccountId"
    ///            },
    ///            "key_base64": {
    ///              "type": "string"
    ///            },
    ///            "value_base64": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "data_update"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "change",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "change": {
    ///            "description": "A view of the account",
    ///            "type": "object",
    ///            "required": [
    ///              "account_id",
    ///              "amount",
    ///              "code_hash",
    ///              "locked",
    ///              "storage_usage"
    ///            ],
    ///            "properties": {
    ///              "account_id": {
    ///                "$ref": "#/components/schemas/AccountId"
    ///              },
    ///              "amount": {
    ///                "type": "string"
    ///              },
    ///              "code_hash": {
    ///                "$ref": "#/components/schemas/CryptoHash"
    ///              },
    ///              "global_contract_account_id": {
    ///                "oneOf": [
    ///                  {
    ///                    "type": "null"
    ///                  },
    ///                  {
    ///                    "allOf": [
    ///                      {
    ///                        "$ref": "#/components/schemas/AccountId"
    ///                      }
    ///                    ]
    ///                  }
    ///                ]
    ///              },
    ///              "global_contract_hash": {
    ///                "oneOf": [
    ///                  {
    ///                    "type": "null"
    ///                  },
    ///                  {
    ///                    "allOf": [
    ///                      {
    ///                        "$ref": "#/components/schemas/CryptoHash"
    ///                      }
    ///                    ]
    ///                  }
    ///                ]
    ///              },
    ///              "locked": {
    ///                "type": "string"
    ///              },
    ///              "storage_paid_at": {
    ///                "description": "TODO(2271): deprecated.",
    ///                "default": 0,
    ///                "type": "integer",
    ///                "format": "uint64",
    ///                "minimum": 0.0
    ///              },
    ///              "storage_usage": {
    ///                "type": "integer",
    ///                "format": "uint64",
    ///                "minimum": 0.0
    ///              }
    ///            }
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "account_update"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "change",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "change": {
    ///            "type": "object",
    ///            "required": [
    ///              "account_id"
    ///            ],
    ///            "properties": {
    ///              "account_id": {
    ///                "$ref": "#/components/schemas/AccountId"
    ///              }
    ///            }
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "account_deletion"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "change",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "change": {
    ///            "type": "object",
    ///            "required": [
    ///              "access_key",
    ///              "account_id",
    ///              "public_key"
    ///            ],
    ///            "properties": {
    ///              "access_key": {
    ///                "$ref": "#/components/schemas/AccessKeyView"
    ///              },
    ///              "account_id": {
    ///                "$ref": "#/components/schemas/AccountId"
    ///              },
    ///              "public_key": {
    ///                "$ref": "#/components/schemas/PublicKey"
    ///              }
    ///            }
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "access_key_update"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "change",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "change": {
    ///            "type": "object",
    ///            "required": [
    ///              "account_id",
    ///              "public_key"
    ///            ],
    ///            "properties": {
    ///              "account_id": {
    ///                "$ref": "#/components/schemas/AccountId"
    ///              },
    ///              "public_key": {
    ///                "$ref": "#/components/schemas/PublicKey"
    ///              }
    ///            }
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "access_key_deletion"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "change",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "change": {
    ///            "type": "object",
    ///            "required": [
    ///              "account_id",
    ///              "key_base64"
    ///            ],
    ///            "properties": {
    ///              "account_id": {
    ///                "$ref": "#/components/schemas/AccountId"
    ///              },
    ///              "key_base64": {
    ///                "type": "string"
    ///              }
    ///            }
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "data_deletion"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "change",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "change": {
    ///            "type": "object",
    ///            "required": [
    ///              "account_id",
    ///              "code_base64"
    ///            ],
    ///            "properties": {
    ///              "account_id": {
    ///                "$ref": "#/components/schemas/AccountId"
    ///              },
    ///              "code_base64": {
    ///                "type": "string"
    ///              }
    ///            }
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "contract_code_update"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "change",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "change": {
    ///            "type": "object",
    ///            "required": [
    ///              "account_id"
    ///            ],
    ///            "properties": {
    ///              "account_id": {
    ///                "$ref": "#/components/schemas/AccountId"
    ///              }
    ///            }
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "contract_code_deletion"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    }
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
    #[serde(deny_unknown_fields)]
    pub enum StateChangeWithCauseViewVariant4 {}
    impl ::std::convert::From<&Self> for StateChangeWithCauseViewVariant4 {
        fn from(value: &StateChangeWithCauseViewVariant4) -> Self {
            value.clone()
        }
    }

    ///StateChangeWithCauseViewVariant5
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "cause"
    ///      ],
    ///      "properties": {
    ///        "cause": {
    ///          "$ref": "#/components/schemas/StateChangeCauseView"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "change",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "change": {
    ///          "type": "object",
    ///          "required": [
    ///            "account_id",
    ///            "key_base64"
    ///          ],
    ///          "properties": {
    ///            "account_id": {
    ///              "$ref": "#/components/schemas/AccountId"
    ///            },
    ///            "key_base64": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "data_deletion"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "change",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "change": {
    ///            "description": "A view of the account",
    ///            "type": "object",
    ///            "required": [
    ///              "account_id",
    ///              "amount",
    ///              "code_hash",
    ///              "locked",
    ///              "storage_usage"
    ///            ],
    ///            "properties": {
    ///              "account_id": {
    ///                "$ref": "#/components/schemas/AccountId"
    ///              },
    ///              "amount": {
    ///                "type": "string"
    ///              },
    ///              "code_hash": {
    ///                "$ref": "#/components/schemas/CryptoHash"
    ///              },
    ///              "global_contract_account_id": {
    ///                "oneOf": [
    ///                  {
    ///                    "type": "null"
    ///                  },
    ///                  {
    ///                    "allOf": [
    ///                      {
    ///                        "$ref": "#/components/schemas/AccountId"
    ///                      }
    ///                    ]
    ///                  }
    ///                ]
    ///              },
    ///              "global_contract_hash": {
    ///                "oneOf": [
    ///                  {
    ///                    "type": "null"
    ///                  },
    ///                  {
    ///                    "allOf": [
    ///                      {
    ///                        "$ref": "#/components/schemas/CryptoHash"
    ///                      }
    ///                    ]
    ///                  }
    ///                ]
    ///              },
    ///              "locked": {
    ///                "type": "string"
    ///              },
    ///              "storage_paid_at": {
    ///                "description": "TODO(2271): deprecated.",
    ///                "default": 0,
    ///                "type": "integer",
    ///                "format": "uint64",
    ///                "minimum": 0.0
    ///              },
    ///              "storage_usage": {
    ///                "type": "integer",
    ///                "format": "uint64",
    ///                "minimum": 0.0
    ///              }
    ///            }
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "account_update"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "change",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "change": {
    ///            "type": "object",
    ///            "required": [
    ///              "account_id"
    ///            ],
    ///            "properties": {
    ///              "account_id": {
    ///                "$ref": "#/components/schemas/AccountId"
    ///              }
    ///            }
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "account_deletion"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "change",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "change": {
    ///            "type": "object",
    ///            "required": [
    ///              "access_key",
    ///              "account_id",
    ///              "public_key"
    ///            ],
    ///            "properties": {
    ///              "access_key": {
    ///                "$ref": "#/components/schemas/AccessKeyView"
    ///              },
    ///              "account_id": {
    ///                "$ref": "#/components/schemas/AccountId"
    ///              },
    ///              "public_key": {
    ///                "$ref": "#/components/schemas/PublicKey"
    ///              }
    ///            }
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "access_key_update"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "change",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "change": {
    ///            "type": "object",
    ///            "required": [
    ///              "account_id",
    ///              "public_key"
    ///            ],
    ///            "properties": {
    ///              "account_id": {
    ///                "$ref": "#/components/schemas/AccountId"
    ///              },
    ///              "public_key": {
    ///                "$ref": "#/components/schemas/PublicKey"
    ///              }
    ///            }
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "access_key_deletion"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "change",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "change": {
    ///            "type": "object",
    ///            "required": [
    ///              "account_id",
    ///              "key_base64",
    ///              "value_base64"
    ///            ],
    ///            "properties": {
    ///              "account_id": {
    ///                "$ref": "#/components/schemas/AccountId"
    ///              },
    ///              "key_base64": {
    ///                "type": "string"
    ///              },
    ///              "value_base64": {
    ///                "type": "string"
    ///              }
    ///            }
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "data_update"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "change",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "change": {
    ///            "type": "object",
    ///            "required": [
    ///              "account_id",
    ///              "code_base64"
    ///            ],
    ///            "properties": {
    ///              "account_id": {
    ///                "$ref": "#/components/schemas/AccountId"
    ///              },
    ///              "code_base64": {
    ///                "type": "string"
    ///              }
    ///            }
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "contract_code_update"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "change",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "change": {
    ///            "type": "object",
    ///            "required": [
    ///              "account_id"
    ///            ],
    ///            "properties": {
    ///              "account_id": {
    ///                "$ref": "#/components/schemas/AccountId"
    ///              }
    ///            }
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "contract_code_deletion"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    }
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
    #[serde(deny_unknown_fields)]
    pub enum StateChangeWithCauseViewVariant5 {}
    impl ::std::convert::From<&Self> for StateChangeWithCauseViewVariant5 {
        fn from(value: &StateChangeWithCauseViewVariant5) -> Self {
            value.clone()
        }
    }

    ///StateChangeWithCauseViewVariant6
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "cause"
    ///      ],
    ///      "properties": {
    ///        "cause": {
    ///          "$ref": "#/components/schemas/StateChangeCauseView"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "change",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "change": {
    ///          "type": "object",
    ///          "required": [
    ///            "account_id",
    ///            "code_base64"
    ///          ],
    ///          "properties": {
    ///            "account_id": {
    ///              "$ref": "#/components/schemas/AccountId"
    ///            },
    ///            "code_base64": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "contract_code_update"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "change",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "change": {
    ///            "description": "A view of the account",
    ///            "type": "object",
    ///            "required": [
    ///              "account_id",
    ///              "amount",
    ///              "code_hash",
    ///              "locked",
    ///              "storage_usage"
    ///            ],
    ///            "properties": {
    ///              "account_id": {
    ///                "$ref": "#/components/schemas/AccountId"
    ///              },
    ///              "amount": {
    ///                "type": "string"
    ///              },
    ///              "code_hash": {
    ///                "$ref": "#/components/schemas/CryptoHash"
    ///              },
    ///              "global_contract_account_id": {
    ///                "oneOf": [
    ///                  {
    ///                    "type": "null"
    ///                  },
    ///                  {
    ///                    "allOf": [
    ///                      {
    ///                        "$ref": "#/components/schemas/AccountId"
    ///                      }
    ///                    ]
    ///                  }
    ///                ]
    ///              },
    ///              "global_contract_hash": {
    ///                "oneOf": [
    ///                  {
    ///                    "type": "null"
    ///                  },
    ///                  {
    ///                    "allOf": [
    ///                      {
    ///                        "$ref": "#/components/schemas/CryptoHash"
    ///                      }
    ///                    ]
    ///                  }
    ///                ]
    ///              },
    ///              "locked": {
    ///                "type": "string"
    ///              },
    ///              "storage_paid_at": {
    ///                "description": "TODO(2271): deprecated.",
    ///                "default": 0,
    ///                "type": "integer",
    ///                "format": "uint64",
    ///                "minimum": 0.0
    ///              },
    ///              "storage_usage": {
    ///                "type": "integer",
    ///                "format": "uint64",
    ///                "minimum": 0.0
    ///              }
    ///            }
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "account_update"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "change",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "change": {
    ///            "type": "object",
    ///            "required": [
    ///              "account_id"
    ///            ],
    ///            "properties": {
    ///              "account_id": {
    ///                "$ref": "#/components/schemas/AccountId"
    ///              }
    ///            }
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "account_deletion"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "change",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "change": {
    ///            "type": "object",
    ///            "required": [
    ///              "access_key",
    ///              "account_id",
    ///              "public_key"
    ///            ],
    ///            "properties": {
    ///              "access_key": {
    ///                "$ref": "#/components/schemas/AccessKeyView"
    ///              },
    ///              "account_id": {
    ///                "$ref": "#/components/schemas/AccountId"
    ///              },
    ///              "public_key": {
    ///                "$ref": "#/components/schemas/PublicKey"
    ///              }
    ///            }
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "access_key_update"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "change",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "change": {
    ///            "type": "object",
    ///            "required": [
    ///              "account_id",
    ///              "public_key"
    ///            ],
    ///            "properties": {
    ///              "account_id": {
    ///                "$ref": "#/components/schemas/AccountId"
    ///              },
    ///              "public_key": {
    ///                "$ref": "#/components/schemas/PublicKey"
    ///              }
    ///            }
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "access_key_deletion"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "change",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "change": {
    ///            "type": "object",
    ///            "required": [
    ///              "account_id",
    ///              "key_base64",
    ///              "value_base64"
    ///            ],
    ///            "properties": {
    ///              "account_id": {
    ///                "$ref": "#/components/schemas/AccountId"
    ///              },
    ///              "key_base64": {
    ///                "type": "string"
    ///              },
    ///              "value_base64": {
    ///                "type": "string"
    ///              }
    ///            }
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "data_update"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "change",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "change": {
    ///            "type": "object",
    ///            "required": [
    ///              "account_id",
    ///              "key_base64"
    ///            ],
    ///            "properties": {
    ///              "account_id": {
    ///                "$ref": "#/components/schemas/AccountId"
    ///              },
    ///              "key_base64": {
    ///                "type": "string"
    ///              }
    ///            }
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "data_deletion"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "change",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "change": {
    ///            "type": "object",
    ///            "required": [
    ///              "account_id"
    ///            ],
    ///            "properties": {
    ///              "account_id": {
    ///                "$ref": "#/components/schemas/AccountId"
    ///              }
    ///            }
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "contract_code_deletion"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    }
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
    #[serde(deny_unknown_fields)]
    pub enum StateChangeWithCauseViewVariant6 {}
    impl ::std::convert::From<&Self> for StateChangeWithCauseViewVariant6 {
        fn from(value: &StateChangeWithCauseViewVariant6) -> Self {
            value.clone()
        }
    }

    ///StateChangeWithCauseViewVariant7
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "cause"
    ///      ],
    ///      "properties": {
    ///        "cause": {
    ///          "$ref": "#/components/schemas/StateChangeCauseView"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "change",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "change": {
    ///          "type": "object",
    ///          "required": [
    ///            "account_id"
    ///          ],
    ///          "properties": {
    ///            "account_id": {
    ///              "$ref": "#/components/schemas/AccountId"
    ///            }
    ///          }
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "contract_code_deletion"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "change",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "change": {
    ///            "description": "A view of the account",
    ///            "type": "object",
    ///            "required": [
    ///              "account_id",
    ///              "amount",
    ///              "code_hash",
    ///              "locked",
    ///              "storage_usage"
    ///            ],
    ///            "properties": {
    ///              "account_id": {
    ///                "$ref": "#/components/schemas/AccountId"
    ///              },
    ///              "amount": {
    ///                "type": "string"
    ///              },
    ///              "code_hash": {
    ///                "$ref": "#/components/schemas/CryptoHash"
    ///              },
    ///              "global_contract_account_id": {
    ///                "oneOf": [
    ///                  {
    ///                    "type": "null"
    ///                  },
    ///                  {
    ///                    "allOf": [
    ///                      {
    ///                        "$ref": "#/components/schemas/AccountId"
    ///                      }
    ///                    ]
    ///                  }
    ///                ]
    ///              },
    ///              "global_contract_hash": {
    ///                "oneOf": [
    ///                  {
    ///                    "type": "null"
    ///                  },
    ///                  {
    ///                    "allOf": [
    ///                      {
    ///                        "$ref": "#/components/schemas/CryptoHash"
    ///                      }
    ///                    ]
    ///                  }
    ///                ]
    ///              },
    ///              "locked": {
    ///                "type": "string"
    ///              },
    ///              "storage_paid_at": {
    ///                "description": "TODO(2271): deprecated.",
    ///                "default": 0,
    ///                "type": "integer",
    ///                "format": "uint64",
    ///                "minimum": 0.0
    ///              },
    ///              "storage_usage": {
    ///                "type": "integer",
    ///                "format": "uint64",
    ///                "minimum": 0.0
    ///              }
    ///            }
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "account_update"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "change",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "change": {
    ///            "type": "object",
    ///            "required": [
    ///              "account_id"
    ///            ],
    ///            "properties": {
    ///              "account_id": {
    ///                "$ref": "#/components/schemas/AccountId"
    ///              }
    ///            }
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "account_deletion"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "change",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "change": {
    ///            "type": "object",
    ///            "required": [
    ///              "access_key",
    ///              "account_id",
    ///              "public_key"
    ///            ],
    ///            "properties": {
    ///              "access_key": {
    ///                "$ref": "#/components/schemas/AccessKeyView"
    ///              },
    ///              "account_id": {
    ///                "$ref": "#/components/schemas/AccountId"
    ///              },
    ///              "public_key": {
    ///                "$ref": "#/components/schemas/PublicKey"
    ///              }
    ///            }
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "access_key_update"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "change",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "change": {
    ///            "type": "object",
    ///            "required": [
    ///              "account_id",
    ///              "public_key"
    ///            ],
    ///            "properties": {
    ///              "account_id": {
    ///                "$ref": "#/components/schemas/AccountId"
    ///              },
    ///              "public_key": {
    ///                "$ref": "#/components/schemas/PublicKey"
    ///              }
    ///            }
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "access_key_deletion"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "change",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "change": {
    ///            "type": "object",
    ///            "required": [
    ///              "account_id",
    ///              "key_base64",
    ///              "value_base64"
    ///            ],
    ///            "properties": {
    ///              "account_id": {
    ///                "$ref": "#/components/schemas/AccountId"
    ///              },
    ///              "key_base64": {
    ///                "type": "string"
    ///              },
    ///              "value_base64": {
    ///                "type": "string"
    ///              }
    ///            }
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "data_update"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "change",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "change": {
    ///            "type": "object",
    ///            "required": [
    ///              "account_id",
    ///              "key_base64"
    ///            ],
    ///            "properties": {
    ///              "account_id": {
    ///                "$ref": "#/components/schemas/AccountId"
    ///              },
    ///              "key_base64": {
    ///                "type": "string"
    ///              }
    ///            }
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "data_deletion"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "not": {
    ///        "type": "object",
    ///        "required": [
    ///          "change",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "change": {
    ///            "type": "object",
    ///            "required": [
    ///              "account_id",
    ///              "code_base64"
    ///            ],
    ///            "properties": {
    ///              "account_id": {
    ///                "$ref": "#/components/schemas/AccountId"
    ///              },
    ///              "code_base64": {
    ///                "type": "string"
    ///              }
    ///            }
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "contract_code_update"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    }
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
    #[serde(deny_unknown_fields)]
    pub enum StateChangeWithCauseViewVariant7 {}
    impl ::std::convert::From<&Self> for StateChangeWithCauseViewVariant7 {
        fn from(value: &StateChangeWithCauseViewVariant7) -> Self {
            value.clone()
        }
    }

    ///Options for dumping state to S3.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Options for dumping state to S3.",
    ///  "type": "object",
    ///  "properties": {
    ///    "dump": {
    ///      "description": "`none` value disables state dump to external
    /// storage.",
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/DumpConfig"
    ///            }
    ///          ]
    ///        }
    ///      ]
    ///    },
    ///    "sync": {
    ///      "$ref": "#/components/schemas/SyncConfig"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct StateSyncConfig {
        ///`none` value disables state dump to external storage.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub dump: ::std::option::Option<DumpConfig>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sync: ::std::option::Option<SyncConfig>,
    }

    impl ::std::convert::From<&StateSyncConfig> for StateSyncConfig {
        fn from(value: &StateSyncConfig) -> Self {
            value.clone()
        }
    }

    impl ::std::default::Default for StateSyncConfig {
        fn default() -> Self {
            Self {
                dump: Default::default(),
                sync: Default::default(),
            }
        }
    }

    ///StatusMethodNameHelperEnum
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "status"
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
    pub enum StatusMethodNameHelperEnum {
        #[serde(rename = "status")]
        Status,
    }

    impl ::std::convert::From<&Self> for StatusMethodNameHelperEnum {
        fn from(value: &StatusMethodNameHelperEnum) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for StatusMethodNameHelperEnum {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Status => write!(f, "status"),
            }
        }
    }

    impl ::std::str::FromStr for StatusMethodNameHelperEnum {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "status" => Ok(Self::Status),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for StatusMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for StatusMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for StatusMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///StatusSyncInfo
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "earliest_block_time",
    ///    "latest_block_hash",
    ///    "latest_block_height",
    ///    "latest_block_time",
    ///    "latest_state_root",
    ///    "syncing"
    ///  ],
    ///  "properties": {
    ///    "earliest_block_hash": {
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/CryptoHash"
    ///            }
    ///          ]
    ///        }
    ///      ]
    ///    },
    ///    "earliest_block_height": {
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "earliest_block_time": {
    ///      "type": "string"
    ///    },
    ///    "epoch_id": {
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/EpochId"
    ///            }
    ///          ]
    ///        }
    ///      ]
    ///    },
    ///    "epoch_start_height": {
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "latest_block_hash": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "latest_block_height": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "latest_block_time": {
    ///      "type": "string"
    ///    },
    ///    "latest_state_root": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "syncing": {
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct StatusSyncInfo {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub earliest_block_hash: ::std::option::Option<CryptoHash>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub earliest_block_height: ::std::option::Option<u64>,
        pub earliest_block_time: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub epoch_id: ::std::option::Option<EpochId>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub epoch_start_height: ::std::option::Option<u64>,
        pub latest_block_hash: CryptoHash,
        pub latest_block_height: u64,
        pub latest_block_time: ::std::string::String,
        pub latest_state_root: CryptoHash,
        pub syncing: bool,
    }

    impl ::std::convert::From<&StatusSyncInfo> for StatusSyncInfo {
        fn from(value: &StatusSyncInfo) -> Self {
            value.clone()
        }
    }

    ///Errors which may occur during working with trie storages, storing trie
    /// values (trie nodes and state values) by their hashes.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Errors which may occur during working with trie
    /// storages, storing trie values (trie nodes and state values) by their
    /// hashes.",
    ///  "oneOf": [
    ///    {
    ///      "description": "Key-value db internal failure",
    ///      "type": "string",
    ///      "enum": [
    ///        "StorageInternalError"
    ///      ]
    ///    },
    ///    {
    ///      "description": "Requested trie value by its hash which is missing
    /// in storage.",
    ///      "type": "object",
    ///      "required": [
    ///        "MissingTrieValue"
    ///      ],
    ///      "properties": {
    ///        "MissingTrieValue": {
    ///          "$ref": "#/components/schemas/MissingTrieValue"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "Found trie node which shouldn't be part of state.
    /// Raised during validation of state sync parts where incorrect node was
    /// passed. TODO (#8997): consider including hash of trie node.",
    ///      "type": "string",
    ///      "enum": [
    ///        "UnexpectedTrieValue"
    ///      ]
    ///    },
    ///    {
    ///      "description": "Either invalid state or key-value db is corrupted.
    /// For PartialStorage it cannot be corrupted. Error message is unreliable
    /// and for debugging purposes only. It's also probably ok to panic in every
    /// place that produces this error. We can check if db is corrupted by
    /// verifying everything in the state trie.",
    ///      "type": "object",
    ///      "required": [
    ///        "StorageInconsistentState"
    ///      ],
    ///      "properties": {
    ///        "StorageInconsistentState": {
    ///          "type": "string"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "Flat storage error, meaning that it doesn't support
    /// some block anymore. We guarantee that such block cannot become final,
    /// thus block processing must resume normally.",
    ///      "type": "object",
    ///      "required": [
    ///        "FlatStorageBlockNotSupported"
    ///      ],
    ///      "properties": {
    ///        "FlatStorageBlockNotSupported": {
    ///          "type": "string"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "In-memory trie could not be loaded for some
    /// reason.",
    ///      "type": "object",
    ///      "required": [
    ///        "MemTrieLoadingError"
    ///      ],
    ///      "properties": {
    ///        "MemTrieLoadingError": {
    ///          "type": "string"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "Indicates that a resharding operation on flat
    /// storage is already in progress, when it wasn't expected to be so.",
    ///      "type": "string",
    ///      "enum": [
    ///        "FlatStorageReshardingAlreadyInProgress"
    ///      ]
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub enum StorageError {
        ///Key-value db internal failure
        StorageInternalError,
        ///Requested trie value by its hash which is missing in storage.
        MissingTrieValue(MissingTrieValue),
        ///Found trie node which shouldn't be part of state. Raised during
        /// validation of state sync parts where incorrect node was passed. TODO
        /// (#8997): consider including hash of trie node.
        UnexpectedTrieValue,
        ///Either invalid state or key-value db is corrupted. For
        /// PartialStorage it cannot be corrupted. Error message is unreliable
        /// and for debugging purposes only. It's also probably ok to panic in
        /// every place that produces this error. We can check if db is
        /// corrupted by verifying everything in the state trie.
        StorageInconsistentState(::std::string::String),
        ///Flat storage error, meaning that it doesn't support some block
        /// anymore. We guarantee that such block cannot become final, thus
        /// block processing must resume normally.
        FlatStorageBlockNotSupported(::std::string::String),
        ///In-memory trie could not be loaded for some reason.
        MemTrieLoadingError(::std::string::String),
        ///Indicates that a resharding operation on flat storage is already in
        /// progress, when it wasn't expected to be so.
        FlatStorageReshardingAlreadyInProgress,
    }

    impl ::std::convert::From<&Self> for StorageError {
        fn from(value: &StorageError) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<MissingTrieValue> for StorageError {
        fn from(value: MissingTrieValue) -> Self {
            Self::MissingTrieValue(value)
        }
    }

    ///This enum represents if a storage_get call will be performed through
    /// flat storage or trie
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "This enum represents if a storage_get call will be
    /// performed through flat storage or trie",
    ///  "type": "string",
    ///  "enum": [
    ///    "FlatStorage",
    ///    "Trie"
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
    pub enum StorageGetMode {
        FlatStorage,
        Trie,
    }

    impl ::std::convert::From<&Self> for StorageGetMode {
        fn from(value: &StorageGetMode) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for StorageGetMode {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::FlatStorage => write!(f, "FlatStorage"),
                Self::Trie => write!(f, "Trie"),
            }
        }
    }

    impl ::std::str::FromStr for StorageGetMode {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "FlatStorage" => Ok(Self::FlatStorage),
                "Trie" => Ok(Self::Trie),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for StorageGetMode {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for StorageGetMode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for StorageGetMode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///Describes cost of storage per block
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Describes cost of storage per block",
    ///  "type": "object",
    ///  "required": [
    ///    "num_bytes_account",
    ///    "num_extra_bytes_record"
    ///  ],
    ///  "properties": {
    ///    "num_bytes_account": {
    ///      "description": "Number of bytes for an account record, including
    /// rounding up for account id.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "num_extra_bytes_record": {
    ///      "description": "Additional number of bytes for a k/v record",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct StorageUsageConfigView {
        ///Number of bytes for an account record, including rounding up for
        /// account id.
        pub num_bytes_account: u64,
        ///Additional number of bytes for a k/v record
        pub num_extra_bytes_record: u64,
    }

    impl ::std::convert::From<&StorageUsageConfigView> for StorageUsageConfigView {
        fn from(value: &StorageUsageConfigView) -> Self {
            value.clone()
        }
    }

    ///SyncCheckpoint
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "genesis",
    ///    "earliest_available"
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
    pub enum SyncCheckpoint {
        #[serde(rename = "genesis")]
        Genesis,
        #[serde(rename = "earliest_available")]
        EarliestAvailable,
    }

    impl ::std::convert::From<&Self> for SyncCheckpoint {
        fn from(value: &SyncCheckpoint) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for SyncCheckpoint {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Genesis => write!(f, "genesis"),
                Self::EarliestAvailable => write!(f, "earliest_available"),
            }
        }
    }

    impl ::std::str::FromStr for SyncCheckpoint {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "genesis" => Ok(Self::Genesis),
                "earliest_available" => Ok(Self::EarliestAvailable),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for SyncCheckpoint {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for SyncCheckpoint {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for SyncCheckpoint {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///Configures how to fetch state parts during state sync.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Configures how to fetch state parts during state
    /// sync.",
    ///  "oneOf": [
    ///    {
    ///      "description": "Syncs state from the peers without reading anything
    /// from external storage.",
    ///      "type": "string",
    ///      "enum": [
    ///        "Peers"
    ///      ]
    ///    },
    ///    {
    ///      "description": "Expects parts to be available in external
    /// storage.",
    ///      "type": "object",
    ///      "required": [
    ///        "ExternalStorage"
    ///      ],
    ///      "properties": {
    ///        "ExternalStorage": {
    ///          "$ref": "#/components/schemas/ExternalStorageConfig"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub enum SyncConfig {
        ///Syncs state from the peers without reading anything from external
        /// storage.
        Peers,
        ///Expects parts to be available in external storage.
        ExternalStorage(ExternalStorageConfig),
    }

    impl ::std::convert::From<&Self> for SyncConfig {
        fn from(value: &SyncConfig) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<ExternalStorageConfig> for SyncConfig {
        fn from(value: ExternalStorageConfig) -> Self {
            Self::ExternalStorage(value)
        }
    }

    ///Tier1ProxyView
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "addr",
    ///    "peer_id"
    ///  ],
    ///  "properties": {
    ///    "addr": {
    ///      "type": "string"
    ///    },
    ///    "peer_id": {
    ///      "$ref": "#/components/schemas/PublicKey"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Tier1ProxyView {
        pub addr: ::std::string::String,
        pub peer_id: PublicKey,
    }

    impl ::std::convert::From<&Tier1ProxyView> for Tier1ProxyView {
        fn from(value: &Tier1ProxyView) -> Self {
            value.clone()
        }
    }

    ///TransferAction
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "deposit"
    ///  ],
    ///  "properties": {
    ///    "deposit": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct TransferAction {
        pub deposit: ::std::string::String,
    }

    impl ::std::convert::From<&TransferAction> for TransferAction {
        fn from(value: &TransferAction) -> Self {
            value.clone()
        }
    }

    ///Error returned in the ExecutionOutcome in case of failure
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Error returned in the ExecutionOutcome in case of
    /// failure",
    ///  "oneOf": [
    ///    {
    ///      "description": "An error happened during Action execution",
    ///      "type": "object",
    ///      "required": [
    ///        "ActionError"
    ///      ],
    ///      "properties": {
    ///        "ActionError": {
    ///          "$ref": "#/components/schemas/ActionError"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "An error happened during Transaction execution",
    ///      "type": "object",
    ///      "required": [
    ///        "InvalidTxError"
    ///      ],
    ///      "properties": {
    ///        "InvalidTxError": {
    ///          "$ref": "#/components/schemas/InvalidTxError"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub enum TxExecutionError {
        ///An error happened during Action execution
        ActionError(ActionError),
        ///An error happened during Transaction execution
        InvalidTxError(InvalidTxError),
    }

    impl ::std::convert::From<&Self> for TxExecutionError {
        fn from(value: &TxExecutionError) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<ActionError> for TxExecutionError {
        fn from(value: ActionError) -> Self {
            Self::ActionError(value)
        }
    }

    impl ::std::convert::From<InvalidTxError> for TxExecutionError {
        fn from(value: InvalidTxError) -> Self {
            Self::InvalidTxError(value)
        }
    }

    ///TxExecutionStatus
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "description": "Transaction is waiting to be included into the
    /// block",
    ///      "type": "string",
    ///      "enum": [
    ///        "NONE"
    ///      ]
    ///    },
    ///    {
    ///      "description": "Transaction is included into the block. The block
    /// may be not finalized yet",
    ///      "type": "string",
    ///      "enum": [
    ///        "INCLUDED"
    ///      ]
    ///    },
    ///    {
    ///      "description": "Transaction is included into the block + All non-refund transaction receipts finished their execution. The corresponding blocks for tx and each receipt may be not finalized yet",
    ///      "type": "string",
    ///      "enum": [
    ///        "EXECUTED_OPTIMISTIC"
    ///      ]
    ///    },
    ///    {
    ///      "description": "Transaction is included into finalized block",
    ///      "type": "string",
    ///      "enum": [
    ///        "INCLUDED_FINAL"
    ///      ]
    ///    },
    ///    {
    ///      "description": "Transaction is included into finalized block + All
    /// non-refund transaction receipts finished their execution. The
    /// corresponding blocks for each receipt may be not finalized yet",
    ///      "type": "string",
    ///      "enum": [
    ///        "EXECUTED"
    ///      ]
    ///    },
    ///    {
    ///      "description": "Transaction is included into finalized block +
    /// Execution of all transaction receipts is finalized, including refund
    /// receipts",
    ///      "type": "string",
    ///      "enum": [
    ///        "FINAL"
    ///      ]
    ///    }
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
    pub enum TxExecutionStatus {
        ///Transaction is waiting to be included into the block
        #[serde(rename = "NONE")]
        None,
        ///Transaction is included into the block. The block may be not
        /// finalized yet
        #[serde(rename = "INCLUDED")]
        Included,
        ///Transaction is included into the block + All non-refund transaction
        /// receipts finished their execution. The corresponding blocks for tx
        /// and each receipt may be not finalized yet
        #[serde(rename = "EXECUTED_OPTIMISTIC")]
        ExecutedOptimistic,
        ///Transaction is included into finalized block
        #[serde(rename = "INCLUDED_FINAL")]
        IncludedFinal,
        ///Transaction is included into finalized block + All non-refund
        /// transaction receipts finished their execution. The corresponding
        /// blocks for each receipt may be not finalized yet
        #[serde(rename = "EXECUTED")]
        Executed,
        ///Transaction is included into finalized block + Execution of all
        /// transaction receipts is finalized, including refund receipts
        #[serde(rename = "FINAL")]
        Final,
    }

    impl ::std::convert::From<&Self> for TxExecutionStatus {
        fn from(value: &TxExecutionStatus) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for TxExecutionStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::None => write!(f, "NONE"),
                Self::Included => write!(f, "INCLUDED"),
                Self::ExecutedOptimistic => write!(f, "EXECUTED_OPTIMISTIC"),
                Self::IncludedFinal => write!(f, "INCLUDED_FINAL"),
                Self::Executed => write!(f, "EXECUTED"),
                Self::Final => write!(f, "FINAL"),
            }
        }
    }

    impl ::std::str::FromStr for TxExecutionStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "NONE" => Ok(Self::None),
                "INCLUDED" => Ok(Self::Included),
                "EXECUTED_OPTIMISTIC" => Ok(Self::ExecutedOptimistic),
                "INCLUDED_FINAL" => Ok(Self::IncludedFinal),
                "EXECUTED" => Ok(Self::Executed),
                "FINAL" => Ok(Self::Final),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for TxExecutionStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for TxExecutionStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for TxExecutionStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///TxMethodNameHelperEnum
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "tx"
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
    pub enum TxMethodNameHelperEnum {
        #[serde(rename = "tx")]
        Tx,
    }

    impl ::std::convert::From<&Self> for TxMethodNameHelperEnum {
        fn from(value: &TxMethodNameHelperEnum) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for TxMethodNameHelperEnum {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Tx => write!(f, "tx"),
            }
        }
    }

    impl ::std::str::FromStr for TxMethodNameHelperEnum {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "tx" => Ok(Self::Tx),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for TxMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for TxMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for TxMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///Use global contract action
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Use global contract action",
    ///  "type": "object",
    ///  "required": [
    ///    "contract_identifier"
    ///  ],
    ///  "properties": {
    ///    "contract_identifier": {
    ///      "$ref": "#/components/schemas/GlobalContractIdentifier"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UseGlobalContractAction {
        pub contract_identifier: GlobalContractIdentifier,
    }

    impl ::std::convert::From<&UseGlobalContractAction> for UseGlobalContractAction {
        fn from(value: &UseGlobalContractAction) -> Self {
            value.clone()
        }
    }

    ///ValidatorInfo
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "account_id",
    ///    "is_slashed"
    ///  ],
    ///  "properties": {
    ///    "account_id": {
    ///      "$ref": "#/components/schemas/AccountId"
    ///    },
    ///    "is_slashed": {
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ValidatorInfo {
        pub account_id: AccountId,
        pub is_slashed: bool,
    }

    impl ::std::convert::From<&ValidatorInfo> for ValidatorInfo {
        fn from(value: &ValidatorInfo) -> Self {
            value.clone()
        }
    }

    ///Reasons for removing a validator from the validator set.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Reasons for removing a validator from the validator
    /// set.",
    ///  "oneOf": [
    ///    {
    ///      "description": "Slashed validators are kicked out.",
    ///      "type": "string",
    ///      "enum": [
    ///        "Slashed"
    ///      ]
    ///    },
    ///    {
    ///      "description": "Validator didn't produce enough blocks.",
    ///      "type": "object",
    ///      "required": [
    ///        "NotEnoughBlocks"
    ///      ],
    ///      "properties": {
    ///        "NotEnoughBlocks": {
    ///          "type": "object",
    ///          "required": [
    ///            "expected",
    ///            "produced"
    ///          ],
    ///          "properties": {
    ///            "expected": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            },
    ///            "produced": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "Validator didn't produce enough chunks.",
    ///      "type": "object",
    ///      "required": [
    ///        "NotEnoughChunks"
    ///      ],
    ///      "properties": {
    ///        "NotEnoughChunks": {
    ///          "type": "object",
    ///          "required": [
    ///            "expected",
    ///            "produced"
    ///          ],
    ///          "properties": {
    ///            "expected": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            },
    ///            "produced": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "Validator unstaked themselves.",
    ///      "type": "string",
    ///      "enum": [
    ///        "Unstaked"
    ///      ]
    ///    },
    ///    {
    ///      "description": "Validator stake is now below threshold",
    ///      "type": "object",
    ///      "required": [
    ///        "NotEnoughStake"
    ///      ],
    ///      "properties": {
    ///        "NotEnoughStake": {
    ///          "type": "object",
    ///          "required": [
    ///            "stake_u128",
    ///            "threshold_u128"
    ///          ],
    ///          "properties": {
    ///            "stake_u128": {
    ///              "type": "string"
    ///            },
    ///            "threshold_u128": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "description": "Enough stake but is not chosen because of seat
    /// limits.",
    ///      "type": "string",
    ///      "enum": [
    ///        "DidNotGetASeat"
    ///      ]
    ///    },
    ///    {
    ///      "description": "Validator didn't produce enough chunk
    /// endorsements.",
    ///      "type": "object",
    ///      "required": [
    ///        "NotEnoughChunkEndorsements"
    ///      ],
    ///      "properties": {
    ///        "NotEnoughChunkEndorsements": {
    ///          "type": "object",
    ///          "required": [
    ///            "expected",
    ///            "produced"
    ///          ],
    ///          "properties": {
    ///            "expected": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            },
    ///            "produced": {
    ///              "type": "integer",
    ///              "format": "uint64",
    ///              "minimum": 0.0
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub enum ValidatorKickoutReason {
        ///Slashed validators are kicked out.
        Slashed,
        ///Validator didn't produce enough blocks.
        NotEnoughBlocks { expected: u64, produced: u64 },
        ///Validator didn't produce enough chunks.
        NotEnoughChunks { expected: u64, produced: u64 },
        ///Validator unstaked themselves.
        Unstaked,
        ///Validator stake is now below threshold
        NotEnoughStake {
            stake_u128: ::std::string::String,
            threshold_u128: ::std::string::String,
        },
        ///Enough stake but is not chosen because of seat limits.
        DidNotGetASeat,
        ///Validator didn't produce enough chunk endorsements.
        NotEnoughChunkEndorsements { expected: u64, produced: u64 },
    }

    impl ::std::convert::From<&Self> for ValidatorKickoutReason {
        fn from(value: &ValidatorKickoutReason) -> Self {
            value.clone()
        }
    }

    ///ValidatorKickoutView
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "account_id",
    ///    "reason"
    ///  ],
    ///  "properties": {
    ///    "account_id": {
    ///      "$ref": "#/components/schemas/AccountId"
    ///    },
    ///    "reason": {
    ///      "$ref": "#/components/schemas/ValidatorKickoutReason"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ValidatorKickoutView {
        pub account_id: AccountId,
        pub reason: ValidatorKickoutReason,
    }

    impl ::std::convert::From<&ValidatorKickoutView> for ValidatorKickoutView {
        fn from(value: &ValidatorKickoutView) -> Self {
            value.clone()
        }
    }

    ///ValidatorStakeView
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "account_id",
    ///        "public_key",
    ///        "stake",
    ///        "validator_stake_struct_version"
    ///      ],
    ///      "properties": {
    ///        "account_id": {
    ///          "$ref": "#/components/schemas/AccountId"
    ///        },
    ///        "public_key": {
    ///          "$ref": "#/components/schemas/PublicKey"
    ///        },
    ///        "stake": {
    ///          "type": "string"
    ///        },
    ///        "validator_stake_struct_version": {
    ///          "type": "string",
    ///          "enum": [
    ///            "V1"
    ///          ]
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(tag = "validator_stake_struct_version")]
    pub enum ValidatorStakeView {
        V1 {
            account_id: AccountId,
            public_key: PublicKey,
            stake: ::std::string::String,
        },
    }

    impl ::std::convert::From<&Self> for ValidatorStakeView {
        fn from(value: &ValidatorStakeView) -> Self {
            value.clone()
        }
    }

    ///ValidatorsMethodNameHelperEnum
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "validators"
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
    pub enum ValidatorsMethodNameHelperEnum {
        #[serde(rename = "validators")]
        Validators,
    }

    impl ::std::convert::From<&Self> for ValidatorsMethodNameHelperEnum {
        fn from(value: &ValidatorsMethodNameHelperEnum) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for ValidatorsMethodNameHelperEnum {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Validators => write!(f, "validators"),
            }
        }
    }

    impl ::std::str::FromStr for ValidatorsMethodNameHelperEnum {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "validators" => Ok(Self::Validators),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for ValidatorsMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for ValidatorsMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ValidatorsMethodNameHelperEnum {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///Data structure for semver version and github tag or commit.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Data structure for semver version and github tag or
    /// commit.",
    ///  "type": "object",
    ///  "required": [
    ///    "build",
    ///    "commit",
    ///    "version"
    ///  ],
    ///  "properties": {
    ///    "build": {
    ///      "type": "string"
    ///    },
    ///    "commit": {
    ///      "type": "string"
    ///    },
    ///    "rustc_version": {
    ///      "default": "",
    ///      "type": "string"
    ///    },
    ///    "version": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Version {
        pub build: ::std::string::String,
        pub commit: ::std::string::String,
        #[serde(default)]
        pub rustc_version: ::std::string::String,
        pub version: ::std::string::String,
    }

    impl ::std::convert::From<&Version> for Version {
        fn from(value: &Version) -> Self {
            value.clone()
        }
    }

    ///VmConfigView
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "alt_bn128",
    ///    "disable_9393_fix",
    ///    "discard_custom_sections",
    ///    "ed25519_verify",
    ///    "eth_implicit_accounts",
    ///    "ext_costs",
    ///    "fix_contract_loading_cost",
    ///    "function_call_weight",
    ///    "grow_mem_cost",
    ///    "implicit_account_creation",
    ///    "limit_config",
    ///    "math_extension",
    ///    "regular_op_cost",
    ///    "storage_get_mode",
    ///    "vm_kind",
    ///    "yield_resume_host_functions"
    ///  ],
    ///  "properties": {
    ///    "alt_bn128": {
    ///      "description": "See
    /// [VMConfig::alt_bn128](crate::vm::Config::alt_bn128).",
    ///      "type": "boolean"
    ///    },
    ///    "disable_9393_fix": {
    ///      "description": "See
    /// [VMConfig::disable_9393_fix](crate::vm::Config::disable_9393_fix).",
    ///      "type": "boolean"
    ///    },
    ///    "discard_custom_sections": {
    ///      "description": "See
    /// [VMConfig::discard_custom_sections](crate::vm::Config::discard_custom_sections).
    /// ",
    ///      "type": "boolean"
    ///    },
    ///    "ed25519_verify": {
    ///      "description": "See
    /// [VMConfig::ed25519_verify](crate::vm::Config::ed25519_verify).",
    ///      "type": "boolean"
    ///    },
    ///    "eth_implicit_accounts": {
    ///      "description": "See
    /// [VMConfig::eth_implicit_accounts](crate::vm::Config::eth_implicit_accounts).
    /// ",
    ///      "type": "boolean"
    ///    },
    ///    "ext_costs": {
    ///      "description": "Costs for runtime externals",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/ExtCostsConfigView"
    ///        }
    ///      ]
    ///    },
    ///    "fix_contract_loading_cost": {
    ///      "description": "See
    /// [VMConfig::fix_contract_loading_cost](crate::vm::Config::fix_contract_loading_cost).
    /// ",
    ///      "type": "boolean"
    ///    },
    ///    "function_call_weight": {
    ///      "description": "See
    /// [VMConfig::function_call_weight](crate::vm::Config::function_call_weight).
    /// ",
    ///      "type": "boolean"
    ///    },
    ///    "grow_mem_cost": {
    ///      "description": "Gas cost of a growing memory by single page.",
    ///      "type": "integer",
    ///      "format": "uint32",
    ///      "minimum": 0.0
    ///    },
    ///    "implicit_account_creation": {
    ///      "description": "See
    /// [VMConfig::implicit_account_creation](crate::vm::Config::implicit_account_creation).
    /// ",
    ///      "type": "boolean"
    ///    },
    ///    "limit_config": {
    ///      "description": "Describes limits for VM and Runtime.\n\nTODO:
    /// Consider changing this to `VMLimitConfigView` to avoid dependency on
    /// runtime.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/LimitConfig"
    ///        }
    ///      ]
    ///    },
    ///    "math_extension": {
    ///      "description": "See
    /// [VMConfig::math_extension](crate::vm::Config::math_extension).",
    ///      "type": "boolean"
    ///    },
    ///    "regular_op_cost": {
    ///      "description": "Gas cost of a regular operation.",
    ///      "type": "integer",
    ///      "format": "uint32",
    ///      "minimum": 0.0
    ///    },
    ///    "storage_get_mode": {
    ///      "description": "See
    /// [VMConfig::storage_get_mode](crate::vm::Config::storage_get_mode).",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/StorageGetMode"
    ///        }
    ///      ]
    ///    },
    ///    "vm_kind": {
    ///      "description": "See
    /// [VMConfig::vm_kind](crate::vm::Config::vm_kind).",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/VMKind"
    ///        }
    ///      ]
    ///    },
    ///    "yield_resume_host_functions": {
    ///      "description": "See
    /// [VMConfig::yield_resume_host_functions](`crate::vm::Config::yield_resume_host_functions).
    /// ",
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct VmConfigView {
        ///See [VMConfig::alt_bn128](crate::vm::Config::alt_bn128).
        pub alt_bn128: bool,
        ///See [VMConfig::disable_9393_fix](crate::vm::Config::disable_9393_fix).
        pub disable_9393_fix: bool,
        ///See [VMConfig::discard_custom_sections](crate::vm::Config::discard_custom_sections).
        pub discard_custom_sections: bool,
        ///See [VMConfig::ed25519_verify](crate::vm::Config::ed25519_verify).
        pub ed25519_verify: bool,
        ///See [VMConfig::eth_implicit_accounts](crate::vm::Config::eth_implicit_accounts).
        pub eth_implicit_accounts: bool,
        ///Costs for runtime externals
        pub ext_costs: ExtCostsConfigView,
        ///See [VMConfig::fix_contract_loading_cost](crate::vm::Config::fix_contract_loading_cost).
        pub fix_contract_loading_cost: bool,
        ///See [VMConfig::function_call_weight](crate::vm::Config::function_call_weight).
        pub function_call_weight: bool,
        ///Gas cost of a growing memory by single page.
        pub grow_mem_cost: u32,
        ///See [VMConfig::implicit_account_creation](crate::vm::Config::implicit_account_creation).
        pub implicit_account_creation: bool,
        ///Describes limits for VM and Runtime.
        ///
        ///TODO: Consider changing this to `VMLimitConfigView` to avoid
        /// dependency on runtime.
        pub limit_config: LimitConfig,
        ///See [VMConfig::math_extension](crate::vm::Config::math_extension).
        pub math_extension: bool,
        ///Gas cost of a regular operation.
        pub regular_op_cost: u32,
        ///See [VMConfig::storage_get_mode](crate::vm::Config::storage_get_mode).
        pub storage_get_mode: StorageGetMode,
        ///See [VMConfig::vm_kind](crate::vm::Config::vm_kind).
        pub vm_kind: VmKind,
        ///See [VMConfig::yield_resume_host_functions](`crate::vm::Config::yield_resume_host_functions).
        pub yield_resume_host_functions: bool,
    }

    impl ::std::convert::From<&VmConfigView> for VmConfigView {
        fn from(value: &VmConfigView) -> Self {
            value.clone()
        }
    }

    ///VmKind
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "description": "Wasmer 0.17.x VM.",
    ///      "type": "string",
    ///      "enum": [
    ///        "Wasmer0"
    ///      ]
    ///    },
    ///    {
    ///      "description": "Wasmtime VM.",
    ///      "type": "string",
    ///      "enum": [
    ///        "Wasmtime"
    ///      ]
    ///    },
    ///    {
    ///      "description": "Wasmer 2.x VM.",
    ///      "type": "string",
    ///      "enum": [
    ///        "Wasmer2"
    ///      ]
    ///    },
    ///    {
    ///      "description": "NearVM.",
    ///      "type": "string",
    ///      "enum": [
    ///        "NearVm"
    ///      ]
    ///    }
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
    pub enum VmKind {
        ///Wasmer 0.17.x VM.
        Wasmer0,
        ///Wasmtime VM.
        Wasmtime,
        ///Wasmer 2.x VM.
        Wasmer2,
        ///NearVM.
        NearVm,
    }

    impl ::std::convert::From<&Self> for VmKind {
        fn from(value: &VmKind) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for VmKind {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Wasmer0 => write!(f, "Wasmer0"),
                Self::Wasmtime => write!(f, "Wasmtime"),
                Self::Wasmer2 => write!(f, "Wasmer2"),
                Self::NearVm => write!(f, "NearVm"),
            }
        }
    }

    impl ::std::str::FromStr for VmKind {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "Wasmer0" => Ok(Self::Wasmer0),
                "Wasmtime" => Ok(Self::Wasmtime),
                "Wasmer2" => Ok(Self::Wasmer2),
                "NearVm" => Ok(Self::NearVm),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for VmKind {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for VmKind {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for VmKind {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///A kind of a trap happened during execution of a binary
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A kind of a trap happened during execution of a
    /// binary",
    ///  "oneOf": [
    ///    {
    ///      "description": "An `unreachable` opcode was executed.",
    ///      "type": "string",
    ///      "enum": [
    ///        "Unreachable"
    ///      ]
    ///    },
    ///    {
    ///      "description": "Call indirect incorrect signature trap.",
    ///      "type": "string",
    ///      "enum": [
    ///        "IncorrectCallIndirectSignature"
    ///      ]
    ///    },
    ///    {
    ///      "description": "Memory out of bounds trap.",
    ///      "type": "string",
    ///      "enum": [
    ///        "MemoryOutOfBounds"
    ///      ]
    ///    },
    ///    {
    ///      "description": "Call indirect out of bounds trap.",
    ///      "type": "string",
    ///      "enum": [
    ///        "CallIndirectOOB"
    ///      ]
    ///    },
    ///    {
    ///      "description": "An arithmetic exception, e.g. divided by zero.",
    ///      "type": "string",
    ///      "enum": [
    ///        "IllegalArithmetic"
    ///      ]
    ///    },
    ///    {
    ///      "description": "Misaligned atomic access trap.",
    ///      "type": "string",
    ///      "enum": [
    ///        "MisalignedAtomicAccess"
    ///      ]
    ///    },
    ///    {
    ///      "description": "Indirect call to null.",
    ///      "type": "string",
    ///      "enum": [
    ///        "IndirectCallToNull"
    ///      ]
    ///    },
    ///    {
    ///      "description": "Stack overflow.",
    ///      "type": "string",
    ///      "enum": [
    ///        "StackOverflow"
    ///      ]
    ///    },
    ///    {
    ///      "description": "Generic trap.",
    ///      "type": "string",
    ///      "enum": [
    ///        "GenericTrap"
    ///      ]
    ///    }
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
    pub enum WasmTrap {
        ///An `unreachable` opcode was executed.
        Unreachable,
        ///Call indirect incorrect signature trap.
        IncorrectCallIndirectSignature,
        ///Memory out of bounds trap.
        MemoryOutOfBounds,
        ///Call indirect out of bounds trap.
        #[serde(rename = "CallIndirectOOB")]
        CallIndirectOob,
        ///An arithmetic exception, e.g. divided by zero.
        IllegalArithmetic,
        ///Misaligned atomic access trap.
        MisalignedAtomicAccess,
        ///Indirect call to null.
        IndirectCallToNull,
        ///Stack overflow.
        StackOverflow,
        ///Generic trap.
        GenericTrap,
    }

    impl ::std::convert::From<&Self> for WasmTrap {
        fn from(value: &WasmTrap) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for WasmTrap {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Unreachable => write!(f, "Unreachable"),
                Self::IncorrectCallIndirectSignature => write!(f, "IncorrectCallIndirectSignature"),
                Self::MemoryOutOfBounds => write!(f, "MemoryOutOfBounds"),
                Self::CallIndirectOob => write!(f, "CallIndirectOOB"),
                Self::IllegalArithmetic => write!(f, "IllegalArithmetic"),
                Self::MisalignedAtomicAccess => write!(f, "MisalignedAtomicAccess"),
                Self::IndirectCallToNull => write!(f, "IndirectCallToNull"),
                Self::StackOverflow => write!(f, "StackOverflow"),
                Self::GenericTrap => write!(f, "GenericTrap"),
            }
        }
    }

    impl ::std::str::FromStr for WasmTrap {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "Unreachable" => Ok(Self::Unreachable),
                "IncorrectCallIndirectSignature" => Ok(Self::IncorrectCallIndirectSignature),
                "MemoryOutOfBounds" => Ok(Self::MemoryOutOfBounds),
                "CallIndirectOOB" => Ok(Self::CallIndirectOob),
                "IllegalArithmetic" => Ok(Self::IllegalArithmetic),
                "MisalignedAtomicAccess" => Ok(Self::MisalignedAtomicAccess),
                "IndirectCallToNull" => Ok(Self::IndirectCallToNull),
                "StackOverflow" => Ok(Self::StackOverflow),
                "GenericTrap" => Ok(Self::GenericTrap),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for WasmTrap {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for WasmTrap {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for WasmTrap {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///Configuration specific to ChunkStateWitness.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Configuration specific to ChunkStateWitness.",
    ///  "type": "object",
    ///  "required": [
    ///    "combined_transactions_size_limit",
    ///    "main_storage_proof_size_soft_limit",
    ///    "new_transactions_validation_state_size_soft_limit"
    ///  ],
    ///  "properties": {
    ///    "combined_transactions_size_limit": {
    ///      "description": "A witness contains transactions from both the
    /// previous chunk and the current one. This parameter limits the sum of
    /// sizes of transactions from both of those chunks.",
    ///      "type": "integer",
    ///      "format": "uint",
    ///      "minimum": 0.0
    ///    },
    ///    "main_storage_proof_size_soft_limit": {
    ///      "description": "Size limit for storage proof generated while
    /// executing receipts in a chunk. After this limit is reached we defer
    /// execution of any new receipts.",
    ///      "type": "integer",
    ///      "format": "uint",
    ///      "minimum": 0.0
    ///    },
    ///    "new_transactions_validation_state_size_soft_limit": {
    ///      "description": "Soft size limit of storage proof used to validate
    /// new transactions in ChunkStateWitness.",
    ///      "type": "integer",
    ///      "format": "uint",
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct WitnessConfigView {
        ///A witness contains transactions from both the previous chunk and the
        /// current one. This parameter limits the sum of sizes of transactions
        /// from both of those chunks.
        pub combined_transactions_size_limit: u32,
        ///Size limit for storage proof generated while executing receipts in a
        /// chunk. After this limit is reached we defer execution of any new
        /// receipts.
        pub main_storage_proof_size_soft_limit: u32,
        ///Soft size limit of storage proof used to validate new transactions
        /// in ChunkStateWitness.
        pub new_transactions_validation_state_size_soft_limit: u32,
    }

    impl ::std::convert::From<&WitnessConfigView> for WitnessConfigView {
        fn from(value: &WitnessConfigView) -> Self {
            value.clone()
        }
    }

    /// Generation of default values for serde.
    pub mod defaults {
        pub(super) fn default_u64<T, const V: u64>() -> T
        where
            T: std::convert::TryFrom<u64>,
            <T as std::convert::TryFrom<u64>>::Error: std::fmt::Debug,
        {
            T::try_from(V).unwrap()
        }

        pub(super) fn execution_outcome_view_metadata() -> super::ExecutionMetadataView {
            super::ExecutionMetadataView {
                gas_profile: ::std::option::Option::None,
                version: 1_u32,
            }
        }

        pub(super) fn gc_config_gc_step_period() -> super::DurationSchemeProvider {
            super::DurationSchemeProvider {
                nanoseconds: 0_i32,
                seconds: 1_i64,
            }
        }

        pub(super) fn genesis_config_minimum_stake_ratio() -> super::Rational32SchemaProvider {
            super::Rational32SchemaProvider {
                denom: 1_i32,
                numer: 6250_i32,
            }
        }

        pub(super) fn genesis_config_online_max_threshold() -> super::Rational32SchemaProvider {
            super::Rational32SchemaProvider {
                denom: 99_i32,
                numer: 100_i32,
            }
        }

        pub(super) fn genesis_config_online_min_threshold() -> super::Rational32SchemaProvider {
            super::Rational32SchemaProvider {
                denom: 9_i32,
                numer: 10_i32,
            }
        }

        pub(super) fn genesis_config_protocol_upgrade_stake_threshold(
        ) -> super::Rational32SchemaProvider {
            super::Rational32SchemaProvider {
                denom: 4_i32,
                numer: 5_i32,
            }
        }

        pub(super) fn genesis_config_shard_layout() -> super::ShardLayout {
            super::ShardLayout::V2(super::ShardLayoutV2 {
                boundary_accounts: vec![],
                id_to_index_map: [("0".to_string(), 0_u32)].into_iter().collect(),
                index_to_id_map: [("0".to_string(), super::ShardId(0_u64))]
                    .into_iter()
                    .collect(),
                shard_ids: vec![super::ShardId(0_u64)],
                shards_parent_map: ::std::option::Option::None,
                shards_split_map: ::std::option::Option::None,
                version: 0_u32,
            })
        }

        pub(super) fn rpc_send_transaction_request_wait_until() -> super::TxExecutionStatus {
            super::TxExecutionStatus::ExecutedOptimistic
        }

        pub(super) fn rpc_transaction_status_request_variant1_wait_until(
        ) -> super::TxExecutionStatus {
            super::TxExecutionStatus::ExecutedOptimistic
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
    ///Sends a `POST` request to `/EXPERIMENTAL_changes`
    pub async fn experimental_changes<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForExpChangeMethodNameHelperEnum,
    ) -> Result<
        ResponseValue<types::JsonRpcResponseForRpcStateChangesInBlockResponseAndRpcError>,
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

    ///Sends a `POST` request to `/EXPERIMENTAL_changes_in_block`
    pub async fn experimental_changes_in_block<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForExpChangesBlockMethodNameHelperEnum,
    ) -> Result<
        ResponseValue<types::JsonRpcResponseForRpcStateChangesInBlockByTypeResponseAndRpcError>,
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

    ///Sends a `POST` request to `/EXPERIMENTAL_congestion_level`
    pub async fn experimental_congestion_level<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForExpGongestionMethodNameHelperEnum,
    ) -> Result<
        ResponseValue<types::JsonRpcResponseForRpcCongestionLevelResponseAndRpcError>,
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

    ///Sends a `POST` request to `/EXPERIMENTAL_genesis_config`
    pub async fn experimental_genesis_config<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForExpGenesisMethodNameHelperEnum,
    ) -> Result<ResponseValue<types::JsonRpcResponseForGenesisConfigAndRpcError>, Error<()>> {
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

    ///Sends a `POST` request to `/EXPERIMENTAL_light_client_block_proof`
    pub async fn experimental_light_client_block_proof<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForExpLightClientBlockProofMethodNameHelperEnum,
    ) -> Result<
        ResponseValue<types::JsonRpcResponseForRpcLightClientBlockProofResponseAndRpcError>,
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

    ///Sends a `POST` request to `/EXPERIMENTAL_light_client_proof`
    pub async fn experimental_light_client_proof<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForExpLightClientProofMethodNameHelperEnum,
    ) -> Result<
        ResponseValue<types::JsonRpcResponseForRpcLightClientExecutionProofResponseAndRpcError>,
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

    ///Sends a `POST` request to `/EXPERIMENTAL_protocol_config`
    pub async fn experimental_protocol_config<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForExpProtocolConfigMethodNameHelperEnum,
    ) -> Result<
        ResponseValue<types::JsonRpcResponseForRpcProtocolConfigResponseAndRpcError>,
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

    ///Sends a `POST` request to `/EXPERIMENTAL_receipt`
    pub async fn experimental_receipt<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForExpReceiptMethodNameHelperEnum,
    ) -> Result<ResponseValue<types::JsonRpcResponseForRpcReceiptResponseAndRpcError>, Error<()>>
    {
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

    ///Sends a `POST` request to `/EXPERIMENTAL_tx_status`
    pub async fn experimental_tx_status<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForExpTxStatusMethodNameHelperEnum,
    ) -> Result<ResponseValue<types::JsonRpcResponseForRpcTransactionResponseAndRpcError>, Error<()>>
    {
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

    ///Sends a `POST` request to `/EXPERIMENTAL_validators_ordered`
    pub async fn experimental_validators_ordered<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForExpValidatorsMethodNameHelperEnum,
    ) -> Result<
        ResponseValue<types::JsonRpcResponseForArrayOfValidatorStakeViewAndRpcError>,
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

    ///Sends a `POST` request to `/block`
    pub async fn block<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForBlockMethodNameHelperEnum,
    ) -> Result<ResponseValue<types::JsonRpcResponseForRpcBlockResponseAndRpcError>, Error<()>>
    {
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

    ///Sends a `POST` request to `/broadcast_tx_async`
    pub async fn broadcast_tx_async<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForBroadCastTxAsyncMethodNameHelperEnum,
    ) -> Result<ResponseValue<types::JsonRpcResponseForRpcBlockResponseAndRpcError>, Error<()>>
    {
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

    ///Sends a `POST` request to `/broadcast_tx_commit`
    pub async fn broadcast_tx_commit<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForBroadCastTxCommitMethodNameHelperEnum,
    ) -> Result<ResponseValue<types::JsonRpcResponseForRpcTransactionResponseAndRpcError>, Error<()>>
    {
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

    ///Sends a `POST` request to `/chunk`
    pub async fn chunk<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForBlockMethodNameHelperEnum,
    ) -> Result<ResponseValue<types::JsonRpcResponseForRpcBlockResponseAndRpcError>, Error<()>>
    {
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

    ///Sends a `POST` request to `/client_config`
    pub async fn client_config<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForClientConfigMethodNameHelperEnum,
    ) -> Result<ResponseValue<types::JsonRpcResponseForRpcClientConfigResponseAndRpcError>, Error<()>>
    {
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

    ///Sends a `POST` request to `/gas_price`
    pub async fn gas_price<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForGasPriceMethodNameHelperEnum,
    ) -> Result<ResponseValue<types::JsonRpcResponseForRpcGasPriceResponseAndRpcError>, Error<()>>
    {
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

    ///Sends a `POST` request to `/health`
    pub async fn health<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForHealthMethodNameHelperEnum,
    ) -> Result<ResponseValue<types::JsonRpcResponseForRpcHealthResponseAndRpcError>, Error<()>>
    {
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

    ///Sends a `POST` request to `/light_client_proof`
    pub async fn light_client_proof<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForLightClientProofMethodNameHelperEnum,
    ) -> Result<
        ResponseValue<types::JsonRpcResponseForRpcLightClientExecutionProofResponseAndRpcError>,
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

    ///Sends a `POST` request to `/network_info`
    pub async fn network_info<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForNetworkInfoMethodNameHelperEnum,
    ) -> Result<ResponseValue<types::JsonRpcResponseForRpcNetworkInfoResponseAndRpcError>, Error<()>>
    {
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

    ///Sends a `POST` request to `/next_light_client_block`
    pub async fn next_light_client_block<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForNextLightClientBlockMethodNameHelperEnum,
    ) -> Result<
        ResponseValue<types::JsonRpcResponseForRpcLightClientNextBlockResponseAndRpcError>,
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

    ///Sends a `POST` request to `/send_tx`
    pub async fn send_tx<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForSendTxMethodNameHelperEnum,
    ) -> Result<ResponseValue<types::JsonRpcResponseForRpcTransactionResponseAndRpcError>, Error<()>>
    {
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

    ///Sends a `POST` request to `/status`
    pub async fn status<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForStatusMethodNameHelperEnum,
    ) -> Result<ResponseValue<types::JsonRpcResponseForRpcStatusResponseAndRpcError>, Error<()>>
    {
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

    ///Sends a `POST` request to `/tx`
    pub async fn tx<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForTxMethodNameHelperEnum,
    ) -> Result<ResponseValue<types::JsonRpcResponseForRpcTransactionResponseAndRpcError>, Error<()>>
    {
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

    ///Sends a `POST` request to `/validators`
    pub async fn validators<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForValidatorsMethodNameHelperEnum,
    ) -> Result<ResponseValue<types::JsonRpcResponseForRpcValidatorResponseAndRpcError>, Error<()>>
    {
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
