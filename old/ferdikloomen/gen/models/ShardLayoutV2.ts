/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { AccountId } from './AccountId';
import type { ShardId } from './ShardId';
/**
 * Counterpart to `ShardLayoutV2` composed of maps with string keys to aid
 * serde serialization.
 */
export type ShardLayoutV2 = {
    boundary_accounts: Array<AccountId>;
    id_to_index_map: Record<string, number>;
    index_to_id_map: Record<string, ShardId>;
    shard_ids: Array<ShardId>;
    shards_parent_map?: Record<string, ShardId> | null;
    shards_split_map?: Record<string, Array<ShardId>> | null;
    version: number;
};

