/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { RpcError } from './RpcError';
import type { RpcNetworkInfoResponse } from './RpcNetworkInfoResponse';
export type JsonRpcResponse_for_RpcNetworkInfoResponse_and_RpcError = ({
    result: RpcNetworkInfoResponse;
} | {
    tmp: RpcError;
} | {
    error: RpcError;
} | {
    id: string;
    jsonrpc: string;
});

