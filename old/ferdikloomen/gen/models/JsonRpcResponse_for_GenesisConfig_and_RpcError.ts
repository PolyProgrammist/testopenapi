/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { GenesisConfig } from './GenesisConfig';
import type { RpcError } from './RpcError';
export type JsonRpcResponse_for_GenesisConfig_and_RpcError = ({
    result: GenesisConfig;
} | {
    tmp: RpcError;
} | {
    error: RpcError;
} | {
    id: string;
    jsonrpc: string;
});

