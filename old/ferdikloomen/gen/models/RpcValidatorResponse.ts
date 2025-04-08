/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { CurrentEpochValidatorInfo } from './CurrentEpochValidatorInfo';
import type { NextEpochValidatorInfo } from './NextEpochValidatorInfo';
import type { ValidatorKickoutView } from './ValidatorKickoutView';
import type { ValidatorStakeView } from './ValidatorStakeView';
/**
 * Information about this epoch validators and next epoch validators
 */
export type RpcValidatorResponse = {
    /**
     * Fishermen for the current epoch
     */
    current_fishermen: Array<ValidatorStakeView>;
    /**
     * Proposals in the current epoch
     */
    current_proposals: Array<ValidatorStakeView>;
    /**
     * Validators for the current epoch
     */
    current_validators: Array<CurrentEpochValidatorInfo>;
    /**
     * Epoch height
     */
    epoch_height: number;
    /**
     * Epoch start block height
     */
    epoch_start_height: number;
    /**
     * Fishermen for the next epoch
     */
    next_fishermen: Array<ValidatorStakeView>;
    /**
     * Validators for the next epoch
     */
    next_validators: Array<NextEpochValidatorInfo>;
    /**
     * Kickout in the previous epoch
     */
    prev_epoch_kickout: Array<ValidatorKickoutView>;
};

