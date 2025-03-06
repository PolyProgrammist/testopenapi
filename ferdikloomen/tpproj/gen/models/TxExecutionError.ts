/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { ActionError } from './ActionError';
import type { InvalidTxError } from './InvalidTxError';
/**
 * Error returned in the ExecutionOutcome in case of failure
 */
export type TxExecutionError = ({
    ActionError: ActionError;
} | {
    InvalidTxError: InvalidTxError;
});

