/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { CryptoHash } from './CryptoHash';
export type BlockHeaderInnerLiteView = {
    block_merkle_root: CryptoHash;
    epoch_id: CryptoHash;
    height: number;
    next_bp_hash: CryptoHash;
    next_epoch_id: CryptoHash;
    outcome_root: CryptoHash;
    prev_state_root: CryptoHash;
    /**
     * Legacy json number. Should not be used.
     */
    timestamp: number;
    timestamp_nanosec: string;
};

