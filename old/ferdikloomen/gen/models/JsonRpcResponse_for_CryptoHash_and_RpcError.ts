/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { CryptoHash } from './CryptoHash';
import type { RpcError } from './RpcError';
export type JsonRpcResponse_for_CryptoHash_and_RpcError = ({
    result: CryptoHash;
} | {
    tmp: RpcError;
} | {
    error: RpcError;
} | {
    id: string;
    jsonrpc: string;
});

