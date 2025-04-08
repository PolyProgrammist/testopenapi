/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { AccountId } from './AccountId';
import type { AccountInfo } from './AccountInfo';
import type { Rational32SchemaProvider } from './Rational32SchemaProvider';
import type { ShardLayout } from './ShardLayout';
export type GenesisConfig = {
    /**
     * Expected number of hidden validators per shard.
     */
    avg_hidden_validator_seats_per_shard: Array<number>;
    /**
     * Threshold for kicking out block producers, between 0 and 100.
     */
    block_producer_kickout_threshold: number;
    /**
     * ID of the blockchain. This must be unique for every blockchain.
     * If your testnet blockchains do not have unique chain IDs, you will have a bad time.
     */
    chain_id: string;
    /**
     * Limits the number of shard changes in chunk producer assignments,
     * if algorithm is able to choose assignment with better balance of
     * number of chunk producers for shards.
     */
    chunk_producer_assignment_changes_limit?: number;
    /**
     * Threshold for kicking out chunk producers, between 0 and 100.
     */
    chunk_producer_kickout_threshold: number;
    /**
     * Threshold for kicking out nodes which are only chunk validators, between 0 and 100.
     */
    chunk_validator_only_kickout_threshold?: number;
    /**
     * Enable dynamic re-sharding.
     */
    dynamic_resharding: boolean;
    /**
     * Epoch length counted in block heights.
     */
    epoch_length: number;
    /**
     * Fishermen stake threshold.
     */
    fishermen_threshold: string;
    /**
     * Initial gas limit.
     */
    gas_limit: number;
    /**
     * Gas price adjustment rate
     */
    gas_price_adjustment_rate: Rational32SchemaProvider;
    /**
     * Height of genesis block.
     */
    genesis_height: number;
    /**
     * Official time of blockchain start.
     */
    genesis_time: string;
    max_gas_price: string;
    /**
     * Maximum inflation on the total supply every epoch.
     */
    max_inflation_rate: Rational32SchemaProvider;
    /**
     * Max stake percentage of the validators we will kick out.
     */
    max_kickout_stake_perc?: number;
    /**
     * Minimum gas price. It is also the initial gas price.
     */
    min_gas_price: string;
    /**
     * The minimum stake required for staking is last seat price divided by this number.
     */
    minimum_stake_divisor?: number;
    /**
     * The lowest ratio s/s_total any block producer can have.
     * See <https://github.com/near/NEPs/pull/167> for details
     */
    minimum_stake_ratio?: Rational32SchemaProvider;
    /**
     * The minimum number of validators each shard must have
     */
    minimum_validators_per_shard?: number;
    /**
     * Number of block producer seats at genesis.
     */
    num_block_producer_seats: number;
    /**
     * Defines number of shards and number of block producer seats per each shard at genesis.
     * Note: not used with protocol_feature_chunk_only_producers -- replaced by minimum_validators_per_shard
     * Note: not used before as all block producers produce chunks for all shards
     */
    num_block_producer_seats_per_shard: Array<number>;
    /**
     * Expected number of blocks per year
     */
    num_blocks_per_year: number;
    /**
     * Deprecated.
     */
    num_chunk_only_producer_seats?: number;
    /**
     * Number of chunk producers.
     * Don't mess it up with chunk-only producers feature which is deprecated.
     */
    num_chunk_producer_seats?: number;
    num_chunk_validator_seats?: number;
    /**
     * Online maximum threshold above which validator gets full reward.
     */
    online_max_threshold?: Rational32SchemaProvider;
    /**
     * Online minimum threshold below which validator doesn't receive reward.
     */
    online_min_threshold?: Rational32SchemaProvider;
    /**
     * Protocol treasury rate
     */
    protocol_reward_rate: Rational32SchemaProvider;
    /**
     * Protocol treasury account
     */
    protocol_treasury_account: AccountId;
    /**
     * Threshold of stake that needs to indicate that they ready for upgrade.
     */
    protocol_upgrade_stake_threshold?: Rational32SchemaProvider;
    /**
     * Protocol version that this genesis works with.
     */
    protocol_version: number;
    /**
     * Layout information regarding how to split accounts to shards
     */
    shard_layout?: ShardLayout;
    /**
     * If true, shuffle the chunk producers across shards. In other words, if
     * the shard assignments were `[S_0, S_1, S_2, S_3]` where `S_i` represents
     * the set of chunk producers for shard `i`, if this flag were true, the
     * shard assignments might become, for example, `[S_2, S_0, S_3, S_1]`.
     */
    shuffle_shard_assignment_for_chunk_producers?: boolean;
    /**
     * Number of target chunk validator mandates for each shard.
     */
    target_validator_mandates_per_shard?: number;
    /**
     * Total supply of tokens at genesis.
     */
    total_supply: string;
    /**
     * Number of blocks for which a given transaction is valid
     */
    transaction_validity_period: number;
    /**
     * This is only for test purposes. We hard code some configs for mainnet and testnet
     * in AllEpochConfig, and we want to have a way to test that code path. This flag is for that.
     * If set to true, the node will use the same config override path as mainnet and testnet.
     */
    use_production_config?: boolean;
    /**
     * List of initial validators.
     */
    validators: Array<AccountInfo>;
};

