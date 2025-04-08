/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { AccountId } from './AccountId';
import type { PublicKey } from './PublicKey';
/**
 * Information about a Producer: its account name, peer_id and a list of connected peers that
 * the node can use to send message for this producer.
 */
export type KnownProducerView = {
    account_id: AccountId;
    next_hops?: Array<PublicKey> | null;
    peer_id: PublicKey;
};

