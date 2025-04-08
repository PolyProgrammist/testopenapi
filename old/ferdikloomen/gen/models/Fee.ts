/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
/**
 * Costs associated with an object that can only be sent over the network (and executed
 * by the receiver).
 * NOTE: `send_sir` or `send_not_sir` fees are usually burned when the item is being created.
 * And `execution` fee is burned when the item is being executed.
 */
export type Fee = {
    /**
     * Fee for executing the object.
     */
    execution: number;
    /**
     * Fee for sending an object potentially across the shards.
     */
    send_not_sir: number;
    /**
     * Fee for sending an object from the sender to itself, guaranteeing that it does not leave
     * the shard.
     */
    send_sir: number;
};

