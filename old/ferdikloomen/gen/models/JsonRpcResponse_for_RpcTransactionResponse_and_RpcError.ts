/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { RpcError } from './RpcError';
import type { RpcTransactionResponse } from './RpcTransactionResponse';
export type JsonRpcResponse_for_RpcTransactionResponse_and_RpcError = ({
    result: RpcTransactionResponse;
} | {
    tmp: RpcError;
} | {
    error: RpcError;
} | {
    id: string;
    jsonrpc: string;
});

