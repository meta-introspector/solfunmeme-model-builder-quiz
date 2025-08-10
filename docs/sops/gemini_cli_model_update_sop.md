# SOP: Gemini CLI Model Update Procedure

## 1. Objective

To define a clear, structured, and auditable process for using the Gemini CLI agent to perform **model updates** within the `ragit` project, specifically focusing on the `solfunmeme-model-builder-quiz` tool and its underlying term embeddings. This SOP ensures all model-related actions align with our established change management and quality assurance procedures.

## 2. Scope

This SOP applies to all development tasks performed by the Gemini CLI agent that involve modifying or interacting with the project's semantic models, particularly the term embeddings managed by the `solfunmeme-model-builder-quiz` tool.

## 3. Procedure

This procedure builds upon the general principles outlined in the **SOP: Using Gemini CLI with Structured Change Management** (`docs/quality_procedures/gemini_cli_change_management_sop.md`). The following steps provide model-specific considerations.

### Phase 1: Proposal and Planning (Model-Specific)

1.  **Define the Objective:** Clearly articulate the desired model update (e.g., "refine embeddings for X terms," "adjust weights for Y questions," "integrate new embedding data").
2.  **Consult Existing Documentation:**
    *   Review `docs/sops/ragit_embedding_quiz_sop.md` for detailed usage of the `model-builder-quiz` tool.
    *   Consult `spinoffs/model-builder-quiz/.gemini/GEMINI.md` for specific memories related to the quiz tool.
3.  **Analyze the Model and Data Sources:**
    *   Understand the current state of the model (e.g., by inspecting `term_embeddings.json`).
    *   Identify the specific terms or questions affected by the update.
    *   Analyze the impact of `update_weight` and `update_embedding` operations.
4.  **Formulate a Written Plan:** Detail the steps for the model update, including:
    *   Which `model-builder-quiz` commands will be used (`start`, `take`, `answer`, `stop`).
    *   Expected changes to `term_embeddings.json`.
    *   Verification steps to confirm the model update.

### Phase 2: Implementation (Model-Specific)

1.  **Execute the Plan:**
    *   Use `model-builder-quiz start` to initiate the integrated server.
    *   Interact with the model using `model-builder-quiz take` to retrieve questions.
    *   Apply updates using `model-builder-quiz answer <ID> "<EMBEDDING>"` to refine embeddings and adjust weights.
    *   Ensure `term_embeddings.json` is correctly modified by the tool.
    *   Use `model-builder-quiz stop` when interactions are complete.

### Phase 3: Verification and Commit (Model-Specific)

1.  **Verify Model Changes:**
    *   After interaction, re-examine `term_embeddings.json` to confirm expected changes.
    *   Run `model-builder-quiz take` again to observe if question selection or reported embeddings reflect the updates.
    *   Perform manual integration tests as outlined in `spinoffs/model-builder-quiz/docs/how_to_test.md`.
2.  **Stage Changes:** Stage `term_embeddings.json` and any other modified files.
3.  **Create Commit Message:** Clearly describe the model update, including the terms affected and the nature of the changes (e.g., "Refine embeddings for 'X' and 'Y' based on quiz feedback").
4.  **Commit Changes:** Commit the changes following the general change management SOP.

## 4. Best Practices for Model Updates

*   **Iterative Refinement:** Model updates are often an iterative process. Make small, focused changes and verify their impact.
*   **Backup Data:** Before significant updates, consider backing up `term_embeddings.json`.
*   **Understand Impact:** Be aware that `answer` commands directly modify `term_embeddings.json` and affect subsequent quiz question selection.

## 5. Related Documentation

*   **SOP: Using Gemini CLI with Structured Change Management:** `docs/quality_procedures/gemini_cli_change_management_sop.md`
*   **SOP: Ragit Embedding Quiz CLI Tool:** `docs/sops/ragit_embedding_quiz_sop.md`
*   **How to Use `solfunmeme-model-builder-quiz`:** `spinoffs/model-builder-quiz/docs/how_to_use.md`
*   **How to Test `solfunmeme-model-builder-quiz`:** `spinoffs/model-builder-quiz/docs/how_to_test.md`
