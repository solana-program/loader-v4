import { execSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";

const nightly = execSync("make --no-print-directory rust-toolchain-nightly")
  .toString()
  .trim();

const prettierOptions = JSON.parse(
  fs.readFileSync(path.join("clients", "js", ".prettierrc.json"), "utf-8")
);

export default {
  idl: "idl.json",
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
          toolchain: `+${nightly}`,
        },
      ],
    },
  },
};
