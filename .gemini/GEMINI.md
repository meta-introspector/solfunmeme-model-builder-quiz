## Gemini Added Memories for `solfunmeme-model-builder-quiz`

- This project is a standalone spinoff from the main `ragit` repository.
- Its primary purpose is to provide a command-line interface for quizzing on and updating term embeddings.
- The core functionality includes:
    - Starting an integrated quiz server.
    - Taking quiz questions.
    - Submitting answers to update the model's embeddings and weights.
- The model improvement happens through `update_weight` (for question frequency) and `update_embedding` (for refining term representations).
- Current commands: `start`, `take`, `answer`.
- The `term_embeddings.json` file is crucial for the model's data.
- The project uses `clap` for CLI, `axum` for the integrated server, `tokio` for async, `reqwest` for client HTTP, `serde` for serialization, and `rand` for randomness.
- The `daemonize` dependency was removed for simplicity in this standalone version.
- The `SuggestTerms`, `Train`, `Query`, and `AddVector` commands from the original `ragit-embedding-quiz` were removed in this simplified version.
- The `ragit_embedding_quiz_sop.md` is located in `docs/sops/`.
