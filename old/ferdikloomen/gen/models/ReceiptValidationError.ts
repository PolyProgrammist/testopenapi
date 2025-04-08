/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { ActionsValidationError } from './ActionsValidationError';
/**
 * Describes the error for validating a receipt.
 */
export type ReceiptValidationError = ({
    InvalidPredecessorId: {
        account_id: string;
    };
} | {
    InvalidReceiverId: {
        account_id: string;
    };
} | {
    InvalidSignerId: {
        account_id: string;
    };
} | {
    InvalidDataReceiverId: {
        account_id: string;
    };
} | {
    ReturnedValueLengthExceeded: {
        length: number;
        limit: number;
    };
} | {
    NumberInputDataDependenciesExceeded: {
        limit: number;
        number_of_input_data_dependencies: number;
    };
} | {
    ActionsValidation: ActionsValidationError;
} | {
    ReceiptSizeExceeded: {
        limit: number;
        size: number;
    };
});

