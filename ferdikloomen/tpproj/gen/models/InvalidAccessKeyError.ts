/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { AccountId } from './AccountId';
import type { PublicKey } from './PublicKey';
export type InvalidAccessKeyError = ({
    AccessKeyNotFound: {
        account_id: AccountId;
        public_key: PublicKey;
    };
} | {
    ReceiverMismatch: {
        ak_receiver: string;
        tx_receiver: AccountId;
    };
} | {
    MethodNameMismatch: {
        method_name: string;
    };
} | 'RequiresFullAccess' | {
    NotEnoughAllowance: {
        account_id: AccountId;
        allowance: string;
        cost: string;
        public_key: PublicKey;
    };
} | 'DepositWithFunctionCall');

