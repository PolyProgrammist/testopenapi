/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { RpcClientConfigResponse } from './RpcClientConfigResponse';
import type { RpcError } from './RpcError';
export type JsonRpcResponse_for_RpcClientConfigResponse_and_RpcError = ({
    result: RpcClientConfigResponse;
} | {
    tmp: RpcError;
} | {
    error: RpcError;
} | {
    id: string;
    jsonrpc: string;
});

