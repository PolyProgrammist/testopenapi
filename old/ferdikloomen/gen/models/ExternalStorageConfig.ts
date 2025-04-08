/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { ExternalStorageLocation } from './ExternalStorageLocation';
export type ExternalStorageConfig = {
    /**
     * The number of attempts the node will make to obtain a part from peers in
     * the network before it fetches from external storage.
     */
    external_storage_fallback_threshold?: number;
    /**
     * Location of state parts.
     */
    location: ExternalStorageLocation;
    /**
     * When fetching state parts from external storage, throttle fetch requests
     * to this many concurrent requests.
     */
    num_concurrent_requests?: number;
    /**
     * During catchup, the node will use a different number of concurrent requests
     * to reduce the performance impact of state sync.
     */
    num_concurrent_requests_during_catchup?: number;
};

