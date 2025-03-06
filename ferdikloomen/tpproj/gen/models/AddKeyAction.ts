/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { AccessKey } from './AccessKey';
import type { PublicKey } from './PublicKey';
export type AddKeyAction = {
    /**
     * An access key with the permission
     */
    access_key: AccessKey;
    /**
     * A public key which will be associated with an access_key
     */
    public_key: PublicKey;
};

