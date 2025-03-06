/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { AccountId } from './AccountId';
import type { FunctionCallError } from './FunctionCallError';
import type { GlobalContractIdentifier } from './GlobalContractIdentifier';
import type { InvalidAccessKeyError } from './InvalidAccessKeyError';
import type { PublicKey } from './PublicKey';
import type { ReceiptValidationError } from './ReceiptValidationError';
export type ActionErrorKind = ({
    AccountAlreadyExists: {
        account_id: AccountId;
    };
} | {
    AccountDoesNotExist: {
        account_id: AccountId;
    };
} | {
    CreateAccountOnlyByRegistrar: {
        account_id: AccountId;
        predecessor_id: AccountId;
        registrar_account_id: AccountId;
    };
} | {
    CreateAccountNotAllowed: {
        account_id: AccountId;
        predecessor_id: AccountId;
    };
} | {
    ActorNoPermission: {
        account_id: AccountId;
        actor_id: AccountId;
    };
} | {
    DeleteKeyDoesNotExist: {
        account_id: AccountId;
        public_key: PublicKey;
    };
} | {
    AddKeyAlreadyExists: {
        account_id: AccountId;
        public_key: PublicKey;
    };
} | {
    DeleteAccountStaking: {
        account_id: AccountId;
    };
} | {
    LackBalanceForState: {
        /**
         * An account which needs balance
         */
        account_id: AccountId;
        /**
         * Balance required to complete an action.
         */
        amount: string;
    };
} | {
    TriesToUnstake: {
        account_id: AccountId;
    };
} | {
    TriesToStake: {
        account_id: AccountId;
        balance: string;
        locked: string;
        stake: string;
    };
} | {
    InsufficientStake: {
        account_id: AccountId;
        minimum_stake: string;
        stake: string;
    };
} | {
    FunctionCallError: FunctionCallError;
} | {
    NewReceiptValidationError: ReceiptValidationError;
} | {
    OnlyImplicitAccountCreationAllowed: {
        account_id: AccountId;
    };
} | {
    DeleteAccountWithLargeState: {
        account_id: AccountId;
    };
} | 'DelegateActionInvalidSignature' | {
    DelegateActionSenderDoesNotMatchTxReceiver: {
        receiver_id: AccountId;
        sender_id: AccountId;
    };
} | 'DelegateActionExpired' | {
    DelegateActionAccessKeyError: InvalidAccessKeyError;
} | {
    DelegateActionInvalidNonce: {
        ak_nonce: number;
        delegate_nonce: number;
    };
} | {
    DelegateActionNonceTooLarge: {
        delegate_nonce: number;
        upper_bound: number;
    };
} | {
    GlobalContractDoesNotExist: {
        identifier: GlobalContractIdentifier;
    };
});

