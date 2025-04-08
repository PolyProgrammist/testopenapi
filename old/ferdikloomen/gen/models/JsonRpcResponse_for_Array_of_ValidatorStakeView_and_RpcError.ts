/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { RpcError } from './RpcError';
import type { ValidatorStakeView } from './ValidatorStakeView';
export type JsonRpcResponse_for_Array_of_ValidatorStakeView_and_RpcError = ({
    result: Array<ValidatorStakeView>;
} | {
    tmp: RpcError;
} | {
    error: RpcError;
} | {
    id: string;
    jsonrpc: string;
});

