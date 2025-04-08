/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
/**
 * Reasons for removing a validator from the validator set.
 */
export type ValidatorKickoutReason = (string | {
    NotEnoughBlocks: {
        expected: number;
        produced: number;
    };
} | {
    NotEnoughChunks: {
        expected: number;
        produced: number;
    };
} | {
    NotEnoughStake: {
        stake_u128: string;
        threshold_u128: string;
    };
} | {
    NotEnoughChunkEndorsements: {
        expected: number;
        produced: number;
    };
});

