/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { AccountId } from './AccountId';
import type { PublicKey } from './PublicKey';
import type { ShardId } from './ShardId';
export type NextEpochValidatorInfo = {
    account_id: AccountId;
    public_key: PublicKey;
    shards: Array<ShardId>;
    stake: string;
};

