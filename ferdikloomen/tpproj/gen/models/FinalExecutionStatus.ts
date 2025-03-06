/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { TxExecutionError } from './TxExecutionError';
export type FinalExecutionStatus = ('NotStarted' | 'Started' | {
    Failure: TxExecutionError;
} | {
    SuccessValue: string;
});

