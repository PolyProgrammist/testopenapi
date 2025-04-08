/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { RpcKnownProducer } from './RpcKnownProducer';
import type { RpcPeerInfo } from './RpcPeerInfo';
export type RpcNetworkInfoResponse = {
    active_peers: Array<RpcPeerInfo>;
    /**
     * Accounts of known block and chunk producers from routing table.
     */
    known_producers: Array<RpcKnownProducer>;
    num_active_peers: number;
    peer_max_count: number;
    received_bytes_per_sec: number;
    sent_bytes_per_sec: number;
};

