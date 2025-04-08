/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { AccountId } from './AccountId';
import type { ChunkDistributionNetworkConfig } from './ChunkDistributionNetworkConfig';
import type { EpochSyncConfig } from './EpochSyncConfig';
import type { GCConfig } from './GCConfig';
import type { LogSummaryStyle } from './LogSummaryStyle';
import type { MutableConfigValue } from './MutableConfigValue';
import type { ShardId } from './ShardId';
import type { StateSyncConfig } from './StateSyncConfig';
import type { Version } from './Version';
/**
 * ClientConfig where some fields can be updated at runtime.
 */
export type RpcClientConfigResponse = {
    /**
     * Not clear old data, set `true` for archive nodes.
     */
    archive: boolean;
    /**
     * Horizon at which instead of fetching block, fetch full state.
     */
    block_fetch_horizon: number;
    /**
     * Behind this horizon header fetch kicks in.
     */
    block_header_fetch_horizon: number;
    /**
     * Duration to check for producing / skipping block.
     */
    block_production_tracking_delay: Array<number>;
    /**
     * Time between check to perform catchup.
     */
    catchup_step_period: Array<number>;
    /**
     * Chain id for status.
     */
    chain_id: string;
    /**
     * Optional config for the Chunk Distribution Network feature.
     * If set to `None` then this node does not participate in the Chunk Distribution Network.
     * Nodes not participating will still function fine, but possibly with higher
     * latency due to the need of requesting chunks over the peer-to-peer network.
     */
    chunk_distribution_network?: ChunkDistributionNetworkConfig | null;
    /**
     * Time between checking to re-request chunks.
     */
    chunk_request_retry_period: Array<number>;
    /**
     * Number of threads to execute background migration work in client.
     */
    client_background_migration_threads: number;
    /**
     * Time between running doomslug timer.
     */
    doomslug_step_period: Array<number>;
    enable_multiline_logging: boolean;
    /**
     * Re-export storage layer statistics as prometheus metrics.
     */
    enable_statistics_export: boolean;
    /**
     * Epoch length.
     */
    epoch_length: number;
    /**
     * Options for epoch sync.
     */
    epoch_sync: EpochSyncConfig;
    /**
     * Graceful shutdown at expected block height.
     */
    expected_shutdown: MutableConfigValue;
    /**
     * Garbage collection configuration.
     */
    gc: GCConfig;
    /**
     * Expected increase of header head height per second during header sync
     */
    header_sync_expected_height_per_second: number;
    /**
     * How much time to wait after initial header sync
     */
    header_sync_initial_timeout: Array<number>;
    /**
     * How much time to wait after some progress is made in header sync
     */
    header_sync_progress_timeout: Array<number>;
    /**
     * How much time to wait before banning a peer in header sync if sync is too slow
     */
    header_sync_stall_ban_timeout: Array<number>;
    /**
     * Period between logging summary information.
     */
    log_summary_period: Array<number>;
    /**
     * Enable coloring of the logs
     */
    log_summary_style: LogSummaryStyle;
    /**
     * Maximum wait for approvals before producing block.
     */
    max_block_production_delay: Array<number>;
    /**
     * Maximum duration before skipping given height.
     */
    max_block_wait_delay: Array<number>;
    /**
     * Max burnt gas per view method.  If present, overrides value stored in
     * genesis file.  The value only affects the RPCs without influencing the
     * protocol thus changing it per-node doesn’t affect the blockchain.
     */
    max_gas_burnt_view?: number | null;
    /**
     * Minimum duration before producing block.
     */
    min_block_production_delay: Array<number>;
    /**
     * Minimum number of peers to start syncing.
     */
    min_num_peers: number;
    /**
     * Number of block producer seats
     */
    num_block_producer_seats: number;
    /**
     * Maximum size of state witnesses in the OrphanStateWitnessPool.
     *
     * We keep only orphan witnesses which are smaller than this size.
     * This limits the maximum memory usage of OrphanStateWitnessPool.
     */
    orphan_state_witness_max_size: number;
    /**
     * OrphanStateWitnessPool keeps instances of ChunkStateWitness which can't be processed
     * because the previous block isn't available. The witnesses wait in the pool until the
     * required block appears. This variable controls how many witnesses can be stored in the pool.
     */
    orphan_state_witness_pool_size: number;
    /**
     * Limit the time of adding transactions to a chunk.
     * A node produces a chunk by adding transactions from the transaction pool until
     * some limit is reached. This time limit ensures that adding transactions won't take
     * longer than the specified duration, which helps to produce the chunk quickly.
     */
    produce_chunk_add_transactions_time_limit: string;
    /**
     * Produce empty blocks, use `false` for testing.
     */
    produce_empty_blocks: boolean;
    resharding_config: MutableConfigValue;
    /**
     * Listening rpc port for status.
     */
    rpc_addr?: string | null;
    /**
     * Save observed instances of ChunkStateWitness to the database in DBCol::LatestChunkStateWitnesses.
     * Saving the latest witnesses is useful for analysis and debugging.
     * When this option is enabled, the node will save ALL witnesses it observes, even invalid ones,
     * which can cause extra load on the database. This option is not recommended for production use,
     * as a large number of incoming witnesses could cause denial of service.
     */
    save_latest_witnesses: boolean;
    /**
     * save_trie_changes should be set to true iff
     * - archive if false - non-archival nodes need trie changes to perform garbage collection
     * - archive is true, cold_store is configured and migration to split_storage is finished - node
     * working in split storage mode needs trie changes in order to do garbage collection on hot.
     */
    save_trie_changes: boolean;
    /**
     * Skip waiting for sync (for testing or single node testnet).
     */
    skip_sync_wait: boolean;
    /**
     * Options for syncing state.
     */
    state_sync: StateSyncConfig;
    /**
     * Whether to use the State Sync mechanism.
     * If disabled, the node will do Block Sync instead of State Sync.
     */
    state_sync_enabled: boolean;
    /**
     * Additional waiting period after a failed request to external storage
     */
    state_sync_external_backoff: Array<number>;
    /**
     * How long to wait for a response from centralized state sync
     */
    state_sync_external_timeout: Array<number>;
    /**
     * How long to wait for a response from p2p state sync
     */
    state_sync_p2p_timeout: Array<number>;
    /**
     * How long to wait after a failed state sync request
     */
    state_sync_retry_backoff: Array<number>;
    /**
     * How often to check that we are not out of sync.
     */
    sync_check_period: Array<number>;
    /**
     * Sync height threshold: below this difference in height don't start syncing.
     */
    sync_height_threshold: number;
    /**
     * Maximum number of block requests to send to peers to sync
     */
    sync_max_block_requests: number;
    /**
     * While syncing, how long to check for each step.
     */
    sync_step_period: Array<number>;
    /**
     * Accounts that this client tracks.
     */
    tracked_accounts: Array<AccountId>;
    /**
     * Track shards that should be tracked by given validator.
     */
    tracked_shadow_validator?: AccountId | null;
    /**
     * Rotate between these sets of tracked shards.
     * Used to simulate the behavior of chunk only producers without staking tokens.
     * This field is only used if `tracked_shards` is empty.
     */
    tracked_shard_schedule: Array<Array<ShardId>>;
    /**
     * Shards that this client tracks.
     */
    tracked_shards: Array<ShardId>;
    /**
     * Limit of the size of per-shard transaction pool measured in bytes. If not set, the size
     * will be unbounded.
     */
    transaction_pool_size_limit?: number | null;
    /**
     * Upper bound of the byte size of contract state that is still viewable. None is no limit
     */
    trie_viewer_state_size_limit?: number | null;
    /**
     * Time to persist Accounts Id in the router without removing them.
     */
    ttl_account_id_router: Array<number>;
    /**
     * If the node is not a chunk producer within that many blocks, then route
     * to upcoming chunk producers.
     */
    tx_routing_height_horizon: number;
    /**
     * Version of the binary.
     */
    version: Version;
    /**
     * Number of threads for ViewClientActor pool.
     */
    view_client_threads: number;
    /**
     * Number of seconds between state requests for view client.
     */
    view_client_throttle_period: Array<number>;
};

