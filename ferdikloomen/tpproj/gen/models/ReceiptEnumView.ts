/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { AccountId } from './AccountId';
import type { ActionView } from './ActionView';
import type { CryptoHash } from './CryptoHash';
import type { DataReceiverView } from './DataReceiverView';
import type { GlobalContractData } from './GlobalContractData';
import type { PublicKey } from './PublicKey';
export type ReceiptEnumView = ({
    Action: {
        actions: Array<ActionView>;
        gas_price: string;
        input_data_ids: Array<CryptoHash>;
        is_promise_yield?: boolean;
        output_data_receivers: Array<DataReceiverView>;
        signer_id: AccountId;
        signer_public_key: PublicKey;
    };
} | {
    Data: {
        data?: string | null;
        data_id: CryptoHash;
        is_promise_resume?: boolean;
    };
} | {
    GlobalContractDistribution: {
        data: GlobalContractData;
    };
});

