/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { ChunkDistributionUris } from './ChunkDistributionUris';
/**
 * Config for the Chunk Distribution Network feature.
 * This allows nodes to push and pull chunks from a central stream.
 * The two benefits of this approach are: (1) less request/response traffic
 * on the peer-to-peer network and (2) lower latency for RPC nodes indexing the chain.
 */
export type ChunkDistributionNetworkConfig = {
    enabled: boolean;
    uris: ChunkDistributionUris;
};

