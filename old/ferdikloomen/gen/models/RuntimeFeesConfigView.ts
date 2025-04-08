/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { ActionCreationConfigView } from './ActionCreationConfigView';
import type { DataReceiptCreationConfigView } from './DataReceiptCreationConfigView';
import type { Fee } from './Fee';
import type { Rational32SchemaProvider } from './Rational32SchemaProvider';
import type { StorageUsageConfigView } from './StorageUsageConfigView';
export type RuntimeFeesConfigView = {
    /**
     * Describes the cost of creating a certain action, `Action`. Includes all variants.
     */
    action_creation_config: ActionCreationConfigView;
    /**
     * Describes the cost of creating an action receipt, `ActionReceipt`, excluding the actual cost
     * of actions.
     * - `send` cost is burned when a receipt is created using `promise_create` or
     * `promise_batch_create`
     * - `exec` cost is burned when the receipt is being executed.
     */
    action_receipt_creation_config: Fee;
    /**
     * Fraction of the burnt gas to reward to the contract account for execution.
     */
    burnt_gas_reward: Rational32SchemaProvider;
    /**
     * Describes the cost of creating a data receipt, `DataReceipt`.
     */
    data_receipt_creation_config: DataReceiptCreationConfigView;
    /**
     * Pessimistic gas price inflation ratio.
     */
    pessimistic_gas_price_inflation_ratio: Rational32SchemaProvider;
    /**
     * Describes fees for storage.
     */
    storage_usage_config: StorageUsageConfigView;
};

