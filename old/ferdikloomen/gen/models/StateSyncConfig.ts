/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { DumpConfig } from './DumpConfig';
import type { SyncConfig } from './SyncConfig';
/**
 * Options for dumping state to S3.
 */
export type StateSyncConfig = {
    /**
     * `none` value disables state dump to external storage.
     */
    dump?: DumpConfig | null;
    sync?: SyncConfig;
};

