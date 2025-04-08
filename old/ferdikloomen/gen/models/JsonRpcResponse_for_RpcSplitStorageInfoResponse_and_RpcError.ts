/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { RpcError } from './RpcError';
import type { RpcSplitStorageInfoResponse } from './RpcSplitStorageInfoResponse';
export type JsonRpcResponse_for_RpcSplitStorageInfoResponse_and_RpcError = ({
    result: RpcSplitStorageInfoResponse;
} | {
    tmp: RpcError;
} | {
    error: RpcError;
} | {
    id: string;
    jsonrpc: string;
});

