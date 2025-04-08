/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { LightClientBlockLiteView } from './LightClientBlockLiteView';
import type { MerklePathItem } from './MerklePathItem';
export type RpcLightClientBlockProofResponse = {
    block_header_lite: LightClientBlockLiteView;
    block_proof: Array<MerklePathItem>;
};

