/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { DurationSchemeProvider } from './DurationSchemeProvider';
/**
 * Configuration for garbage collection.
 */
export type GCConfig = {
    /**
     * Maximum number of blocks to garbage collect at every garbage collection
     * call.
     */
    gc_blocks_limit?: number;
    /**
     * Maximum number of height to go through at each garbage collection step
     * when cleaning forks during garbage collection.
     */
    gc_fork_clean_step?: number;
    /**
     * Number of epochs for which we keep store data.
     */
    gc_num_epochs_to_keep?: number;
    /**
     * How often gc should be run
     */
    gc_step_period?: DurationSchemeProvider;
};

