/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { PublicKey } from './PublicKey';
/**
 * An action which stakes signer_id tokens and setup's validator public key
 */
export type StakeAction = {
    /**
     * Validator key which will be used to sign transactions on behalf of signer_id
     */
    public_key: PublicKey;
    /**
     * Amount of tokens to stake.
     */
    stake: string;
};

