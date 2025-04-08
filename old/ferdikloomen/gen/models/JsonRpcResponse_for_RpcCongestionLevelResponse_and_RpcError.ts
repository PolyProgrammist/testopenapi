/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { RpcCongestionLevelResponse } from './RpcCongestionLevelResponse';
import type { RpcError } from './RpcError';
export type JsonRpcResponse_for_RpcCongestionLevelResponse_and_RpcError = ({
    result: RpcCongestionLevelResponse;
} | {
    tmp: RpcError;
} | {
    error: RpcError;
} | {
    id: string;
    jsonrpc: string;
});

