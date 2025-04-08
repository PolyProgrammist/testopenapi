/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
/**
 * A shard layout that maps accounts evenly across all shards -- by calculate the hash of account
 * id and mod number of shards. This is added to capture the old `account_id_to_shard_id` algorithm,
 * to keep backward compatibility for some existing tests.
 * `parent_shards` for `ShardLayoutV1` is always `None`, meaning it can only be the first shard layout
 * a chain uses.
 */
export type ShardLayoutV0 = {
    /**
     * Map accounts evenly across all shards
     */
    num_shards: number;
    /**
     * Version of the shard layout, this is useful for uniquely identify the shard layout
     */
    version: number;
};

