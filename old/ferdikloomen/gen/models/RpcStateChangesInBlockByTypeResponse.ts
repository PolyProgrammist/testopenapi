/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { CryptoHash } from './CryptoHash';
import type { StateChangeKindView } from './StateChangeKindView';
export type RpcStateChangesInBlockByTypeResponse = {
    block_hash: CryptoHash;
    changes: Array<StateChangeKindView>;
};

