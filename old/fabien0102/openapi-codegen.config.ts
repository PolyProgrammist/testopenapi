import {
  generateSchemaTypes,
  generateFetchers,
} from "@openapi-codegen/typescript";
import { defineConfig } from "@openapi-codegen/cli";
export default defineConfig({
  kekus: {
    from: {
      relativePath: "../testokplain/transaction.yaml",
      source: "file",
    },
    outputDir: "kekus_f",
    to: async (context) => {
      const filenamePrefix = "kekus";
      const { schemasFiles } = await generateSchemaTypes(context, {
        filenamePrefix,
      });
      await generateFetchers(context, {
        filenamePrefix,
        schemasFiles,
      });
    },
  },
});
