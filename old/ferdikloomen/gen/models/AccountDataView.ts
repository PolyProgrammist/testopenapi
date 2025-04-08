/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { PublicKey } from './PublicKey';
import type { Tier1ProxyView } from './Tier1ProxyView';
export type AccountDataView = {
    account_key: PublicKey;
    peer_id: PublicKey;
    proxies: Array<Tier1ProxyView>;
    timestamp: string;
};

