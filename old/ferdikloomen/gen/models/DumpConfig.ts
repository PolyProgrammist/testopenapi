/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { DurationSchemeProvider } from './DurationSchemeProvider';
import type { ExternalStorageLocation } from './ExternalStorageLocation';
import type { ShardId } from './ShardId';
/**
 * Configures how to dump state to external storage.
 */
export type DumpConfig = {
    /**
     * Location of a json file with credentials allowing write access to the bucket.
     */
    credentials_file?: string | null;
    /**
     * How often to check if a new epoch has started.
     * Feel free to set to `None`, defaults are sensible.
     */
    iteration_delay?: DurationSchemeProvider;
    /**
     * Specifies where to write the obtained state parts.
     */
    location: ExternalStorageLocation;
    /**
     * Use in case a node that dumps state to the external storage
     * gets in trouble.
     */
    restart_dump_for_shards?: Array<ShardId> | null;
};

