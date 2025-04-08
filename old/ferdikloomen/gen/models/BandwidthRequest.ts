/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { BandwidthRequestBitmap } from './BandwidthRequestBitmap';
/**
 * `BandwidthRequest` describes the size of receipts that a shard would like to send to another shard.
 * When a shard wants to send a lot of receipts to another shard, it needs to create a request and wait
 * for a bandwidth grant from the bandwidth scheduler.
 */
export type BandwidthRequest = {
    /**
     * Bitmap which describes what values of bandwidth are requested.
     */
    requested_values_bitmap: BandwidthRequestBitmap;
    /**
     * Requesting bandwidth to this shard.
     */
    to_shard: number;
};

