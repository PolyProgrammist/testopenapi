/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { RpcError } from './RpcError';
import type { RpcStatusResponse } from './RpcStatusResponse';
export type JsonRpcResponse_for_RpcStatusResponse_and_RpcError = ({
    result: RpcStatusResponse;
} | {
    tmp: RpcError;
} | {
    error: RpcError;
} | {
    id: string;
    jsonrpc: string;
});

