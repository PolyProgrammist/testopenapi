/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { CompilationError } from './CompilationError';
import type { HostError } from './HostError';
import type { MethodResolveError } from './MethodResolveError';
import type { WasmTrap } from './WasmTrap';
/**
 * Serializable version of `near-vm-runner::FunctionCallError`.
 *
 * Must never reorder/remove elements, can only add new variants at the end (but do that very carefully). It describes stable serialization format, and only used by serialization logic.
 */
export type FunctionCallError = ('WasmUnknownError' | '_EVMError' | {
    CompilationError: CompilationError;
} | {
    LinkError: {
        msg: string;
    };
} | {
    MethodResolveError: MethodResolveError;
} | {
    WasmTrap: WasmTrap;
} | {
    HostError: HostError;
} | {
    ExecutionError: string;
});

