/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { AccountId } from './AccountId';
/**
 * The structure describes configuration for creation of new accounts.
 */
export type AccountCreationConfigView = {
    /**
     * The minimum length of the top-level account ID that is allowed to be created by any account.
     */
    min_allowed_top_level_account_length: number;
    /**
     * The account ID of the account registrar. This account ID allowed to create top-level
     * accounts of any valid length.
     */
    registrar_account_id: AccountId;
};

