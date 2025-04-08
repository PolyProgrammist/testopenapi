/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { ExecutionOutcomeWithIdView } from './ExecutionOutcomeWithIdView';
import type { LightClientBlockLiteView } from './LightClientBlockLiteView';
import type { MerklePathItem } from './MerklePathItem';
export type RpcLightClientExecutionProofResponse = {
    block_header_lite: LightClientBlockLiteView;
    block_proof: Array<MerklePathItem>;
    outcome_proof: ExecutionOutcomeWithIdView;
    outcome_root_proof: Array<MerklePathItem>;
};

