/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { ShardLayoutV0 } from './ShardLayoutV0';
import type { ShardLayoutV1 } from './ShardLayoutV1';
import type { ShardLayoutV2 } from './ShardLayoutV2';
/**
 * A versioned struct that contains all information needed to assign accounts to shards.
 *
 * Because of re-sharding, the chain may use different shard layout to split shards at different
 * times. Currently, `ShardLayout` is stored as part of `EpochConfig`, which is generated each
 * epoch given the epoch protocol version. In mainnet/testnet, we use two shard layouts since
 * re-sharding has only happened once. It is stored as part of genesis config, see
 * default_simple_nightshade_shard_layout() Below is an overview for some important
 * functionalities of ShardLayout interface.
 */
export type ShardLayout = ({
    V0: ShardLayoutV0;
} | {
    V1: ShardLayoutV1;
} | {
    V2: ShardLayoutV2;
});

