/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
/**
 * Errors which may occur during working with trie storages, storing trie values (trie nodes and state values) by their hashes.
 */
export type StorageError = ('StorageInternalError' | {
    MissingTrieValue: Array<any>;
} | 'UnexpectedTrieValue' | {
    StorageInconsistentState: string;
} | {
    FlatStorageBlockNotSupported: string;
} | {
    MemTrieLoadingError: string;
} | 'FlatStorageReshardingAlreadyInProgress');

