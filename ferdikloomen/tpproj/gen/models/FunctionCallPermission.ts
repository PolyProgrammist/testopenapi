/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
/**
 * Grants limited permission to make transactions with FunctionCallActions The permission can limit the allowed balance to be spent on the prepaid gas. It also restrict the account ID of the receiver for this function call. It also can restrict the method name for the allowed function calls.
 */
export type FunctionCallPermission = {
    /**
     * Allowance is a balance limit to use by this access key to pay for function call gas and transaction fees. When this access key is used, both account balance and the allowance is decreased by the same value. `None` means unlimited allowance. NOTE: To change or increase the allowance, the old access key needs to be deleted and a new access key should be created.
     */
    allowance: string;
    /**
     * A list of method names that can be used. The access key only allows transactions with the function call of one of the given method names. Empty list means any method name can be used.
     */
    method_names: Array<string>;
    /**
     * The access key only allows transactions with the given receiver's account id.
     */
    receiver_id: string;
};

