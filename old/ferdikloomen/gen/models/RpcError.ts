/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { CauseRpcErrorKind } from './CauseRpcErrorKind';
import type { NameRpcErrorKind } from './NameRpcErrorKind';
export type RpcError = {
    cause?: CauseRpcErrorKind | null;
    code: number;
    data?: any;
    message: string;
    name?: NameRpcErrorKind | null;
};

