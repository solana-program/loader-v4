import { execSync } from "node:child_process";

const nightly = execSync("make --no-print-directory rust-toolchain-nightly")
  .toString()
  .trim();

const prettierOptions = {
  arrowParens: "avoid",
  printWidth: 120,
  singleQuote: true,
  tabWidth: 4,
  trailingComma: "all",
};

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
