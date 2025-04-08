/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { CryptoHash } from './CryptoHash';
import type { Signature } from './Signature';
import type { SlashedValidator } from './SlashedValidator';
import type { ValidatorStakeView } from './ValidatorStakeView';
export type BlockHeaderView = {
    approvals: Array<Signature>;
    block_body_hash?: CryptoHash | null;
    block_merkle_root: CryptoHash;
    block_ordinal?: number | null;
    challenges_result: Array<SlashedValidator>;
    challenges_root: CryptoHash;
    chunk_endorsements?: Array<Array<number>> | null;
    chunk_headers_root: CryptoHash;
    chunk_mask: Array<boolean>;
    chunk_receipts_root: CryptoHash;
    chunk_tx_root: CryptoHash;
    chunks_included: number;
    epoch_id: CryptoHash;
    epoch_sync_data_hash?: CryptoHash | null;
    gas_price: string;
    hash: CryptoHash;
    height: number;
    last_ds_final_block: CryptoHash;
    last_final_block: CryptoHash;
    latest_protocol_version: number;
    next_bp_hash: CryptoHash;
    next_epoch_id: CryptoHash;
    outcome_root: CryptoHash;
    prev_hash: CryptoHash;
    prev_height?: number | null;
    prev_state_root: CryptoHash;
    random_value: CryptoHash;
    /**
     * TODO(2271): deprecated.
     */
    rent_paid: string;
    signature: Signature;
    /**
     * Legacy json number. Should not be used.
     */
    timestamp: number;
    timestamp_nanosec: string;
    total_supply: string;
    validator_proposals: Array<ValidatorStakeView>;
    /**
     * TODO(2271): deprecated.
     */
    validator_reward: string;
};

