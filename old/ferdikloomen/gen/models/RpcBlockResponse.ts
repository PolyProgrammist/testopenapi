/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { AccountId } from './AccountId';
import type { BlockHeaderView } from './BlockHeaderView';
import type { ChunkHeaderView } from './ChunkHeaderView';
export type RpcBlockResponse = {
    author: AccountId;
    chunks: Array<ChunkHeaderView>;
    header: BlockHeaderView;
};

