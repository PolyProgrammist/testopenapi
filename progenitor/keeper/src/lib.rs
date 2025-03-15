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

    ///JsonRpcRequestForRpcTransactionStatusRequest
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "JsonRpcRequest_for_RpcTransactionStatusRequest",
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
    ///      "$ref": "#/components/schemas/Tx_enum"
    ///    },
    ///    "params": {
    ///      "$ref": "#/components/schemas/RpcTransactionStatusRequest"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct JsonRpcRequestForRpcTransactionStatusRequest {
        pub id: ::std::string::String,
        pub jsonrpc: ::std::string::String,
        pub method: TxEnum,
        pub params: RpcTransactionStatusRequest,
    }

    impl ::std::convert::From<&JsonRpcRequestForRpcTransactionStatusRequest>
        for JsonRpcRequestForRpcTransactionStatusRequest
    {
        fn from(value: &JsonRpcRequestForRpcTransactionStatusRequest) -> Self {
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
    ///      "description": "Deprecated please use the `error_struct` instead",
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "data": {
    ///      "description": "Deprecated please use the `error_struct` instead"
    ///    },
    ///    "message": {
    ///      "description": "Deprecated please use the `error_struct` instead",
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
        ///Deprecated please use the `error_struct` instead
        pub code: i64,
        ///Deprecated please use the `error_struct` instead
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub data: ::std::option::Option<::serde_json::Value>,
        ///Deprecated please use the `error_struct` instead
        pub message: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<NameRpcErrorKind>,
    }

    impl ::std::convert::From<&RpcError> for RpcError {
        fn from(value: &RpcError) -> Self {
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
    ///          "type": "string"
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
        MissingTrieValue(::std::string::String),
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

    ///TxEnum
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
    pub enum TxEnum {
        #[serde(rename = "tx")]
        Tx,
    }

    impl ::std::convert::From<&Self> for TxEnum {
        fn from(value: &TxEnum) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for TxEnum {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Tx => write!(f, "tx"),
            }
        }
    }

    impl ::std::str::FromStr for TxEnum {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "tx" => Ok(Self::Tx),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for TxEnum {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for TxEnum {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for TxEnum {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
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

    /// Generation of default values for serde.
    pub mod defaults {
        pub(super) fn execution_outcome_view_metadata() -> super::ExecutionMetadataView {
            super::ExecutionMetadataView {
                gas_profile: ::std::option::Option::None,
                version: 1_u32,
            }
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
    ///Create a new user
    ///
    ///Sends a `POST` request to `/users`
    ///
    ///Arguments:
    /// - `body`: User registration data
    pub async fn create_user<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForRpcTransactionStatusRequest,
    ) -> Result<ResponseValue<types::JsonRpcResponseForRpcTransactionResponseAndRpcError>, Error<()>>
    {
        let url = format!("{}/users", self.baseurl,);
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
            201u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

/// Items consumers will typically use such as the Client.
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
}
