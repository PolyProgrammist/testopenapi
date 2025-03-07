/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
export type HostError = ('BadUTF16' | 'BadUTF8' | 'GasExceeded' | 'GasLimitExceeded' | 'BalanceExceeded' | 'EmptyMethodName' | {
    GuestPanic: {
        panic_msg: string;
    };
} | 'IntegerOverflow' | {
    InvalidPromiseIndex: {
        promise_idx: number;
    };
} | 'CannotAppendActionToJointPromise' | 'CannotReturnJointPromise' | {
    InvalidPromiseResultIndex: {
        result_idx: number;
    };
} | {
    InvalidRegisterId: {
        register_id: number;
    };
} | {
    IteratorWasInvalidated: {
        iterator_index: number;
    };
} | 'MemoryAccessViolation' | {
    InvalidReceiptIndex: {
        receipt_index: number;
    };
} | {
    InvalidIteratorIndex: {
        iterator_index: number;
    };
} | 'InvalidAccountId' | 'InvalidMethodName' | 'InvalidPublicKey' | {
    ProhibitedInView: {
        method_name: string;
    };
} | {
    NumberOfLogsExceeded: {
        limit: number;
    };
} | {
    KeyLengthExceeded: {
        length: number;
        limit: number;
    };
} | {
    ValueLengthExceeded: {
        length: number;
        limit: number;
    };
} | {
    TotalLogLengthExceeded: {
        length: number;
        limit: number;
    };
} | {
    NumberPromisesExceeded: {
        limit: number;
        number_of_promises: number;
    };
} | {
    NumberInputDataDependenciesExceeded: {
        limit: number;
        number_of_input_data_dependencies: number;
    };
} | {
    ReturnedValueLengthExceeded: {
        length: number;
        limit: number;
    };
} | {
    ContractSizeExceeded: {
        limit: number;
        size: number;
    };
} | {
    Deprecated: {
        method_name: string;
    };
} | {
    ECRecoverError: {
        msg: string;
    };
} | {
    AltBn128InvalidInput: {
        msg: string;
    };
} | {
    Ed25519VerifyInvalidInput: {
        msg: string;
    };
});

