/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { AccountId } from './AccountId';
import type { CryptoHash } from './CryptoHash';
import type { ReceiptEnumView } from './ReceiptEnumView';
export type ReceiptView = {
    predecessor_id: AccountId;
    priority?: number;
    receipt: ReceiptEnumView;
    receipt_id: CryptoHash;
    receiver_id: AccountId;
};

