/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { AddKeyAction } from './AddKeyAction';
import type { CreateAccountAction } from './CreateAccountAction';
import type { DeleteAccountAction } from './DeleteAccountAction';
import type { DeleteKeyAction } from './DeleteKeyAction';
import type { DeployContractAction } from './DeployContractAction';
import type { DeployGlobalContractAction } from './DeployGlobalContractAction';
import type { FunctionCallAction } from './FunctionCallAction';
import type { SignedDelegateAction } from './SignedDelegateAction';
import type { StakeAction } from './StakeAction';
import type { TransferAction } from './TransferAction';
import type { UseGlobalContractAction } from './UseGlobalContractAction';
export type Action = ({
    CreateAccount: CreateAccountAction;
} | {
    DeployContract: DeployContractAction;
} | {
    FunctionCall: FunctionCallAction;
} | {
    Transfer: TransferAction;
} | {
    Stake: StakeAction;
} | {
    AddKey: AddKeyAction;
} | {
    DeleteKey: DeleteKeyAction;
} | {
    DeleteAccount: DeleteAccountAction;
} | {
    Delegate: SignedDelegateAction;
} | {
    DeployGlobalContract: DeployGlobalContractAction;
} | {
    UseGlobalContract: UseGlobalContractAction;
});

