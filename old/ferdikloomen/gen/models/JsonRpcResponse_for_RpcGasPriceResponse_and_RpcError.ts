/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { RpcError } from './RpcError';
import type { RpcGasPriceResponse } from './RpcGasPriceResponse';
export type JsonRpcResponse_for_RpcGasPriceResponse_and_RpcError = ({
    result: RpcGasPriceResponse;
} | {
    tmp: RpcError;
} | {
    error: RpcError;
} | {
    id: string;
    jsonrpc: string;
});

