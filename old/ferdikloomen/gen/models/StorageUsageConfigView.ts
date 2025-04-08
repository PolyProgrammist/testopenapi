/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
/**
 * Describes cost of storage per block
 */
export type StorageUsageConfigView = {
    /**
     * Number of bytes for an account record, including rounding up for account id.
     */
    num_bytes_account: number;
    /**
     * Additional number of bytes for a k/v record
     */
    num_extra_bytes_record: number;
};

