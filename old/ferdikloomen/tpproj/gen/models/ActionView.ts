/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { AccessKeyView } from './AccessKeyView';
import type { AccountId } from './AccountId';
import type { CryptoHash } from './CryptoHash';
import type { DelegateAction } from './DelegateAction';
import type { PublicKey } from './PublicKey';
import type { Signature } from './Signature';
export type ActionView = ('CreateAccount' | {
    DeployContract: {
        code: string;
    };
} | {
    FunctionCall: {
        args: string;
        deposit: string;
        gas: number;
        method_name: string;
    };
} | {
    Transfer: {
        deposit: string;
    };
} | {
    Stake: {
        public_key: PublicKey;
        stake: string;
    };
} | {
    AddKey: {
        access_key: AccessKeyView;
        public_key: PublicKey;
    };
} | {
    DeleteKey: {
        public_key: PublicKey;
    };
} | {
    DeleteAccount: {
        beneficiary_id: AccountId;
    };
} | {
    Delegate: {
        delegate_action: DelegateAction;
        signature: Signature;
    };
} | {
    DeployGlobalContract: {
        code: string;
    };
} | {
    DeployGlobalContractByAccountId: {
        code: string;
    };
} | {
    UseGlobalContract: {
        code_hash: CryptoHash;
    };
} | {
    UseGlobalContractByAccountId: {
        account_id: AccountId;
    };
});

