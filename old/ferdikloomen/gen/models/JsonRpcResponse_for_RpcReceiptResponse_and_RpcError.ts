/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { RpcError } from './RpcError';
import type { RpcReceiptResponse } from './RpcReceiptResponse';
export type JsonRpcResponse_for_RpcReceiptResponse_and_RpcError = ({
    result: RpcReceiptResponse;
} | {
    tmp: RpcError;
} | {
    error: RpcError;
} | {
    id: string;
    jsonrpc: string;
});

