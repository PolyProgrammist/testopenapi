/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { AccountId } from './AccountId';
import type { CryptoHash } from './CryptoHash';
import type { TypeTransactionOrReceiptId } from './TypeTransactionOrReceiptId';
export type RpcLightClientExecutionProofRequest = ({
    sender_id: AccountId;
    transaction_hash: CryptoHash;
} | {
    receipt_id: CryptoHash;
    receiver_id: AccountId;
} | {
    light_client_head: CryptoHash;
    type: TypeTransactionOrReceiptId;
});

