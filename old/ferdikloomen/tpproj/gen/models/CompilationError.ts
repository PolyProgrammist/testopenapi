/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { AccountId } from './AccountId';
import type { PrepareError } from './PrepareError';
export type CompilationError = ({
    CodeDoesNotExist: {
        account_id: AccountId;
    };
} | {
    PrepareError: PrepareError;
} | {
    WasmerCompileError: {
        msg: string;
    };
});

