import { getToolchainArgument } from "./scripts/utils.mjs";
import path from "node:path";
import fs from "node:fs";

const prettierOptions = JSON.parse(
  fs.readFileSync(path.join("clients", "js", ".prettierrc.json"), "utf-8")
);

export default {
  idl: "program/idl.json",
  before: [
    {
      from: "codama#updateProgramsVisitor",
      args: [
        {
          solanaLoaderV4Program: {
            name: "loaderV4",
          },
        },
      ],
    },
  ],
  scripts: {
    js: {
      from: "@codama/renderers-js",
      args: ["clients/js", { kitImportStrategy: "rootOnly", prettierOptions }],
    },
    rust: {
      from: "@codama/renderers-rust",
      args: [
        "clients/rust",
        {
          anchorTraits: false,
          formatCode: true,
          toolchain: getToolchainArgument("format"),
        },
      ],
    },
  },
};
