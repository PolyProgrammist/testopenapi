/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { RpcError } from './RpcError';
import type { RpcStateChangesInBlockResponse } from './RpcStateChangesInBlockResponse';
export type JsonRpcResponse_for_RpcStateChangesInBlockResponse_and_RpcError = ({
    result: RpcStateChangesInBlockResponse;
} | {
    tmp: RpcError;
} | {
    error: RpcError;
} | {
    id: string;
    jsonrpc: string;
});

