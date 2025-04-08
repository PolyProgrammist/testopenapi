/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { RpcError } from './RpcError';
import type { RpcProtocolConfigResponse } from './RpcProtocolConfigResponse';
export type JsonRpcResponse_for_RpcProtocolConfigResponse_and_RpcError = ({
    result: RpcProtocolConfigResponse;
} | {
    tmp: RpcError;
} | {
    error: RpcError;
} | {
    id: string;
    jsonrpc: string;
});

