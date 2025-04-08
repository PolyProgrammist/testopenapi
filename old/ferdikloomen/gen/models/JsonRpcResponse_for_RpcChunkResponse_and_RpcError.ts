/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { RpcChunkResponse } from './RpcChunkResponse';
import type { RpcError } from './RpcError';
export type JsonRpcResponse_for_RpcChunkResponse_and_RpcError = ({
    result: RpcChunkResponse;
} | {
    tmp: RpcError;
} | {
    error: RpcError;
} | {
    id: string;
    jsonrpc: string;
});

