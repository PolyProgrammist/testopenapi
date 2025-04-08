/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { BlockId } from './BlockId';
import type { CryptoHash } from './CryptoHash';
import type { ShardId } from './ShardId';
export type RpcCongestionLevelRequest = ({
    block_id: BlockId;
    shard_id: ShardId;
} | {
    chunk_id: CryptoHash;
});

