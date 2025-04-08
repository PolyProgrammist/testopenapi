/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { RpcError } from './RpcError';
import type { RpcLightClientNextBlockResponse } from './RpcLightClientNextBlockResponse';
export type JsonRpcResponse_for_RpcLightClientNextBlockResponse_and_RpcError = ({
    result: RpcLightClientNextBlockResponse;
} | {
    tmp: RpcError;
} | {
    error: RpcError;
} | {
    id: string;
    jsonrpc: string;
});

