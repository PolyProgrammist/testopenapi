/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { AccountId } from './AccountId';
import type { ChunkHeaderView } from './ChunkHeaderView';
import type { ReceiptView } from './ReceiptView';
import type { SignedTransactionView } from './SignedTransactionView';
export type RpcChunkResponse = {
    author: AccountId;
    header: ChunkHeaderView;
    receipts: Array<ReceiptView>;
    transactions: Array<SignedTransactionView>;
};

