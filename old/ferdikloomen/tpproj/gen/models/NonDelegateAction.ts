/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { Action } from './Action';
/**
 * This is Action which mustn't contain DelegateAction.
 *
 * This struct is needed to avoid the recursion when Action/DelegateAction is deserialized.
 *
 * Important: Don't make the inner Action public, this must only be constructed through the correct interface that ensures the inner Action is actually not a delegate action. That would break an assumption of this type, which we use in several places. For example, borsh de-/serialization relies on it. If the invariant is broken, we may end up with a `Transaction` or `Receipt` that we can serialize but deserializing it back causes a parsing error.
 */
export type NonDelegateAction = Action;

