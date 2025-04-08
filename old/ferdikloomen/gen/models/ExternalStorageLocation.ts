/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
export type ExternalStorageLocation = ({
    S3: {
        /**
         * Location of state dumps on S3.
         */
        bucket: string;
        /**
         * Data may only be available in certain locations.
         */
        region: string;
    };
} | {
    Filesystem: {
        root_dir: string;
    };
} | {
    GCS: {
        bucket: string;
    };
});

