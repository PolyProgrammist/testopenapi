/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { CryptoHash } from './CryptoHash';
import type { TxExecutionError } from './TxExecutionError';
export type ExecutionStatusView = (string | {
    Failure: TxExecutionError;
} | {
    SuccessValue: string;
} | {
    SuccessReceiptId: CryptoHash;
});

