/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
export type HostError = (string | {
    GuestPanic: {
        panic_msg: string;
    };
} | {
    InvalidPromiseIndex: {
        promise_idx: number;
    };
} | {
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
} | {
    InvalidReceiptIndex: {
        receipt_index: number;
    };
} | {
    InvalidIteratorIndex: {
        iterator_index: number;
    };
} | {
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

