# SOP: Branching Strategy

## 1. Purpose
This document outlines the branching strategy for this project to ensure a consistent and stable development process.

## 2. Scope
This SOP applies to all code contributions to this repository.

## 3. Core Branches

*   `main`: This is the primary branch. It represents the latest stable, production-ready version of the codebase. Direct commits to `main` are not allowed.

## 4. Development Branches

All new work, including features and bug fixes, must be done in separate branches.

### Feature Branches

*   **Naming Convention:** `feature/<feature-name>` (e.g., `feature/add-user-authentication`)
*   **Creation:** Branched from the latest `main`.
*   **Purpose:** For developing new features.

### Bugfix Branches

*   **Naming Convention:** `bugfix/<issue-id>-<short-description>` (e.g., `bugfix/42-fix-login-button`)
*   **Creation:** Branched from the latest `main`.
*   **Purpose:** For fixing bugs found in the `main` branch.

## 5. Workflow

1.  **Create a Branch:** Before starting work, create a new `feature` or `bugfix` branch from the `main` branch.
2.  **Develop:** Make your changes in the new branch. Commit your work with clear, descriptive messages.
3.  **Push:** Push your branch to the remote repository.
4.  **Pull Request (PR):** Open a Pull Request from your branch to the `main` branch.
5.  **Review:** At least one other team member must review and approve the PR.
6.  **Merge:** Once approved, the PR can be merged into `main`.

## 6. Commit Messages

While not strictly enforced, we recommend following the [Conventional Commits](https://www.conventionalcommits.org/) specification. This helps in creating a more readable and structured commit history.

**Example:**

```
feat: add user profile page

Users can now view and edit their profiles.
```
