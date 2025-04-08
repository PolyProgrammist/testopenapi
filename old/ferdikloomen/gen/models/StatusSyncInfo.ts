/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { CryptoHash } from './CryptoHash';
import type { EpochId } from './EpochId';
export type StatusSyncInfo = {
    earliest_block_hash?: CryptoHash | null;
    earliest_block_height?: number | null;
    earliest_block_time: string;
    epoch_id?: EpochId | null;
    epoch_start_height?: number | null;
    latest_block_hash: CryptoHash;
    latest_block_height: number;
    latest_block_time: string;
    latest_state_root: CryptoHash;
    syncing: boolean;
};

