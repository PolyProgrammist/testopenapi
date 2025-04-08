/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { AccountId } from './AccountId';
import type { CryptoHash } from './CryptoHash';
import type { DetailedDebugStatus } from './DetailedDebugStatus';
import type { PublicKey } from './PublicKey';
import type { StatusSyncInfo } from './StatusSyncInfo';
import type { ValidatorInfo } from './ValidatorInfo';
import type { Version } from './Version';
export type RpcStatusResponse = {
    /**
     * Unique chain id.
     */
    chain_id: string;
    /**
     * Information about last blocks, network, epoch and chain & chunk info.
     */
    detailed_debug_status?: DetailedDebugStatus | null;
    /**
     * Genesis hash of the chain.
     */
    genesis_hash: CryptoHash;
    /**
     * Latest protocol version that this client supports.
     */
    latest_protocol_version: number;
    /**
     * Deprecated; same as `validator_public_key` which you should use instead.
     */
    node_key?: PublicKey | null;
    /**
     * Public key of the node.
     */
    node_public_key: PublicKey;
    /**
     * Currently active protocol version.
     */
    protocol_version: number;
    /**
     * Address for RPC server.  None if node doesn’t have RPC endpoint enabled.
     */
    rpc_addr?: string | null;
    /**
     * Sync status of the node.
     */
    sync_info: StatusSyncInfo;
    /**
     * Uptime of the node.
     */
    uptime_sec: number;
    /**
     * Validator id of the node
     */
    validator_account_id?: AccountId | null;
    /**
     * Public key of the validator.
     */
    validator_public_key?: PublicKey | null;
    /**
     * Current epoch validators.
     */
    validators: Array<ValidatorInfo>;
    /**
     * Binary version.
     */
    version: Version;
};

