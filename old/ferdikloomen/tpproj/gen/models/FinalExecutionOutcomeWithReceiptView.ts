/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { ExecutionOutcomeWithIdView } from './ExecutionOutcomeWithIdView';
import type { FinalExecutionStatus } from './FinalExecutionStatus';
import type { ReceiptView } from './ReceiptView';
import type { SignedTransactionView } from './SignedTransactionView';
/**
 * Final execution outcome of the transaction and all of subsequent the receipts. Also includes the generated receipt.
 */
export type FinalExecutionOutcomeWithReceiptView = {
    /**
     * Receipts generated from the transaction
     */
    receipts: Array<ReceiptView>;
    /**
     * The execution outcome of receipts.
     */
    receipts_outcome: Array<ExecutionOutcomeWithIdView>;
    /**
     * Execution status defined by chain.rs:get_final_transaction_result FinalExecutionStatus::NotStarted - the tx is not converted to the receipt yet FinalExecutionStatus::Started - we have at least 1 receipt, but the first leaf receipt_id (using dfs) hasn't finished the execution FinalExecutionStatus::Failure - the result of the first leaf receipt_id FinalExecutionStatus::SuccessValue - the result of the first leaf receipt_id
     */
    status: FinalExecutionStatus;
    /**
     * Signed Transaction
     */
    transaction: SignedTransactionView;
    /**
     * The execution outcome of the signed transaction.
     */
    transaction_outcome: ExecutionOutcomeWithIdView;
};

