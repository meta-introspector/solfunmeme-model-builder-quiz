# How to Test `solfunmeme-model-builder-quiz`

This document outlines the procedures for testing the `solfunmeme-model-builder-quiz` tool. Testing ensures the tool's functionality, reliability, and adherence to expected behavior.

## 1. Unit Tests

Unit tests verify individual components or functions of the codebase in isolation.

### Running Unit Tests

To run all unit tests for the `model-builder-quiz` project, navigate to the project root directory and execute the following command:

```bash
cargo test --manifest-path Cargo.toml
```

**Expected Output:**

A successful test run will display output similar to this (though the exact number of tests and their names may vary):

```
running 1 test
test tests::test_calculate_distance ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Any failures will be clearly indicated, along with details about the failing test.

## 2. Integration Tests (Manual)

Integration tests verify the interaction between different components of the system, particularly the CLI commands and the integrated server.

### Prerequisites

*   The `model-builder-quiz` executable built (e.g., `target/release/model-builder-quiz`).

### Test Cases

#### Test Case 1: Server Startup and Status

**Objective:** Verify that the server starts correctly and reports its status.

**Steps:**

1.  **Start the server:**
    ```bash
    ./target/release/model-builder-quiz start &
    ```
2.  **Check server status (in a new terminal):**
    ```bash
    ./target/release/model-builder-quiz status
    ```
    **Expected Output:**
    ```
    Quiz server status: running
    ```
3.  **Stop the server:**
    ```bash
    ./target/release/model-builder-quiz stop
    ```

#### Test Case 2: Taking a Quiz Question

**Objective:** Verify that the `take` command retrieves a question from the server.

**Steps:**

1.  **Start the server:**
    ```bash
    ./target/release/model-builder-quiz start &
    ```
2.  **Take a quiz question:**
    ```bash
    ./target/release/model-builder-quiz take
    ```
    **Expected Output:** A `Question ID`, `Question Text`, and `Current Embedding` should be displayed.
3.  **Stop the server:**
    ```bash
    ./target/release/model-builder-quiz stop
    ```

#### Test Case 3: Answering a Question (Correct)

**Objective:** Verify that submitting a "correct" answer updates the model's weight.

**Steps:**

1.  **Start the server:**
    ```bash
    ./target/release/model-builder-quiz start &
    ```
2.  **Take a quiz question and note its ID and embedding:**
    ```bash
    ./target/release/model-builder-quiz take
    # Example Output: Question ID: 123, Current Embedding: "0.1,0.2,0.3,0.4,0.5,0.6,0.7,0.8"
    ```
3.  **Submit a "correct" answer (use the exact same embedding or one very close):**
    ```bash
    ./target/release/model-builder-quiz answer 123 "0.1,0.2,0.3,0.4,0.5,0.6,0.7,0.8"
    ```
    **Expected Output:**
    ```
    Answer submitted for Question ID: 123
    Correct: true
    ```
4.  **Stop the server:**
    ```bash
    ./target/release/model-builder-quiz stop
    ```

#### Test Case 4: Answering a Question (Incorrect)

**Objective:** Verify that submitting an "incorrect" answer updates the model's embedding and weight.

**Steps:**

1.  **Start the server:**
    ```bash
    ./target/release/model-builder-quiz start &
    ```
2.  **Take a quiz question and note its ID and embedding:**
    ```bash
    ./target/release/model-builder-quiz take
    # Example Output: Question ID: 123, Current Embedding: "0.1,0.2,0.3,0.4,0.5,0.6,0.7,0.8"
    ```
3.  **Submit an "incorrect" answer (use a significantly different embedding):**
    ```bash
    ./target/release/model-builder-quiz answer 123 "0.9,0.8,0.7,0.6,0.5,0.4,0.3,0.2"
    ```
    **Expected Output:**
    ```
    Answer submitted for Question ID: 123
    Correct: false
    Embedding updated.
    ```
4.  **Stop the server:**
    ```bash
    ./target/release/model-builder-quiz stop
    ```

## 3. Code Coverage (Future Work)

Implementing code coverage analysis tools (e.g., `grcov`) can provide insights into which parts of the codebase are exercised by tests and identify areas that need more testing. This is a future enhancement.
<<<<<<< HEAD
=======

## 4. Testing Data: `term_embeddings.json`

The `model-builder-quiz` tool relies on the `term_embeddings.json` file for its question data. This file contains a mapping of terms (words or phrases) to their corresponding 8-dimensional embeddings.

**Location:**

The `term_embeddings.json` file is located in the root directory of the `model-builder-quiz` project:

```
spinoffs/model-builder-quiz/term_embeddings.json
```

**Structure:**

The file is a JSON object where keys are terms (strings) and values are arrays of 8 floating-point numbers (the embeddings).

```json
{
  "example_term_1": [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8],
  "example_term_2": [0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2, 0.1],
  // ... more terms
}
```

**Role in Testing:**

*   **Question Source:** The `Model` struct loads questions directly from this file. Therefore, the content of `term_embeddings.json` directly influences the questions presented during quizzes.
*   **Embedding Updates:** When you use the `answer` command, the embeddings within this file are updated based on your feedback.
*   **CRUD Operations:** The `insert`, `update`, and `delete` commands directly modify the contents of this file.

**Modifying Test Data:**

You can manually edit `term_embeddings.json` to add, modify, or remove terms for testing purposes. After manual edits, ensure the JSON format remains valid. The `model-builder-quiz` tool will load the updated data the next time it starts.
>>>>>>> 67ef160 (wip)
