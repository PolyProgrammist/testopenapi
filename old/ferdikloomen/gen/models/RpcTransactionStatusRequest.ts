/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { AccountId } from './AccountId';
import type { CryptoHash } from './CryptoHash';
import type { SignedTransaction } from './SignedTransaction';
import type { TxExecutionStatus } from './TxExecutionStatus';
export type RpcTransactionStatusRequest = (SignedTransaction | {
    sender_account_id: AccountId;
    tx_hash: CryptoHash;
} | {
    wait_until?: TxExecutionStatus;
});

