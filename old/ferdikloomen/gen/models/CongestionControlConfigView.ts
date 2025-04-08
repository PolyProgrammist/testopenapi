/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
export type CongestionControlConfigView = {
    /**
     * How much gas the chosen allowed shard can send to a 100% congested shard.
     *
     * See [`CongestionControlConfig`] for more details.
     */
    allowed_shard_outgoing_gas: number;
    /**
     * How much gas in delayed receipts of a shard is 100% incoming congestion.
     *
     * See [`CongestionControlConfig`] for more details.
     */
    max_congestion_incoming_gas: number;
    /**
     * How much memory space of all delayed and buffered receipts in a shard is
     * considered 100% congested.
     *
     * See [`CongestionControlConfig`] for more details.
     */
    max_congestion_memory_consumption: number;
    /**
     * How many missed chunks in a row in a shard is considered 100% congested.
     */
    max_congestion_missed_chunks: number;
    /**
     * How much gas in outgoing buffered receipts of a shard is 100% congested.
     *
     * Outgoing congestion contributes to overall congestion, which reduces how
     * much other shards are allowed to forward to this shard.
     */
    max_congestion_outgoing_gas: number;
    /**
     * The maximum amount of gas attached to receipts a shard can forward to
     * another shard per chunk.
     *
     * See [`CongestionControlConfig`] for more details.
     */
    max_outgoing_gas: number;
    /**
     * The maximum amount of gas in a chunk spent on converting new transactions to
     * receipts.
     *
     * See [`CongestionControlConfig`] for more details.
     */
    max_tx_gas: number;
    /**
     * The minimum gas each shard can send to a shard that is not fully congested.
     *
     * See [`CongestionControlConfig`] for more details.
     */
    min_outgoing_gas: number;
    /**
     * The minimum amount of gas in a chunk spent on converting new transactions
     * to receipts, as long as the receiving shard is not congested.
     *
     * See [`CongestionControlConfig`] for more details.
     */
    min_tx_gas: number;
    /**
     * Large size limit for outgoing receipts to a shard, used when it's safe
     * to send a lot of receipts without making the state witness too large.
     * It limits the total sum of outgoing receipts, not individual receipts.
     */
    outgoing_receipts_big_size_limit: number;
    /**
     * The standard size limit for outgoing receipts aimed at a single shard.
     * This limit is pretty small to keep the size of source_receipt_proofs under control.
     * It limits the total sum of outgoing receipts, not individual receipts.
     */
    outgoing_receipts_usual_size_limit: number;
    /**
     * How much congestion a shard can tolerate before it stops all shards from
     * accepting new transactions with the receiver set to the congested shard.
     */
    reject_tx_congestion_threshold: number;
};

