/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { CryptoHash } from './CryptoHash';
/**
 * Epoch identifier -- wrapped hash, to make it easier to distinguish.
 * EpochId of epoch T is the hash of last block in T-2
 * EpochId of first two epochs is 0
 */
export type EpochId = CryptoHash;
