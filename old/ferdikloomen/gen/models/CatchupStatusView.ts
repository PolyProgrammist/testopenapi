/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { BlockStatusView } from './BlockStatusView';
import type { CryptoHash } from './CryptoHash';
export type CatchupStatusView = {
    blocks_to_catchup: Array<BlockStatusView>;
    shard_sync_status: Record<string, string>;
    sync_block_hash: CryptoHash;
    sync_block_height: number;
};

