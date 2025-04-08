/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { CryptoHash } from './CryptoHash';
import type { ExecutionOutcomeView } from './ExecutionOutcomeView';
import type { MerklePathItem } from './MerklePathItem';
export type ExecutionOutcomeWithIdView = {
    block_hash: CryptoHash;
    id: CryptoHash;
    outcome: ExecutionOutcomeView;
    proof: Array<MerklePathItem>;
};

