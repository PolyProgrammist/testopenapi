/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { GlobalContractDeployMode } from './GlobalContractDeployMode';
/**
 * Deploy global contract action
 */
export type DeployGlobalContractAction = {
    /**
     * WebAssembly binary
     */
    code: string;
    deploy_mode: GlobalContractDeployMode;
};

