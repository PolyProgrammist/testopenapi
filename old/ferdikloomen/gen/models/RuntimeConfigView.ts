/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { AccountCreationConfigView } from './AccountCreationConfigView';
import type { CongestionControlConfigView } from './CongestionControlConfigView';
import type { RuntimeFeesConfigView } from './RuntimeFeesConfigView';
import type { VMConfigView } from './VMConfigView';
import type { WitnessConfigView } from './WitnessConfigView';
/**
 * View that preserves JSON format of the runtime config.
 */
export type RuntimeConfigView = {
    /**
     * Config that defines rules for account creation.
     */
    account_creation_config: AccountCreationConfigView;
    /**
     * The configuration for congestion control.
     */
    congestion_control_config: CongestionControlConfigView;
    /**
     * Amount of yN per byte required to have on the account.  See
     * <https://nomicon.io/Economics/Economic#state-stake> for details.
     */
    storage_amount_per_byte: string;
    /**
     * Costs of different actions that need to be performed when sending and
     * processing transaction and receipts.
     */
    transaction_costs: RuntimeFeesConfigView;
    /**
     * Config of wasm operations.
     */
    wasm_config: VMConfigView;
    /**
     * Configuration specific to ChunkStateWitness.
     */
    witness_config: WitnessConfigView;
};

