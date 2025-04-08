/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { AccountId } from './AccountId';
import type { ShardId } from './ShardId';
export type ShardLayoutV1 = {
    /**
     * The boundary accounts are the accounts on boundaries between shards.
     * Each shard contains a range of accounts from one boundary account to
     * another - or the smallest or largest account possible. The total
     * number of shards is equal to the number of boundary accounts plus 1.
     */
    boundary_accounts: Array<AccountId>;
    /**
     * Maps shards from the last shard layout to shards that it splits to in this shard layout,
     * Useful for constructing states for the shards.
     * None for the genesis shard layout
     */
    shards_split_map?: Array<Array<ShardId>> | null;
    /**
     * Maps shard in this shard layout to their parent shard
     * Since shard_ids always range from 0 to num_shards - 1, we use vec instead of a hashmap
     */
    to_parent_shard_map?: Array<ShardId> | null;
    /**
     * Version of the shard layout, this is useful for uniquely identify the shard layout
     */
    version: number;
};

