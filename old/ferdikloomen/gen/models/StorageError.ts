/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { MissingTrieValue } from './MissingTrieValue';
/**
 * Errors which may occur during working with trie storages, storing
 * trie values (trie nodes and state values) by their hashes.
 */
export type StorageError = (string | {
    MissingTrieValue: MissingTrieValue;
} | {
    StorageInconsistentState: string;
} | {
    FlatStorageBlockNotSupported: string;
} | {
    MemTrieLoadingError: string;
});

