/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { AccountId } from './AccountId';
import type { PublicKey } from './PublicKey';
import type { ShardId } from './ShardId';
export type CurrentEpochValidatorInfo = {
    account_id: AccountId;
    is_slashed: boolean;
    num_expected_blocks: number;
    num_expected_chunks?: number;
    /**
     * Number of chunks this validator was expected to produce in each shard.
     * Each entry in the array corresponds to the shard in the `shards_produced` array.
     */
    num_expected_chunks_per_shard?: Array<number>;
    num_expected_endorsements?: number;
    /**
     * Number of chunks this validator was expected to validate and endorse in each shard.
     * Each entry in the array corresponds to the shard in the `shards_endorsed` array.
     */
    num_expected_endorsements_per_shard?: Array<number>;
    num_produced_blocks: number;
    num_produced_chunks?: number;
    num_produced_chunks_per_shard?: Array<number>;
    num_produced_endorsements?: number;
    num_produced_endorsements_per_shard?: Array<number>;
    public_key: PublicKey;
    /**
     * Shards this validator is assigned to as chunk producer in the current epoch.
     */
    shards: Array<ShardId>;
    /**
     * Shards this validator is assigned to as chunk validator in the current epoch.
     */
    shards_endorsed: Array<ShardId>;
    stake: string;
};

