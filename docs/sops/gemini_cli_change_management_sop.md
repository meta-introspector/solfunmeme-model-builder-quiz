# SOP: Gemini CLI Change Management

## 1. Purpose
This SOP defines a structured and auditable process for development tasks performed by the Gemini CLI agent to ensure consistency and quality.

## 2. Scope
This procedure applies to all development and refactoring tasks executed by the Gemini CLI agent within this project.

## 3. Phases

The change management process is divided into three distinct phases:

### Phase 1: Proposal and Planning

1.  **Define Objective:** Clearly state the goal of the task.
2.  **Analyze Codebase:** Use available tools to understand the relevant parts of the codebase.
3.  **Formulate Plan:** Create a step-by-step plan to achieve the objective.
4.  **User Approval:** Present the plan to the user for approval before proceeding.

### Phase 2: Implementation

1.  **Branching:** Create a dedicated `feature` or `bugfix` branch for the changes, following the `branching_strategy_sop.md`.
2.  **Execution:** Execute the plan using the available tools, making changes to the codebase.
3.  **Save Drafts:** For complex changes, save intermediate work to prevent data loss.

### Phase 3: Verification and Commit

1.  **Verification:** Run tests, linting, and builds to ensure the changes are correct and adhere to project standards.
2.  **Staging:** Stage the modified files for commit.
3.  **Commit Message:** Create a clear and descriptive commit message following the Conventional Commits specification.
4.  **Commit:** Commit the changes to the repository.
