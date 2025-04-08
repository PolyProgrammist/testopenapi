/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { RpcError } from './RpcError';
export type JsonRpcResponse_for_Array_of_Tuple_of_uint64_and_uint64_and_RpcError = ({
    result: Array<Array<any>>;
} | {
    tmp: RpcError;
} | {
    error: RpcError;
} | {
    id: string;
    jsonrpc: string;
});

