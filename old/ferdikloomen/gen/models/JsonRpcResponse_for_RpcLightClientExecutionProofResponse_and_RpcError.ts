/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { RpcError } from './RpcError';
import type { RpcLightClientExecutionProofResponse } from './RpcLightClientExecutionProofResponse';
export type JsonRpcResponse_for_RpcLightClientExecutionProofResponse_and_RpcError = ({
    result: RpcLightClientExecutionProofResponse;
} | {
    tmp: RpcError;
} | {
    error: RpcError;
} | {
    id: string;
    jsonrpc: string;
});

