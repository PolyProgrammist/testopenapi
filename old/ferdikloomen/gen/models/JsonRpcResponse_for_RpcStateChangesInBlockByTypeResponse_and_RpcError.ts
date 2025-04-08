/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { RpcError } from './RpcError';
import type { RpcStateChangesInBlockByTypeResponse } from './RpcStateChangesInBlockByTypeResponse';
export type JsonRpcResponse_for_RpcStateChangesInBlockByTypeResponse_and_RpcError = ({
    result: RpcStateChangesInBlockByTypeResponse;
} | {
    tmp: RpcError;
} | {
    error: RpcError;
} | {
    id: string;
    jsonrpc: string;
});

