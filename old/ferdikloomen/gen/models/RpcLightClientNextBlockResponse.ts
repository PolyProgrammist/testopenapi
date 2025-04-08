/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { BlockHeaderInnerLiteView } from './BlockHeaderInnerLiteView';
import type { CryptoHash } from './CryptoHash';
import type { Signature } from './Signature';
import type { ValidatorStakeView } from './ValidatorStakeView';
export type RpcLightClientNextBlockResponse = {
    approvals_after_next?: Array<Signature>;
    inner_lite?: BlockHeaderInnerLiteView;
    inner_rest_hash?: CryptoHash;
    next_block_inner_hash?: CryptoHash;
    next_bps?: Array<ValidatorStakeView> | null;
    prev_block_hash?: CryptoHash;
};

