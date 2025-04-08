/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { BlockStatusView } from './BlockStatusView';
import type { CatchupStatusView } from './CatchupStatusView';
import type { NetworkInfoView } from './NetworkInfoView';
export type DetailedDebugStatus = {
    block_production_delay_millis: number;
    catchup_status: Array<CatchupStatusView>;
    current_head_status: BlockStatusView;
    current_header_head_status: BlockStatusView;
    network_info: NetworkInfoView;
    sync_status: string;
};

