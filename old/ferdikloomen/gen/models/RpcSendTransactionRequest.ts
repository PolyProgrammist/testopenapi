/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { TxExecutionStatus } from './TxExecutionStatus';
export type RpcSendTransactionRequest = {
    signed_tx_base64: string;
    wait_until?: TxExecutionStatus;
};

