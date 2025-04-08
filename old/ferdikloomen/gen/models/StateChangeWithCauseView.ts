/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { AccessKeyView } from './AccessKeyView';
import type { AccountId } from './AccountId';
import type { CryptoHash } from './CryptoHash';
import type { PublicKey } from './PublicKey';
import type { StateChangeCauseView } from './StateChangeCauseView';
export type StateChangeWithCauseView = ({
    /**
     * A view of the account
     */
    change: {
        account_id: AccountId;
        amount: string;
        code_hash: CryptoHash;
        global_contract_account_id?: AccountId | null;
        global_contract_hash?: CryptoHash | null;
        locked: string;
        /**
         * TODO(2271): deprecated.
         */
        storage_paid_at?: number;
        storage_usage: number;
    };
    type: string;
} | {
    change: {
        account_id: AccountId;
    };
    type: string;
} | {
    change: {
        access_key: AccessKeyView;
        account_id: AccountId;
        public_key: PublicKey;
    };
    type: string;
} | {
    change: {
        account_id: AccountId;
        public_key: PublicKey;
    };
    type: string;
} | {
    change: {
        account_id: AccountId;
        key_base64: string;
        value_base64: string;
    };
    type: string;
} | {
    change: {
        account_id: AccountId;
        key_base64: string;
    };
    type: string;
} | {
    change: {
        account_id: AccountId;
        code_base64: string;
    };
    type: string;
} | {
    cause: StateChangeCauseView;
});

