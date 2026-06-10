RUST_TOOLCHAIN_NIGHTLY = $(shell toml get ./Cargo.toml workspace.metadata.toolchains.nightly)
SOLANA_CLI_VERSION = $(shell toml get ./Cargo.toml workspace.metadata.cli.solana)

nightly = +${RUST_TOOLCHAIN_NIGHTLY}

# This is a bit tricky -- findstring returns the found string, so we're looking
# for "directory-", returning that, and replacing "-" with "/" to change the
# first "-" to a "/". But if it isn't found, we replace "" with "", which works
# in the case where there is no subdirectory.
pattern-dir = $(firstword $(subst -, ,$1))
find-pattern-dir = $(findstring $(call pattern-dir,$1)-,$1)
make-path = $(subst $(call find-pattern-dir,$1),$(subst -,/,$(call find-pattern-dir,$1)),$1)

rust-toolchain-nightly:
	@echo ${RUST_TOOLCHAIN_NIGHTLY}

solana-cli-version:
	@echo ${SOLANA_CLI_VERSION}

cargo-nightly:
	cargo $(nightly) $(ARGS)

audit:
	cargo audit $(ARGS)

spellcheck:
	cargo spellcheck --code 1 $(ARGS)

clippy-%:
	cargo $(nightly) clippy --manifest-path $(call make-path,$*)/Cargo.toml \
		--all-targets \
		--all-features \
		-- \
		--deny=warnings \
		--deny=clippy::arithmetic_side_effects $(ARGS)

format-check-js-%:
	cd $(call make-path,$*) && pnpm install && pnpm format $(ARGS)

format-check-%:
	cargo $(nightly) fmt --check --manifest-path $(call make-path,$*)/Cargo.toml $(ARGS)

powerset-%:
	cargo $(nightly) hack check --feature-powerset --all-targets --manifest-path $(call make-path,$*)/Cargo.toml $(ARGS)

format-rust:
	cargo $(nightly) fmt --all $(ARGS)

build-sbf-%:
	cargo build-sbf --manifest-path $(call make-path,$*)/Cargo.toml $(ARGS)

build-doc-%:
	RUSTDOCFLAGS="--cfg docsrs -D warnings" cargo $(nightly) doc --all-features --no-deps --manifest-path $(call make-path,$*)/Cargo.toml $(ARGS)

test-doc-%:
	cargo $(nightly) test --doc --all-features --manifest-path $(call make-path,$*)/Cargo.toml $(ARGS)

# The JS client tests run against a local validator (the loader-v4 program is
# a validator builtin, so no `.so` is loaded).
test-js-%:
	make restart-test-validator
	cd $(call make-path,$*) && pnpm install && pnpm build && pnpm test $(ARGS)
	make stop-test-validator

test-%:
	SBF_OUT_DIR=$(PWD)/target/deploy cargo test --manifest-path $(call make-path,$*)/Cargo.toml $(ARGS)

restart-test-validator:
	./scripts/restart-test-validator.sh

stop-test-validator:
	pkill -f solana-test-validator

lint-js-%:
	cd $(call make-path,$*) && pnpm install && pnpm lint $(ARGS)

generate-clients:
	pnpm codama run --all $(ARGS)

# Helpers for publishing
tag-name = $(lastword $(subst /, ,$(call make-path,$1)))
preid-arg = $(subst pre,--preid $2,$(findstring pre,$1))
package-version = $(subst ",,$(shell jq -r '.version' $(call make-path,$1)/package.json))
crate-version = $(subst ",,$(shell toml get $(call make-path,$1)/Cargo.toml package.version))

git-tag-js-%:
	@echo "$(call tag-name,$*)@v$(call package-version,$*)"

publish-js-%:
	cd "$(call make-path,$*)" && pnpm install && pnpm version $(LEVEL) --no-git-tag-version  $(call preid-arg,$(LEVEL),$(TAG)) && pnpm publish --no-git-checks --tag $(TAG)

git-tag-rust-%:
	@echo "$(call tag-name,$*)@v$(call crate-version,$*)"

publish-rust-dry-run-%:
	cd "$(call make-path,$*)" && cargo release $(LEVEL) --tag-name "$(call tag-name,$*)@v{{version}}"

publish-rust-%:
	cd "$(call make-path,$*)" && cargo release $(LEVEL) --tag-name "$(call tag-name,$*)@v{{version}}" --execute --no-confirm --dependent-version fix
