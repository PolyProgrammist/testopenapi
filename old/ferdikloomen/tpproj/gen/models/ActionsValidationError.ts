/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { PublicKey } from './PublicKey';
/**
 * Describes the error for validating a list of actions.
 */
export type ActionsValidationError = ('DeleteActionMustBeFinal' | {
    TotalPrepaidGasExceeded: {
        limit: number;
        total_prepaid_gas: number;
    };
} | {
    TotalNumberOfActionsExceeded: {
        limit: number;
        total_number_of_actions: number;
    };
} | {
    AddKeyMethodNamesNumberOfBytesExceeded: {
        limit: number;
        total_number_of_bytes: number;
    };
} | {
    AddKeyMethodNameLengthExceeded: {
        length: number;
        limit: number;
    };
} | 'IntegerOverflow' | {
    InvalidAccountId: {
        account_id: string;
    };
} | {
    ContractSizeExceeded: {
        limit: number;
        size: number;
    };
} | {
    FunctionCallMethodNameLengthExceeded: {
        length: number;
        limit: number;
    };
} | {
    FunctionCallArgumentsLengthExceeded: {
        length: number;
        limit: number;
    };
} | {
    UnsuitableStakingKey: {
        public_key: PublicKey;
    };
} | 'FunctionCallZeroAttachedGas' | 'DelegateActionMustBeOnlyOne' | {
    UnsupportedProtocolFeature: {
        protocol_feature: string;
        version: number;
    };
});

