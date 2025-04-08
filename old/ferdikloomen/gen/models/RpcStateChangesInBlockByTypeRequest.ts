/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { AccountId } from './AccountId';
import type { AccountWithPublicKey } from './AccountWithPublicKey';
import type { BlockId } from './BlockId';
import type { Finality } from './Finality';
import type { SyncCheckpoint } from './SyncCheckpoint';
/**
 * It is a [serializable view] of [`StateChangesRequest`].
 *
 * [serializable view]: ./index.html
 * [`StateChangesRequest`]: ../types/struct.StateChangesRequest.html
 */
export type RpcStateChangesInBlockByTypeRequest = (({
    block_id: BlockId;
} | {
    finality: Finality;
} | {
    sync_checkpoint: SyncCheckpoint;
}) & ({
    account_ids: Array<AccountId>;
    changes_type: string;
} | {
    changes_type: string;
    keys: Array<AccountWithPublicKey>;
} | {
    account_ids: Array<AccountId>;
    changes_type: string;
    key_prefix_base64: string;
}));

