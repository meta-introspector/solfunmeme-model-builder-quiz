# SOP: Session Recovery

## 1. Purpose
This SOP outlines the standard procedure for recovering the context of a previous work session.

## 2. Scope
This procedure applies to any team member returning to a project after a break.

## 3. Procedure

1.  **Check Git History**: The first step is to review the recent commit history. The following command is recommended to see the last 3 commits across all branches, including the code changes (patches).
    ```bash
    git log --patch -3 --all
    ```

2.  **Review Gemini Memories**: If you are using the Gemini CLI, check its memories for any stored context about the project.

3.  **Synthesize and Plan**: Combine the information from the Git history and Gemini's memories to reconstruct the last task. Formulate a plan to continue the work.

4.  **Clarify if Needed**: If the context is still unclear, do not hesitate to ask for clarification from other team members or in the project's communication channel.

## 4. Tools
*   Git
*   Gemini CLI (optional)
