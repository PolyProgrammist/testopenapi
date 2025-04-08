/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
/**
 * Describes limits for VM and Runtime.
 * TODO #4139: consider switching to strongly-typed wrappers instead of raw quantities
 */
export type LimitConfig = {
    /**
     * Whether to enforce account_id well-formed-ness where it wasn't enforced
     * historically.
     */
    account_id_validity_rules_version?: number;
    /**
     * Whether a legacy version of stack limiting should be used, see
     * [`ContractPrepareVersion`].
     */
    contract_prepare_version?: number;
    /**
     * The initial number of memory pages.
     * NOTE: It's not a limiter itself, but it's a value we use for initial_memory_pages.
     */
    initial_memory_pages: number;
    /**
     * Max number of actions per receipt.
     */
    max_actions_per_receipt: number;
    /**
     * Max length of arguments in a function call action.
     */
    max_arguments_length: number;
    /**
     * Max contract size
     */
    max_contract_size: number;
    /**
     * If present, stores max number of functions in one contract
     */
    max_functions_number_per_contract?: number | null;
    /**
     * Max amount of gas that can be used, excluding gas attached to promises.
     */
    max_gas_burnt: number;
    /**
     * Max length of any method name (without terminating character).
     */
    max_length_method_name: number;
    /**
     * Max length of returned data
     */
    max_length_returned_data: number;
    /**
     * Max storage key size
     */
    max_length_storage_key: number;
    /**
     * Max storage value size
     */
    max_length_storage_value: number;
    /**
     * If present, stores max number of locals declared globally in one contract
     */
    max_locals_per_contract?: number | null;
    /**
     * What is the maximal memory pages amount is allowed to have for a contract.
     */
    max_memory_pages: number;
    /**
     * Max total length of all method names (including terminating character) for a function call
     * permission access key.
     */
    max_number_bytes_method_names: number;
    /**
     * Max number of input data dependencies
     */
    max_number_input_data_dependencies: number;
    /**
     * Maximum number of log entries.
     */
    max_number_logs: number;
    /**
     * Maximum number of registers that can be used simultaneously.
     *
     * Note that due to an implementation quirk [read: a bug] in VMLogic, if we
     * have this number of registers, no subsequent writes to the registers
     * will succeed even if they replace an existing register.
     */
    max_number_registers: number;
    /**
     * Max number of promises that a function call can create
     */
    max_promises_per_function_call_action: number;
    /**
     * Max receipt size
     */
    max_receipt_size: number;
    /**
     * Maximum number of bytes that can be stored in a single register.
     */
    max_register_size: number;
    /**
     * How tall the stack is allowed to grow?
     *
     * See <https://wiki.parity.io/WebAssembly-StackHeight> to find out how the stack frame cost
     * is calculated.
     */
    max_stack_height: number;
    /**
     * Maximum total length in bytes of all log messages.
     */
    max_total_log_length: number;
    /**
     * Max total prepaid gas for all function call actions per receipt.
     */
    max_total_prepaid_gas: number;
    /**
     * Max transaction size
     */
    max_transaction_size: number;
    /**
     * Maximum number of bytes for payload passed over a yield resume.
     */
    max_yield_payload_size: number;
    /**
     * Hard limit on the size of storage proof generated while executing a single receipt.
     */
    per_receipt_storage_proof_size_limit: number;
    /**
     * Limit of memory used by registers.
     */
    registers_memory_limit: number;
    /**
     * If present, stores the secondary stack limit as implemented by wasmer2.
     *
     * This limit should never be hit normally.
     */
    wasmer2_stack_limit?: number;
    /**
     * Number of blocks after which a yielded promise times out.
     */
    yield_timeout_length_in_blocks: number;
};

