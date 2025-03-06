/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { AccountId } from './AccountId';
import type { ActionView } from './ActionView';
import type { CryptoHash } from './CryptoHash';
import type { PublicKey } from './PublicKey';
import type { Signature } from './Signature';
export type SignedTransactionView = {
    actions: Array<ActionView>;
    hash: CryptoHash;
    nonce: number;
    priority_fee?: number;
    public_key: PublicKey;
    receiver_id: AccountId;
    signature: Signature;
    signer_id: AccountId;
};

