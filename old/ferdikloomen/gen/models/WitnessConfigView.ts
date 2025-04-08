/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
/**
 * Configuration specific to ChunkStateWitness.
 */
export type WitnessConfigView = {
    /**
     * A witness contains transactions from both the previous chunk and the current one.
     * This parameter limits the sum of sizes of transactions from both of those chunks.
     */
    combined_transactions_size_limit: number;
    /**
     * Size limit for storage proof generated while executing receipts in a chunk.
     * After this limit is reached we defer execution of any new receipts.
     */
    main_storage_proof_size_soft_limit: number;
    /**
     * Soft size limit of storage proof used to validate new transactions in ChunkStateWitness.
     */
    new_transactions_validation_state_size_soft_limit: number;
};

