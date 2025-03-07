/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { AccountId } from './AccountId';
import type { NonDelegateAction } from './NonDelegateAction';
import type { PublicKey } from './PublicKey';
/**
 * This action allows to execute the inner actions behalf of the defined sender.
 */
export type DelegateAction = {
    /**
     * List of actions to be executed.
     *
     * With the meta transactions MVP defined in NEP-366, nested DelegateActions are not allowed. A separate type is used to enforce it.
     */
    actions: Array<NonDelegateAction>;
    /**
     * The maximal height of the block in the blockchain below which the given DelegateAction is valid.
     */
    max_block_height: number;
    /**
     * Nonce to ensure that the same delegate action is not sent twice by a relayer and should match for given account's `public_key`. After this action is processed it will increment.
     */
    nonce: number;
    /**
     * Public key used to sign this delegated action.
     */
    public_key: PublicKey;
    /**
     * Receiver of the delegated actions.
     */
    receiver_id: AccountId;
    /**
     * Signer of the delegated actions
     */
    sender_id: AccountId;
};

