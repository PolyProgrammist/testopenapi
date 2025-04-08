/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { CryptoHash } from './CryptoHash';
/**
 * See crate::types::StateChangeCause for details.
 */
export type StateChangeCauseView = ({
    type: string;
} | {
    tx_hash: CryptoHash;
    type: string;
} | {
    receipt_hash: CryptoHash;
    type: string;
});

