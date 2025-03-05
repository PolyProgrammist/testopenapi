const NEAR_RPC_URL: string = "https://archival-rpc.mainnet.near.org";
const transactionHash: string = "9FtHUFBQsZ2MG77K3x3MJ9wjX3UT8zE1TczCrhZEcG8U"; // Replace with your TX hash
const senderAccountId: string = "miraclx.near"; // Replace with sender's account

import { ActionError } from "./gen";

interface JsonRpcResponse {
    result?: any;
    error?: any;
}

async function fetchTransaction(): Promise<void> {
    const payload = {
        jsonrpc: "2.0",
        id: "dontcare",
        method: "tx",
        params: [transactionHash, senderAccountId]
    };

    try {
        const response = await fetch(NEAR_RPC_URL, {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify(payload)
        });

        const data: JsonRpcResponse = await response.json();
        
        if (data.result) {
            console.log("Transaction result:", data.result);
        } else {
            console.error("Error fetching transaction:", data.error);
        }
    } catch (error) {
        console.error("Request failed:", error);
    }
}

fetchTransaction();
