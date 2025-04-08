/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { BlockHeaderInnerLiteView } from './BlockHeaderInnerLiteView';
import type { CryptoHash } from './CryptoHash';
export type LightClientBlockLiteView = {
    inner_lite: BlockHeaderInnerLiteView;
    inner_rest_hash: CryptoHash;
    prev_block_hash: CryptoHash;
};

