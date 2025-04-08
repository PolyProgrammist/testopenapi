/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { AccessKeyCreationConfigView } from './AccessKeyCreationConfigView';
import type { Fee } from './Fee';
/**
 * Describes the cost of creating a specific action, `Action`. Includes all variants.
 */
export type ActionCreationConfigView = {
    /**
     * Base cost of adding a key.
     */
    add_key_cost: AccessKeyCreationConfigView;
    /**
     * Base cost of creating an account.
     */
    create_account_cost: Fee;
    /**
     * Base cost for processing a delegate action.
     *
     * This is on top of the costs for the actions inside the delegate action.
     */
    delegate_cost: Fee;
    /**
     * Base cost of deleting an account.
     */
    delete_account_cost: Fee;
    /**
     * Base cost of deleting a key.
     */
    delete_key_cost: Fee;
    /**
     * Base cost of deploying a contract.
     */
    deploy_contract_cost: Fee;
    /**
     * Cost per byte of deploying a contract.
     */
    deploy_contract_cost_per_byte: Fee;
    /**
     * Base cost of calling a function.
     */
    function_call_cost: Fee;
    /**
     * Cost per byte of method name and arguments of calling a function.
     */
    function_call_cost_per_byte: Fee;
    /**
     * Base cost of staking.
     */
    stake_cost: Fee;
    /**
     * Base cost of making a transfer.
     */
    transfer_cost: Fee;
};

