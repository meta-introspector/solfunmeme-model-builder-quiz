# How to Use `solfunmeme-model-builder-quiz`

This document provides a comprehensive guide on how to effectively use the `solfunmeme-model-builder-quiz` command-line interface (CLI) tool.

## Overview

The `model-builder-quiz` tool is designed for interactive refinement of term embeddings. It operates with an integrated server that manages quiz questions, tracks their performance, and updates their underlying embeddings and weights based on user feedback.

## Commands

The tool provides the following commands:

*   `start`: Starts the integrated quiz server.
*   `stop`: Stops the running quiz server.
*   `take`: Requests a new quiz question from the server.
*   `answer`: Submits an answer to a quiz question, providing feedback to the model.

### `start` Command

Starts the integrated quiz server. This server is essential for the `take` and `answer` commands to function.

**Usage:**

```bash
./target/release/model-builder-quiz start
```

**Important Notes:**

*   The server runs in the foreground by default. To run it in the background and continue using your terminal, append `&` to the command:
    ```bash
    ./target/release/model-builder-quiz start &
    ```
*   For persistent backgrounding (e.g., after closing your terminal), consider using `nohup`:
    ```bash
    nohup ./target/release/model-builder-quiz start > /dev/null 2>&1 &
    ```
*   If you encounter an "Address already in use" error, it means a previous instance of the server is still running or the port hasn't been released. Use the `stop` command to try and shut it down, or wait a moment.

### `stop` Command

Stops the running quiz server.

**Usage:**

```bash
./target/release/model-builder-quiz stop
```

This command sends a shutdown signal to the server.

### `take` Command

Requests a new quiz question from the server. The server selects a question based on its internal weighting system (questions answered incorrectly or less frequently might appear more often).

**Usage:**

```bash
./target/release/model-builder-quiz take
```

**Output:**

The command will output the `Question ID`, `Question Text`, and the `Current Embedding` of the selected question.

```
Question ID: <ID>
Question Text: <TERM_TEXT>
Current Embedding: [<EMBEDDING_VECTOR>]
```

### `answer` Command

Submits your answer to a quiz question. Your answer consists of the `Question ID` and your proposed `submitted_embedding` for that term.

**Usage:**

```bash
./target/release/model-builder-quiz answer <QUESTION_ID> "<COMMA_SEPARATED_EMBEDDING>"
```

*   `<QUESTION_ID>`: The numerical ID of the question you are answering (obtained from the `take` command).
*   `<COMMA_SEPARATED_EMBEDDING>`: Your proposed 8-dimensional embedding for the term, as a comma-separated string of floating-point numbers (e.g., "0.1,0.2,0.3,0.4,0.5,0.6,0.7,0.8").

**How it Works (Model Improvement):**

When you submit an answer, the server performs the following:

1.  **Distance Calculation:** It calculates the Euclidean distance between the question's current embedding and your `submitted_embedding`.
2.  **Correctness Check:** If the distance is below a certain threshold (currently 0.1), your answer is considered "correct."
3.  **Weight Adjustment:**
    *   If "correct," the question's internal weight is increased, making it less likely to be chosen for future quizzes.
    *   If "incorrect," the question's internal weight is decreased, making it more likely to be re-quizzed.
4.  **Embedding Update (for incorrect answers):** If your answer is "incorrect," the question's current embedding is updated by averaging it with your `submitted_embedding`. This helps the model learn from your feedback and refine its representation of the term.

**Output:**

The command will report whether your answer was `Correct: true` or `Correct: false`, and if the embedding was updated.

```
Answer submitted for Question ID: <ID>
Correct: <true/false>
Embedding updated. (Only if Correct: false)
```

## Example Workflow

1.  **Start the server:**
    ```bash
    ./target/release/model-builder-quiz start &
    ```
2.  **Get a question:**
    ```bash
    ./target/release/model-builder-quiz take
    # Output: Question ID: 42, Question Text: "example_term", Current Embedding: [...]
    ```
3.  **Answer the question (with a new embedding):**
    ```bash
    ./target/release/model-builder-quiz answer 42 "0.1,0.1,0.1,0.1,0.1,0.1,0.1,0.1"
    # Output: Correct: false, Embedding updated.
    ```
4.  **Stop the server:**
    ```bash
    ./target/release/model-builder-quiz stop
    ```
