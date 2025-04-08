/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { BlockId } from './BlockId';
import type { EpochId } from './EpochId';
export type RpcValidatorRequest = ('latest' | {
    epoch_id: EpochId;
} | {
    block_id: BlockId;
});

