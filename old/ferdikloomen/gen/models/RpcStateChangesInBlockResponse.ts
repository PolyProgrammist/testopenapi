/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { CryptoHash } from './CryptoHash';
import type { StateChangeWithCauseView } from './StateChangeWithCauseView';
export type RpcStateChangesInBlockResponse = {
    block_hash: CryptoHash;
    changes: Array<StateChangeWithCauseView>;
};

