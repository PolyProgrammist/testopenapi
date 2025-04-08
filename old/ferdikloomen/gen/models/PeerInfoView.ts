/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { AccountId } from './AccountId';
import type { CryptoHash } from './CryptoHash';
import type { PublicKey } from './PublicKey';
import type { ShardId } from './ShardId';
export type PeerInfoView = {
    account_id?: AccountId | null;
    addr: string;
    archival: boolean;
    block_hash?: CryptoHash | null;
    connection_established_time_millis: number;
    height?: number | null;
    is_highest_block_invalid: boolean;
    is_outbound_peer: boolean;
    last_time_peer_requested_millis: number;
    last_time_received_message_millis: number;
    /**
     * Connection nonce.
     */
    nonce: number;
    peer_id: PublicKey;
    received_bytes_per_sec: number;
    sent_bytes_per_sec: number;
    tracked_shards: Array<ShardId>;
};

