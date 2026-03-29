import { defineConfig } from "vite";
import dts from "vite-plugin-dts";
import { resolve } from "path";

export default defineConfig({
    build: {
        outDir: "dist",
        emptyOutDir: true,

        lib: {
            entry: {
                shared: resolve(__dirname, "pb/shared.ts"),
                aquila: resolve(__dirname, "pb/aquila.ts"),
                sagittarius: resolve(__dirname, "pb/sagittarius.ts"),
                helpers: resolve(__dirname, "helpers/helpers.ts"),
            },
            formats: ["es", "cjs"],
            fileName: (format, entryName) => `${entryName}.${format}.js`,
        },

        rollupOptions: {
            // Explicit inputs make multi-entry builds reliable
            input: {
                shared: resolve(__dirname, "pb/shared.ts"),
                aquila: resolve(__dirname, "pb/aquila.ts"),
                sagittarius: resolve(__dirname, "pb/sagittarius.ts"),
                helpers: resolve(__dirname, "helpers/helpers.ts"),
            },

            external: [],

            output: {
                exports: "named",
            },
        },
    },

    plugins: [
        dts({
            // Use project root since your entries are outside /src
            entryRoot: ".",

            outDir: "dist",

            insertTypesEntry: true,

            // Include both wrappers and generated protobuf code
            include: [
                "pb/**/*.ts",
                "pb/_generated/**/*.ts",
                "helpers/**/*.ts",
            ],
            afterDiagnostic: diagnostics => {
                if (diagnostics.length > 0) {
                    throw new Error("dts failed");
                }
            }
        }),
    ],

    resolve: {
        alias: {
            "@": resolve(__dirname, "src"),
        },
    },
});