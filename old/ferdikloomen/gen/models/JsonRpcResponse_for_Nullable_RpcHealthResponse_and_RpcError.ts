/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { RpcError } from './RpcError';
import type { RpcHealthResponse } from './RpcHealthResponse';
export type JsonRpcResponse_for_Nullable_RpcHealthResponse_and_RpcError = ({
    result?: RpcHealthResponse | null;
} | {
    tmp: RpcError;
} | {
    error: RpcError;
} | {
    id: string;
    jsonrpc: string;
});

