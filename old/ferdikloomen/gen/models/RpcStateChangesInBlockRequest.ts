/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { BlockId } from './BlockId';
import type { Finality } from './Finality';
import type { SyncCheckpoint } from './SyncCheckpoint';
export type RpcStateChangesInBlockRequest = ({
    block_id: BlockId;
} | {
    finality: Finality;
} | {
    sync_checkpoint: SyncCheckpoint;
});

