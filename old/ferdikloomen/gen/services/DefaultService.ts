/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { JsonRpcRequest_for_BlockMethodNameHelperEnum } from '../models/JsonRpcRequest_for_BlockMethodNameHelperEnum';
import type { JsonRpcRequest_for_BroadCastTxAsyncMethodNameHelperEnum } from '../models/JsonRpcRequest_for_BroadCastTxAsyncMethodNameHelperEnum';
import type { JsonRpcRequest_for_BroadCastTxCommitMethodNameHelperEnum } from '../models/JsonRpcRequest_for_BroadCastTxCommitMethodNameHelperEnum';
import type { JsonRpcRequest_for_ChunkMethodNameHelperEnum } from '../models/JsonRpcRequest_for_ChunkMethodNameHelperEnum';
import type { JsonRpcRequest_for_ClientConfigMethodNameHelperEnum } from '../models/JsonRpcRequest_for_ClientConfigMethodNameHelperEnum';
import type { JsonRpcRequest_for_ExpChangeMethodNameHelperEnum } from '../models/JsonRpcRequest_for_ExpChangeMethodNameHelperEnum';
import type { JsonRpcRequest_for_ExpChangesBlockMethodNameHelperEnum } from '../models/JsonRpcRequest_for_ExpChangesBlockMethodNameHelperEnum';
import type { JsonRpcRequest_for_ExpGenesisMethodNameHelperEnum } from '../models/JsonRpcRequest_for_ExpGenesisMethodNameHelperEnum';
import type { JsonRpcRequest_for_ExpGongestionMethodNameHelperEnum } from '../models/JsonRpcRequest_for_ExpGongestionMethodNameHelperEnum';
import type { JsonRpcRequest_for_ExpLightClientBlockProofMethodNameHelperEnum } from '../models/JsonRpcRequest_for_ExpLightClientBlockProofMethodNameHelperEnum';
import type { JsonRpcRequest_for_ExpLightClientProofMethodNameHelperEnum } from '../models/JsonRpcRequest_for_ExpLightClientProofMethodNameHelperEnum';
import type { JsonRpcRequest_for_ExpMaintenanceWindoesMethodNameHelperEnum } from '../models/JsonRpcRequest_for_ExpMaintenanceWindoesMethodNameHelperEnum';
import type { JsonRpcRequest_for_ExpProtocolConfigMethodNameHelperEnum } from '../models/JsonRpcRequest_for_ExpProtocolConfigMethodNameHelperEnum';
import type { JsonRpcRequest_for_ExpReceiptMethodNameHelperEnum } from '../models/JsonRpcRequest_for_ExpReceiptMethodNameHelperEnum';
import type { JsonRpcRequest_for_ExpSplitStorageInfoMethodNameHelperEnum } from '../models/JsonRpcRequest_for_ExpSplitStorageInfoMethodNameHelperEnum';
import type { JsonRpcRequest_for_ExpTxStatusMethodNameHelperEnum } from '../models/JsonRpcRequest_for_ExpTxStatusMethodNameHelperEnum';
import type { JsonRpcRequest_for_ExpValidatorsMethodNameHelperEnum } from '../models/JsonRpcRequest_for_ExpValidatorsMethodNameHelperEnum';
import type { JsonRpcRequest_for_GasPriceMethodNameHelperEnum } from '../models/JsonRpcRequest_for_GasPriceMethodNameHelperEnum';
import type { JsonRpcRequest_for_HealthMethodNameHelperEnum } from '../models/JsonRpcRequest_for_HealthMethodNameHelperEnum';
import type { JsonRpcRequest_for_LightClientProofMethodNameHelperEnum } from '../models/JsonRpcRequest_for_LightClientProofMethodNameHelperEnum';
import type { JsonRpcRequest_for_NetworkInfoMethodNameHelperEnum } from '../models/JsonRpcRequest_for_NetworkInfoMethodNameHelperEnum';
import type { JsonRpcRequest_for_NextLightClientBlockMethodNameHelperEnum } from '../models/JsonRpcRequest_for_NextLightClientBlockMethodNameHelperEnum';
import type { JsonRpcRequest_for_SendTxMethodNameHelperEnum } from '../models/JsonRpcRequest_for_SendTxMethodNameHelperEnum';
import type { JsonRpcRequest_for_StatusMethodNameHelperEnum } from '../models/JsonRpcRequest_for_StatusMethodNameHelperEnum';
import type { JsonRpcRequest_for_TxMethodNameHelperEnum } from '../models/JsonRpcRequest_for_TxMethodNameHelperEnum';
import type { JsonRpcRequest_for_ValidatorsMethodNameHelperEnum } from '../models/JsonRpcRequest_for_ValidatorsMethodNameHelperEnum';
import type { JsonRpcResponse_for_Array_of_Tuple_of_uint64_and_uint64_and_RpcError } from '../models/JsonRpcResponse_for_Array_of_Tuple_of_uint64_and_uint64_and_RpcError';
import type { JsonRpcResponse_for_Array_of_ValidatorStakeView_and_RpcError } from '../models/JsonRpcResponse_for_Array_of_ValidatorStakeView_and_RpcError';
import type { JsonRpcResponse_for_CryptoHash_and_RpcError } from '../models/JsonRpcResponse_for_CryptoHash_and_RpcError';
import type { JsonRpcResponse_for_GenesisConfig_and_RpcError } from '../models/JsonRpcResponse_for_GenesisConfig_and_RpcError';
import type { JsonRpcResponse_for_Nullable_RpcHealthResponse_and_RpcError } from '../models/JsonRpcResponse_for_Nullable_RpcHealthResponse_and_RpcError';
import type { JsonRpcResponse_for_RpcBlockResponse_and_RpcError } from '../models/JsonRpcResponse_for_RpcBlockResponse_and_RpcError';
import type { JsonRpcResponse_for_RpcChunkResponse_and_RpcError } from '../models/JsonRpcResponse_for_RpcChunkResponse_and_RpcError';
import type { JsonRpcResponse_for_RpcClientConfigResponse_and_RpcError } from '../models/JsonRpcResponse_for_RpcClientConfigResponse_and_RpcError';
import type { JsonRpcResponse_for_RpcCongestionLevelResponse_and_RpcError } from '../models/JsonRpcResponse_for_RpcCongestionLevelResponse_and_RpcError';
import type { JsonRpcResponse_for_RpcGasPriceResponse_and_RpcError } from '../models/JsonRpcResponse_for_RpcGasPriceResponse_and_RpcError';
import type { JsonRpcResponse_for_RpcLightClientBlockProofResponse_and_RpcError } from '../models/JsonRpcResponse_for_RpcLightClientBlockProofResponse_and_RpcError';
import type { JsonRpcResponse_for_RpcLightClientExecutionProofResponse_and_RpcError } from '../models/JsonRpcResponse_for_RpcLightClientExecutionProofResponse_and_RpcError';
import type { JsonRpcResponse_for_RpcLightClientNextBlockResponse_and_RpcError } from '../models/JsonRpcResponse_for_RpcLightClientNextBlockResponse_and_RpcError';
import type { JsonRpcResponse_for_RpcNetworkInfoResponse_and_RpcError } from '../models/JsonRpcResponse_for_RpcNetworkInfoResponse_and_RpcError';
import type { JsonRpcResponse_for_RpcProtocolConfigResponse_and_RpcError } from '../models/JsonRpcResponse_for_RpcProtocolConfigResponse_and_RpcError';
import type { JsonRpcResponse_for_RpcReceiptResponse_and_RpcError } from '../models/JsonRpcResponse_for_RpcReceiptResponse_and_RpcError';
import type { JsonRpcResponse_for_RpcSplitStorageInfoResponse_and_RpcError } from '../models/JsonRpcResponse_for_RpcSplitStorageInfoResponse_and_RpcError';
import type { JsonRpcResponse_for_RpcStateChangesInBlockByTypeResponse_and_RpcError } from '../models/JsonRpcResponse_for_RpcStateChangesInBlockByTypeResponse_and_RpcError';
import type { JsonRpcResponse_for_RpcStateChangesInBlockResponse_and_RpcError } from '../models/JsonRpcResponse_for_RpcStateChangesInBlockResponse_and_RpcError';
import type { JsonRpcResponse_for_RpcStatusResponse_and_RpcError } from '../models/JsonRpcResponse_for_RpcStatusResponse_and_RpcError';
import type { JsonRpcResponse_for_RpcTransactionResponse_and_RpcError } from '../models/JsonRpcResponse_for_RpcTransactionResponse_and_RpcError';
import type { JsonRpcResponse_for_RpcValidatorResponse_and_RpcError } from '../models/JsonRpcResponse_for_RpcValidatorResponse_and_RpcError';
import type { CancelablePromise } from '../core/CancelablePromise';
import { OpenAPI } from '../core/OpenAPI';
import { request as __request } from '../core/request';
export class DefaultService {
    /**
     * @param requestBody
     * @returns JsonRpcResponse_for_RpcStateChangesInBlockResponse_and_RpcError
     * @throws ApiError
     */
    public static experimentalChanges(
        requestBody: JsonRpcRequest_for_ExpChangeMethodNameHelperEnum,
    ): CancelablePromise<JsonRpcResponse_for_RpcStateChangesInBlockResponse_and_RpcError> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/EXPERIMENTAL_changes',
            body: requestBody,
            mediaType: 'application/json',
        });
    }
    /**
     * @param requestBody
     * @returns JsonRpcResponse_for_RpcStateChangesInBlockByTypeResponse_and_RpcError
     * @throws ApiError
     */
    public static experimentalChangesInBlock(
        requestBody: JsonRpcRequest_for_ExpChangesBlockMethodNameHelperEnum,
    ): CancelablePromise<JsonRpcResponse_for_RpcStateChangesInBlockByTypeResponse_and_RpcError> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/EXPERIMENTAL_changes_in_block',
            body: requestBody,
            mediaType: 'application/json',
        });
    }
    /**
     * @param requestBody
     * @returns JsonRpcResponse_for_RpcCongestionLevelResponse_and_RpcError
     * @throws ApiError
     */
    public static experimentalCongestionLevel(
        requestBody: JsonRpcRequest_for_ExpGongestionMethodNameHelperEnum,
    ): CancelablePromise<JsonRpcResponse_for_RpcCongestionLevelResponse_and_RpcError> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/EXPERIMENTAL_congestion_level',
            body: requestBody,
            mediaType: 'application/json',
        });
    }
    /**
     * @param requestBody
     * @returns JsonRpcResponse_for_GenesisConfig_and_RpcError
     * @throws ApiError
     */
    public static experimentalGenesisConfig(
        requestBody: JsonRpcRequest_for_ExpGenesisMethodNameHelperEnum,
    ): CancelablePromise<JsonRpcResponse_for_GenesisConfig_and_RpcError> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/EXPERIMENTAL_genesis_config',
            body: requestBody,
            mediaType: 'application/json',
        });
    }
    /**
     * @param requestBody
     * @returns JsonRpcResponse_for_RpcLightClientBlockProofResponse_and_RpcError
     * @throws ApiError
     */
    public static experimentalLightClientBlockProof(
        requestBody: JsonRpcRequest_for_ExpLightClientBlockProofMethodNameHelperEnum,
    ): CancelablePromise<JsonRpcResponse_for_RpcLightClientBlockProofResponse_and_RpcError> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/EXPERIMENTAL_light_client_block_proof',
            body: requestBody,
            mediaType: 'application/json',
        });
    }
    /**
     * @param requestBody
     * @returns JsonRpcResponse_for_RpcLightClientExecutionProofResponse_and_RpcError
     * @throws ApiError
     */
    public static experimentalLightClientProof(
        requestBody: JsonRpcRequest_for_ExpLightClientProofMethodNameHelperEnum,
    ): CancelablePromise<JsonRpcResponse_for_RpcLightClientExecutionProofResponse_and_RpcError> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/EXPERIMENTAL_light_client_proof',
            body: requestBody,
            mediaType: 'application/json',
        });
    }
    /**
     * @param requestBody
     * @returns JsonRpcResponse_for_Array_of_Tuple_of_uint64_and_uint64_and_RpcError
     * @throws ApiError
     */
    public static experimentalMaintenanceWindows(
        requestBody: JsonRpcRequest_for_ExpMaintenanceWindoesMethodNameHelperEnum,
    ): CancelablePromise<JsonRpcResponse_for_Array_of_Tuple_of_uint64_and_uint64_and_RpcError> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/EXPERIMENTAL_maintenance_windows',
            body: requestBody,
            mediaType: 'application/json',
        });
    }
    /**
     * @param requestBody
     * @returns JsonRpcResponse_for_RpcProtocolConfigResponse_and_RpcError
     * @throws ApiError
     */
    public static experimentalProtocolConfig(
        requestBody: JsonRpcRequest_for_ExpProtocolConfigMethodNameHelperEnum,
    ): CancelablePromise<JsonRpcResponse_for_RpcProtocolConfigResponse_and_RpcError> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/EXPERIMENTAL_protocol_config',
            body: requestBody,
            mediaType: 'application/json',
        });
    }
    /**
     * @param requestBody
     * @returns JsonRpcResponse_for_RpcReceiptResponse_and_RpcError
     * @throws ApiError
     */
    public static experimentalReceipt(
        requestBody: JsonRpcRequest_for_ExpReceiptMethodNameHelperEnum,
    ): CancelablePromise<JsonRpcResponse_for_RpcReceiptResponse_and_RpcError> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/EXPERIMENTAL_receipt',
            body: requestBody,
            mediaType: 'application/json',
        });
    }
    /**
     * @param requestBody
     * @returns JsonRpcResponse_for_RpcSplitStorageInfoResponse_and_RpcError
     * @throws ApiError
     */
    public static experimentalSplitStorageInfo(
        requestBody: JsonRpcRequest_for_ExpSplitStorageInfoMethodNameHelperEnum,
    ): CancelablePromise<JsonRpcResponse_for_RpcSplitStorageInfoResponse_and_RpcError> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/EXPERIMENTAL_split_storage_info',
            body: requestBody,
            mediaType: 'application/json',
        });
    }
    /**
     * @param requestBody
     * @returns JsonRpcResponse_for_RpcTransactionResponse_and_RpcError
     * @throws ApiError
     */
    public static experimentalTxStatus(
        requestBody: JsonRpcRequest_for_ExpTxStatusMethodNameHelperEnum,
    ): CancelablePromise<JsonRpcResponse_for_RpcTransactionResponse_and_RpcError> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/EXPERIMENTAL_tx_status',
            body: requestBody,
            mediaType: 'application/json',
        });
    }
    /**
     * @param requestBody
     * @returns JsonRpcResponse_for_Array_of_ValidatorStakeView_and_RpcError
     * @throws ApiError
     */
    public static experimentalValidatorsOrdered(
        requestBody: JsonRpcRequest_for_ExpValidatorsMethodNameHelperEnum,
    ): CancelablePromise<JsonRpcResponse_for_Array_of_ValidatorStakeView_and_RpcError> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/EXPERIMENTAL_validators_ordered',
            body: requestBody,
            mediaType: 'application/json',
        });
    }
    /**
     * @param requestBody
     * @returns JsonRpcResponse_for_RpcBlockResponse_and_RpcError
     * @throws ApiError
     */
    public static block(
        requestBody: JsonRpcRequest_for_BlockMethodNameHelperEnum,
    ): CancelablePromise<JsonRpcResponse_for_RpcBlockResponse_and_RpcError> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/block',
            body: requestBody,
            mediaType: 'application/json',
        });
    }
    /**
     * @param requestBody
     * @returns JsonRpcResponse_for_CryptoHash_and_RpcError
     * @throws ApiError
     */
    public static broadcastTxAsync(
        requestBody: JsonRpcRequest_for_BroadCastTxAsyncMethodNameHelperEnum,
    ): CancelablePromise<JsonRpcResponse_for_CryptoHash_and_RpcError> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/broadcast_tx_async',
            body: requestBody,
            mediaType: 'application/json',
        });
    }
    /**
     * @param requestBody
     * @returns JsonRpcResponse_for_RpcTransactionResponse_and_RpcError
     * @throws ApiError
     */
    public static broadcastTxCommit(
        requestBody: JsonRpcRequest_for_BroadCastTxCommitMethodNameHelperEnum,
    ): CancelablePromise<JsonRpcResponse_for_RpcTransactionResponse_and_RpcError> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/broadcast_tx_commit',
            body: requestBody,
            mediaType: 'application/json',
        });
    }
    /**
     * @param requestBody
     * @returns JsonRpcResponse_for_RpcChunkResponse_and_RpcError
     * @throws ApiError
     */
    public static chunk(
        requestBody: JsonRpcRequest_for_ChunkMethodNameHelperEnum,
    ): CancelablePromise<JsonRpcResponse_for_RpcChunkResponse_and_RpcError> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/chunk',
            body: requestBody,
            mediaType: 'application/json',
        });
    }
    /**
     * @param requestBody
     * @returns JsonRpcResponse_for_RpcClientConfigResponse_and_RpcError
     * @throws ApiError
     */
    public static clientConfig(
        requestBody: JsonRpcRequest_for_ClientConfigMethodNameHelperEnum,
    ): CancelablePromise<JsonRpcResponse_for_RpcClientConfigResponse_and_RpcError> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/client_config',
            body: requestBody,
            mediaType: 'application/json',
        });
    }
    /**
     * @param requestBody
     * @returns JsonRpcResponse_for_RpcGasPriceResponse_and_RpcError
     * @throws ApiError
     */
    public static gasPrice(
        requestBody: JsonRpcRequest_for_GasPriceMethodNameHelperEnum,
    ): CancelablePromise<JsonRpcResponse_for_RpcGasPriceResponse_and_RpcError> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/gas_price',
            body: requestBody,
            mediaType: 'application/json',
        });
    }
    /**
     * @param requestBody
     * @returns JsonRpcResponse_for_Nullable_RpcHealthResponse_and_RpcError
     * @throws ApiError
     */
    public static health(
        requestBody: JsonRpcRequest_for_HealthMethodNameHelperEnum,
    ): CancelablePromise<JsonRpcResponse_for_Nullable_RpcHealthResponse_and_RpcError> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/health',
            body: requestBody,
            mediaType: 'application/json',
        });
    }
    /**
     * @param requestBody
     * @returns JsonRpcResponse_for_RpcLightClientExecutionProofResponse_and_RpcError
     * @throws ApiError
     */
    public static lightClientProof(
        requestBody: JsonRpcRequest_for_LightClientProofMethodNameHelperEnum,
    ): CancelablePromise<JsonRpcResponse_for_RpcLightClientExecutionProofResponse_and_RpcError> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/light_client_proof',
            body: requestBody,
            mediaType: 'application/json',
        });
    }
    /**
     * @param requestBody
     * @returns JsonRpcResponse_for_RpcNetworkInfoResponse_and_RpcError
     * @throws ApiError
     */
    public static networkInfo(
        requestBody: JsonRpcRequest_for_NetworkInfoMethodNameHelperEnum,
    ): CancelablePromise<JsonRpcResponse_for_RpcNetworkInfoResponse_and_RpcError> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/network_info',
            body: requestBody,
            mediaType: 'application/json',
        });
    }
    /**
     * @param requestBody
     * @returns JsonRpcResponse_for_RpcLightClientNextBlockResponse_and_RpcError
     * @throws ApiError
     */
    public static nextLightClientBlock(
        requestBody: JsonRpcRequest_for_NextLightClientBlockMethodNameHelperEnum,
    ): CancelablePromise<JsonRpcResponse_for_RpcLightClientNextBlockResponse_and_RpcError> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/next_light_client_block',
            body: requestBody,
            mediaType: 'application/json',
        });
    }
    /**
     * @param requestBody
     * @returns JsonRpcResponse_for_RpcTransactionResponse_and_RpcError
     * @throws ApiError
     */
    public static sendTx(
        requestBody: JsonRpcRequest_for_SendTxMethodNameHelperEnum,
    ): CancelablePromise<JsonRpcResponse_for_RpcTransactionResponse_and_RpcError> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/send_tx',
            body: requestBody,
            mediaType: 'application/json',
        });
    }
    /**
     * @param requestBody
     * @returns JsonRpcResponse_for_RpcStatusResponse_and_RpcError
     * @throws ApiError
     */
    public static status(
        requestBody: JsonRpcRequest_for_StatusMethodNameHelperEnum,
    ): CancelablePromise<JsonRpcResponse_for_RpcStatusResponse_and_RpcError> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/status',
            body: requestBody,
            mediaType: 'application/json',
        });
    }
    /**
     * @param requestBody
     * @returns JsonRpcResponse_for_RpcTransactionResponse_and_RpcError
     * @throws ApiError
     */
    public static tx(
        requestBody: JsonRpcRequest_for_TxMethodNameHelperEnum,
    ): CancelablePromise<JsonRpcResponse_for_RpcTransactionResponse_and_RpcError> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/tx',
            body: requestBody,
            mediaType: 'application/json',
        });
    }
    /**
     * @param requestBody
     * @returns JsonRpcResponse_for_RpcValidatorResponse_and_RpcError
     * @throws ApiError
     */
    public static validators(
        requestBody: JsonRpcRequest_for_ValidatorsMethodNameHelperEnum,
    ): CancelablePromise<JsonRpcResponse_for_RpcValidatorResponse_and_RpcError> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/validators',
            body: requestBody,
            mediaType: 'application/json',
        });
    }
}
