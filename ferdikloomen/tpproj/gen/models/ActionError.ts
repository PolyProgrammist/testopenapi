/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { ActionErrorKind } from './ActionErrorKind';
/**
 * An error happened during Action execution
 */
export type ActionError = {
    /**
     * Index of the failed action in the transaction. Action index is not defined if ActionError.kind is `ActionErrorKind::LackBalanceForState`
     */
    index?: number | null;
    /**
     * The kind of ActionError happened
     */
    kind: ActionErrorKind;
};

