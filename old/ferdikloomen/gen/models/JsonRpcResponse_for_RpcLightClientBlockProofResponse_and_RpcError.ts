/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { RpcError } from './RpcError';
import type { RpcLightClientBlockProofResponse } from './RpcLightClientBlockProofResponse';
export type JsonRpcResponse_for_RpcLightClientBlockProofResponse_and_RpcError = ({
    result: RpcLightClientBlockProofResponse;
} | {
    tmp: RpcError;
} | {
    error: RpcError;
} | {
    id: string;
    jsonrpc: string;
});

