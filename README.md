# Solfunmeme Model Builder Quiz

`solfunmeme-model-builder-quiz` is a standalone command-line interface (CLI) tool designed to help you interactively quiz on and refine term embeddings. It integrates a lightweight quiz server and provides commands to take quizzes and submit answers, which in turn updates the underlying model's understanding of term relationships.

This project is a spinoff from the larger `ragit` repository, specifically evolving from the `ragit-embedding-quiz` crate and now aligning with the core logic of `term_quiz_master` and the CLI interaction patterns of `ragit-command-quiz`.

## Status

This project is under active development. We have recently refactored the core logic to align with `term_quiz_master` and enhanced the CLI to incorporate a broader range of term management functionalities.

## Features

*   **Integrated Quiz Server:** A built-in server handles quiz logic and model updates.
*   **Interactive Quizzing:** Request new quiz questions directly from the CLI.
*   **Model Improvement:** Submit answers to update question weights and refine term embeddings based on your feedback.
*   **Multi-Search:** Query for multiple terms at once to see both individual and combined embedding results.
*   **Comprehensive Term Management:** Directly insert, update, delete, and list terms and their embeddings via CLI commands.

## Getting Started

### Prerequisites

*   [Rust](https://www.rust-lang.org/tools/install) and Cargo (Rust's package manager) installed.

### Building the Project

Navigate to the project root and build the executable:

```bash
cargo build --release --manifest-path Cargo.toml
```

This will create an optimized executable at `target/release/model-builder-quiz`.

## Usage

The `model-builder-quiz` tool operates with an integrated server. You need to start this server first, and then you can interact with it using other commands.

### 1. Start the Quiz Server

The server runs in the background and handles all quiz logic and model updates.

```bash
./target/release/model-builder-quiz start &
```

**Note:** The `&` at the end runs the server in the background, allowing you to continue using your terminal. For persistent backgrounding, consider using `nohup` (e.g., `nohup ./target/release/model-builder-quiz start > /dev/null 2>&1 &`).

### 2. Take a Quiz Question

Once the server is running, you can request a new quiz question:

```bash
./target/release/model-builder-quiz take
```

The output will provide a `Question ID`, `Question Text`, and its `Current Embedding`.

### 3. Answer a Question

To improve the model, you submit an answer with a new embedding for a given question. The model adjusts its internal weights and embeddings based on your response.

```bash
./target/release/model-builder-quiz answer <QUESTION_ID> "<COMMA_SEPARATED_EMBEDDING>"
```

**Example:**

```bash
./target/release/model-builder-quiz answer 123 "0.1,0.2,0.3,0.4,0.5,0.6,0.7,0.8"
```

The tool will report whether your answer was considered "correct" (based on a distance threshold) and if the embedding was updated.

### 4. Query for Terms (Multi-Search)

You can query for one or more terms to see their embeddings and find similar terms.

```bash
./target/release/model-builder-quiz query <TERM_1> <TERM_2> ...
```

**Example:**

```bash
./target/release/model-builder-quiz query code fix
```

This will show the individual embeddings for "code" and "fix", as well as a combined embedding for both terms, and a list of similar terms for each.

### 5. Manage Terms (CRUD Operations)

You can directly manage terms and their embeddings:

#### Insert a new term:
```bash
./target/release/model-builder-quiz insert <TERM> "<COMMA_SEPARATED_EMBEDDING>"
```
**Example:**
```bash
./target/release/model-builder-quiz insert "new_concept" "0.9,0.8,0.7,0.6,0.5,0.4,0.3,0.2"
```

#### Update an existing term's embedding:
```bash
./target/release/model-builder-quiz update <TERM_ID> "<NEW_COMMA_SEPARATED_EMBEDDING>"
```
**Example:**
```bash
./target/release/model-builder-quiz update 123 "0.1,0.1,0.1,0.1,0.1,0.1,0.1,0.1"
```

#### Delete a term:
```bash
./target/release/model-builder-quiz delete <TERM_ID>
```
**Example:**
```bash
./target/release/model-builder-quiz delete 123
```

#### List all terms:
```bash
./target/release/model-builder-quiz list-terms
```

### 6. Stop the Quiz Server

When you are finished, you can stop the running server:

```bash
./target/release/model-builder-quiz stop
```

## Model Improvement

The model improves through your interactions:

*   **Weight Adjustment:** Correct answers increase a question's weight, making it less likely to be asked again. Incorrect answers decrease its weight, making it more likely to be re-quizzed.
*   **Embedding Refinement:** If an answer is incorrect, the question's embedding is updated by averaging it with your submitted embedding, guiding the model towards a more accurate representation.

## Testing

To run the tests for `solfunmeme-model-builder-quiz`:

```bash
cargo test --manifest-path Cargo.toml
```

## Documentation

For more detailed information, refer to the `docs/` directory:

*   `docs/sops/quiz_server_sop.md`: Standard Operating Procedure for the quiz server.
*   `docs/ontology_properties_for_quiz.md`: Ontology properties relevant to quiz augmentation.
*   `docs/testing_data.md`: Documentation for the training data used in this project.
*   `docs/how_to_use.md`: Detailed usage guide (to be created).
*   `docs/how_to_test.md`: Detailed testing guide (to be created).

---

## Next Steps

The core refactoring to align with `term_quiz_master` and `ragit-command-quiz` is complete. The next steps for this project are:

1.  **Verify Functionality:** Run `cargo check` and `cargo test` to ensure all changes compile and existing tests pass.
2.  **Test New Commands:** Manually test the newly exposed `insert`, `update`, `delete`, and `list-terms` commands.
3.  **Integrate `term_embeddings.json` and `training_data_v2.json`:** Restore these files as per the `current_task.md` plan.
4.  **Integrate `ontologies/hyperspace.ttl`:** Restore this file as per the `current_task.md` plan.
5.  **Complete Documentation:** Finish `docs/how_to_use.md` and `docs/how_to_test.md`.
6.  **Consider Daemonization:** Implement or refine the daemonization strategy for the server, potentially aligning with `quiz_server`'s approach.