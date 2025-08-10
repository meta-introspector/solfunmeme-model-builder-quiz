# SOP: Ragit Embedding Quiz CLI Tool

## 1. Purpose

To provide a command-line interface for quizzing on and updating term embeddings within the `ragit` project. This tool allows for interactive feedback on embedding quality and facilitates iterative model improvement.

## 2. Scope

This SOP applies to all interactions with the `ragit-embedding-quiz` CLI tool.

## 3. Procedure

### 3.1. Getting a Quiz Question

To get a quiz question (a term and its current embedding), run the `quiz` subcommand:

```bash
cargo run --package ragit-embedding-quiz quiz
```

The tool will output the question ID, the term (Question Text), and its current 8-dimensional embedding:

```
Question ID: 135
Question Text: 14935
Current Embedding: [0.1234, 0.5678, 0.9012, 0.3456, 0.789, 0.1234, 0.5678, 0.9012]
```

### 3.2. Submitting an Answer (Updating Embedding)

To submit an answer (an updated embedding for a given question ID), run the `answer` subcommand, providing the `question_id` and the new 8-dimensional embedding as a comma-separated string:

```bash
cargo run --package ragit-embedding-quiz answer <question_id> <comma_separated_embedding_values>
```

**Example:**

```bash
cargo run --package ragit-embedding-quiz answer 135 0.1,0.5,0.9,0.3,0.7,0.1,0.5,0.9
```

The tool will report whether the submitted embedding was considered 'correct' (based on a Euclidean distance threshold of 0.1 from the original embedding) and if the embedding was updated:

```
Answer submitted for Question ID: 135
Correct: false
Embedding updated.
```

If the answer is `false` (i.e., the submitted embedding is significantly different from the original), the embedding in `term_embeddings.json` will be updated by averaging the original and submitted embeddings. The `weights` for the question (quizzing frequency) are also adjusted.

## 4. Tools

*   `ragit-embedding-quiz` CLI tool

## 5. Expected Outcome

A functional command-line interface for interactively quizzing on and updating term embeddings, facilitating the iterative refinement of the project's semantic models.