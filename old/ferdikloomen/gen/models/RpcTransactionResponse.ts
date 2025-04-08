/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { FinalExecutionOutcomeView } from './FinalExecutionOutcomeView';
import type { FinalExecutionOutcomeWithReceiptView } from './FinalExecutionOutcomeWithReceiptView';
import type { TxExecutionStatus } from './TxExecutionStatus';
export type RpcTransactionResponse = (FinalExecutionOutcomeWithReceiptView | FinalExecutionOutcomeView | {
    final_execution_status: TxExecutionStatus;
});

