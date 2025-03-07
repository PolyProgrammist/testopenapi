/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { AccountId } from './AccountId';
import type { ActionsValidationError } from './ActionsValidationError';
import type { InvalidAccessKeyError } from './InvalidAccessKeyError';
import type { StorageError } from './StorageError';
/**
 * An error happened during TX execution
 */
export type InvalidTxError = ({
    InvalidAccessKeyError: InvalidAccessKeyError;
} | {
    InvalidSignerId: {
        signer_id: string;
    };
} | {
    SignerDoesNotExist: {
        signer_id: AccountId;
    };
} | {
    InvalidNonce: {
        ak_nonce: number;
        tx_nonce: number;
    };
} | {
    NonceTooLarge: {
        tx_nonce: number;
        upper_bound: number;
    };
} | {
    InvalidReceiverId: {
        receiver_id: string;
    };
} | 'InvalidSignature' | {
    NotEnoughBalance: {
        balance: string;
        cost: string;
        signer_id: AccountId;
    };
} | {
    LackBalanceForState: {
        /**
         * Required balance to cover the state.
         */
        amount: string;
        /**
         * An account which doesn't have enough balance to cover storage.
         */
        signer_id: AccountId;
    };
} | 'CostOverflow' | 'InvalidChain' | 'Expired' | {
    ActionsValidation: ActionsValidationError;
} | {
    TransactionSizeExceeded: {
        limit: number;
        size: number;
    };
} | 'InvalidTransactionVersion' | {
    StorageError: StorageError;
} | {
    ShardCongested: {
        /**
         * A value between 0 (no congestion) and 1 (max congestion).
         */
        congestion_level: number;
        /**
         * The congested shard.
         */
        shard_id: number;
    };
} | {
    ShardStuck: {
        /**
         * The number of blocks since the last included chunk of the shard.
         */
        missed_chunks: number;
        /**
         * The shard that fails making progress.
         */
        shard_id: number;
    };
});

