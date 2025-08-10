# Change Request: Refactoring `model-builder-quiz` to Align with `term_quiz_master` and `ragit-command-quiz`

## Date: August 10, 2025

## Originating Request:
The user requested to "review all this code diff --git a/crates/quiz_server/Cargo.toml ... and merge it all to spinoffs/model-builder-quiz". This was followed by a request to "read the code in spinoff, then review each file mentioned and see whats missing and make a written plan to add it, then we review."

## Current State of `spinoffs/model-builder-quiz`:
The `spinoffs/model-builder-quiz` project is a Rust application using `clap` for CLI, `axum` for a web server, `tokio` for async operations, `reqwest` for HTTP requests, `serde` for serialization, and `rand` for randomness. It manages `term_embeddings.json` for its data and provides `start`, `stop`, `take`, `answer`, `insert`, `update`, `delete`, `query`, and `list_terms` commands. It was explicitly created as a spinoff from the larger `ragit` repository, specifically from `ragit-embedding-quiz`.

## Interpretation of User's Intent:
The provided diff paths (e.g., `crates/quiz_server/`, `crates/ragit-command-quiz/`, `crates/ragit-embedding-quiz/`, `crates/term_quiz_master/`) suggest a significant refactoring within the main `ragit` repository. It appears that the original `ragit-embedding-quiz` is being refactored or replaced by `term_quiz_master` (core logic) and `ragit-command-quiz` (CLI commands), with `quiz_server` as a separate server component.

Given that `spinoffs/model-builder-quiz` is a spinoff of `ragit-embedding-quiz`, the interpretation is that the user wishes to bring the `spinoffs/model-builder-quiz` project up to date with this new, refactored structure and functionality. This implies a transformation of the current `model-builder-quiz` to align with the new architecture.

## Proposed High-Level Plan:

This plan assumes we are essentially transforming `spinoffs/model-builder-quiz` to incorporate the new `term_quiz_master` and `ragit-command-quiz` architecture.

### Phase 1: Core Logic Migration (Term Quiz Master)

1.  **Analyze `crates/term_quiz_master` structure:** Examine the file structure and `lib.rs` of `crates/term_quiz_master` (from the main `ragit` repository) to understand its modules and public API.
2.  **Refactor `model_core.rs`:** Split `model_core.rs` into smaller, more granular modules within a new `src/term_quiz_master/` directory, mirroring the structure of `crates/term_quiz_master`. This will likely involve creating separate files for `Question`, `Model` (if it's still a single struct), and potentially `TermEntry`, `AugmentedTermEntry`, `QuizLogic`, etc., as suggested by the diff paths.
3.  **Migrate `handlers.rs` logic:** Move the logic from `handlers.rs` into the appropriate new modules within `src/term_quiz_master/`, ensuring it aligns with the methods exposed by `term_quiz_master`.
4.  **Update `Cargo.toml`:** Add `term_quiz_master` as a dependency (if it's intended to be a separate crate) or adjust the current `Cargo.toml` to reflect the new internal module structure.

### Phase 2: CLI Command Refactoring (Ragit Command Quiz)

1.  **Analyze `crates/ragit-command-quiz` structure:** Examine its `src/main.rs` and `src/cli.rs` (or similar) to understand how commands are defined and handled.
2.  **Refactor `cli.rs`, `cli_run.rs`, and `cli_commands/`:**
    *   Align the `clap` setup in `cli.rs` with `ragit-command-quiz`.
    *   Adjust `cli_run.rs` to dispatch commands to the new `term_quiz_master` logic.
    *   Refactor individual command files in `cli_commands/` to call the appropriate functions from the new `src/term_quiz_master/` modules.
3.  **Remove redundant code:** Eliminate any duplicate or now-obsolete command handling logic.

### Phase 3: Server Component Alignment (Quiz Server)

1.  **Analyze `crates/quiz_server` structure:** Review its `src/main.rs`, `src/api_handlers.rs`, and `src/api_types.rs` to understand its server setup.
2.  **Align `server.rs`, `api_types.rs`, `api_handlers.rs`:** Ensure these files in `spinoffs/model-builder-quiz` are consistent with the `quiz_server`'s implementation, especially regarding API endpoints, data structures, and handler functions.
3.  **Dependency check:** Verify that `axum` and other server-related dependencies are correctly configured.

### Phase 4: Documentation and Cleanup

1.  **Update `README.md`:** Reflect the new architecture, commands, and usage instructions.
2.  **Update `docs/`:**
    *   Review `docs/how_to_test.md` and `docs/how_to_use.md` and update them to reflect the new commands and internal structure.
    *   Address the new `docs/ontology_properties_for_quiz.md` and `docs/sops/quiz_server_sop.md` mentioned in the diff. I will need to read these from the main `ragit` repository to understand their content and decide if they should be copied or adapted.
    *   Remove `docs/sops/ragit_embedding_quiz_sop.md` if it's superseded.
3.  **Remove `src/model.rs`:** Delete this empty file.
4.  **Run `cargo check` and `cargo test`:** Continuously verify compilation and test execution throughout the refactoring process.

## Next Steps:
Upon user approval of this plan, the next action will be to start by examining the `crates/term_quiz_master` and `crates/ragit-command-quiz` directories in the parent `ragit` repository to understand their structure and content. This will involve constructing absolute paths to read those files.
