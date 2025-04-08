/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { ExtCostsConfigView } from './ExtCostsConfigView';
import type { LimitConfig } from './LimitConfig';
import type { StorageGetMode } from './StorageGetMode';
import type { VMKind } from './VMKind';
export type VMConfigView = {
    /**
     * See [VMConfig::alt_bn128](crate::vm::Config::alt_bn128).
     */
    alt_bn128: boolean;
    /**
     * See [VMConfig::disable_9393_fix](crate::vm::Config::disable_9393_fix).
     */
    disable_9393_fix: boolean;
    /**
     * See [VMConfig::discard_custom_sections](crate::vm::Config::discard_custom_sections).
     */
    discard_custom_sections: boolean;
    /**
     * See [VMConfig::ed25519_verify](crate::vm::Config::ed25519_verify).
     */
    ed25519_verify: boolean;
    /**
     * See [VMConfig::eth_implicit_accounts](crate::vm::Config::eth_implicit_accounts).
     */
    eth_implicit_accounts: boolean;
    /**
     * Costs for runtime externals
     */
    ext_costs: ExtCostsConfigView;
    /**
     * See [VMConfig::fix_contract_loading_cost](crate::vm::Config::fix_contract_loading_cost).
     */
    fix_contract_loading_cost: boolean;
    /**
     * See [VMConfig::function_call_weight](crate::vm::Config::function_call_weight).
     */
    function_call_weight: boolean;
    /**
     * Gas cost of a growing memory by single page.
     */
    grow_mem_cost: number;
    /**
     * See [VMConfig::implicit_account_creation](crate::vm::Config::implicit_account_creation).
     */
    implicit_account_creation: boolean;
    /**
     * Describes limits for VM and Runtime.
     *
     * TODO: Consider changing this to `VMLimitConfigView` to avoid dependency
     * on runtime.
     */
    limit_config: LimitConfig;
    /**
     * See [VMConfig::math_extension](crate::vm::Config::math_extension).
     */
    math_extension: boolean;
    /**
     * Gas cost of a regular operation.
     */
    regular_op_cost: number;
    /**
     * See [VMConfig::storage_get_mode](crate::vm::Config::storage_get_mode).
     */
    storage_get_mode: StorageGetMode;
    /**
     * See [VMConfig::vm_kind](crate::vm::Config::vm_kind).
     */
    vm_kind: VMKind;
    /**
     * See [VMConfig::yield_resume_host_functions](`crate::vm::Config::yield_resume_host_functions).
     */
    yield_resume_host_functions: boolean;
};

