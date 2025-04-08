/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { AccountDataView } from './AccountDataView';
import type { KnownProducerView } from './KnownProducerView';
import type { PeerInfoView } from './PeerInfoView';
import type { PublicKey } from './PublicKey';
export type NetworkInfoView = {
    connected_peers: Array<PeerInfoView>;
    known_producers: Array<KnownProducerView>;
    num_connected_peers: number;
    peer_max_count: number;
    tier1_accounts_data: Array<AccountDataView>;
    tier1_accounts_keys: Array<PublicKey>;
    tier1_connections: Array<PeerInfoView>;
};

