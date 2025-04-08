/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { RpcBlockResponse } from './RpcBlockResponse';
import type { RpcError } from './RpcError';
export type JsonRpcResponse_for_RpcBlockResponse_and_RpcError = ({
    result: RpcBlockResponse;
} | {
    tmp: RpcError;
} | {
    error: RpcError;
} | {
    id: string;
    jsonrpc: string;
});

