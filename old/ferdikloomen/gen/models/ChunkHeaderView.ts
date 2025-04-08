/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { BandwidthRequests } from './BandwidthRequests';
import type { CongestionInfoView } from './CongestionInfoView';
import type { CryptoHash } from './CryptoHash';
import type { ShardId } from './ShardId';
import type { Signature } from './Signature';
import type { ValidatorStakeView } from './ValidatorStakeView';
export type ChunkHeaderView = {
    balance_burnt: string;
    bandwidth_requests?: BandwidthRequests | null;
    chunk_hash: CryptoHash;
    congestion_info?: CongestionInfoView | null;
    encoded_length: number;
    encoded_merkle_root: CryptoHash;
    gas_limit: number;
    gas_used: number;
    height_created: number;
    height_included: number;
    outcome_root: CryptoHash;
    outgoing_receipts_root: CryptoHash;
    prev_block_hash: CryptoHash;
    prev_state_root: CryptoHash;
    /**
     * TODO(2271): deprecated.
     */
    rent_paid: string;
    shard_id: ShardId;
    signature: Signature;
    tx_root: CryptoHash;
    validator_proposals: Array<ValidatorStakeView>;
    /**
     * TODO(2271): deprecated.
     */
    validator_reward: string;
};

