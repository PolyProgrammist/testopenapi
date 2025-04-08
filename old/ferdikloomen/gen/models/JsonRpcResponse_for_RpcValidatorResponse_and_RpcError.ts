/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { RpcError } from './RpcError';
import type { RpcValidatorResponse } from './RpcValidatorResponse';
export type JsonRpcResponse_for_RpcValidatorResponse_and_RpcError = ({
    result: RpcValidatorResponse;
} | {
    tmp: RpcError;
} | {
    error: RpcError;
} | {
    id: string;
    jsonrpc: string;
});

