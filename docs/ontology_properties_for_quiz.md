# Ontology Properties for Quiz Augmentation

This document outlines the ontology properties identified from `numberology.ttl`, `index.ttl`, and `zos/v1.ttl` that are intended to augment the `AugmentedTermEntry` in the `term_quiz_master` crate. These properties will be manually added to the `augmented_terms_hot_take.json` file.

## Mapping to `AugmentedTermEntry` Fields

The `AugmentedTermEntry` struct has been updated to include the following fields:

```rust
pub struct AugmentedTermEntry {
    // ... existing fields ...
    pub emoji_representation: Option<String>,
    pub semantic_names: Option<Vec<String>>,
    pub osi_layer: Option<String>,
    pub prime_factor: Option<usize>,
    pub is_power_of_two: Option<bool>,
    pub numerical_address: Option<usize>,
}
```

Here's how these fields map to the ontology properties:

### 1. `emoji_representation`

*   **Source Ontology:** `vendor/meta-introspector/solfunmeme-dioxus/ontologies/index.ttl` and `vendor/meta-introspector/solfunmeme-dioxus/ontologies/zos/v1.ttl`
*   **Property:** `em:hasEmojiRepresentation`
*   **Description:** This property provides a visual emoji representation for a concept or crate.
*   **Example Usage:**
    *   From `index.ttl`: `crates_root:bootstrapCrate em:hasEmojiRepresentation "🚀🌳" .`
    *   From `zos/v1.ttl`: `em:puzzle_piece a em:Emoji ; em:utf "U+1F9E9" .` (Note: `em:utf` might be used to derive the actual emoji character if `em:hasEmojiRepresentation` is not directly available for all terms).

### 2. `semantic_names`

*   **Source Ontology:** `vendor/meta-introspector/solfunmeme-dioxus/ontologies/index.ttl` and `vendor/meta-introspector/solfunmeme-dioxus/ontologies/zos/v1.ttl`
*   **Property:** `vibe:hasSemanticName` and `rdfs:label`
*   **Description:** A list of semantic names or conceptual labels associated with a term, crate, or vibe layer.
*   **Example Usage:**
    *   From `index.ttl`: `crates_root:bootstrapCrate rdfs:label "Bootstrap Crate" .`
    *   From `zos/v1.ttl`: `vibe:EmacsProject vibe:hasSemanticName "Extensible Editor", "Lisp Machine", "Self-Documenting", "Integrated Development Environment" .`
    *   From `numberology.ttl`: `num:DualityBinaryFundamentalChoices rdfs:label "Duality, Binary, Fundamental Choices" .`

### 3. `osi_layer`

*   **Source Ontology:** `vendor/meta-introspector/solfunmeme-dioxus/ontologies/zos/v1.ttl`
*   **Property:** `vibe:hasOSILayer`
*   **Description:** The conceptual OSI layer associated with a `vibe:Layer`.
*   **Example Usage:**
    *   From `zos/v1.ttl`: `vibe:Layer1 a vibe:Layer ; vibe:hasOSILayer vibe:PhysicalLayer .`

### 4. `prime_factor`

*   **Source Ontology:** `ontologies/numberology.ttl` and `vendor/meta-introspector/solfunmeme-dioxus/ontologies/zos/v1.ttl`
*   **Property:** `num:hasMeaning` (from `numberology.ttl` to link to prime concepts) and `vibe:hasPrimeFactor` (from `zos/v1.ttl` for vibe layers).
*   **Description:** A prime number conceptually associated with the term or its related layer/concept.
*   **Example Usage:**
    *   From `numberology.ttl`: `num:Prime2 num:hasMeaning num:DualityBinaryFundamentalChoices .`
    *   From `zos/v1.ttl`: `vibe:Layer1 a vibe:Layer ; vibe:hasPrimeFactor 2 .`

### 5. `is_power_of_two`

*   **Source Ontology:** `vendor/meta-introspector/solfunmeme-dioxus/ontologies/zos/v1.ttl`
*   **Property:** `vibe:isPowerOfTwo`
*   **Description:** A boolean indicating if the numerical address of a `vibe:Layer` is a power of two.
*   **Example Usage:**
    *   From `zos/v1.ttl`: `vibe:Layer1 a vibe:Layer ; vibe:isPowerOfTwo "true"^^xsd:boolean .`

### 6. `numerical_address`

*   **Source Ontology:** `vendor/meta-introspector/solfunmeme-dioxus/ontologies/index.ttl` and `vendor/meta-introspector/solfunmeme-dioxus/ontologies/zos/v1.ttl`
*   **Property:** `em:hasNumericalAddress` and `vibe:hasNumericalAddress`
*   **Description:** A unique numerical ID for a crate or a vibe layer.
*   **Example Usage:**
    *   From `index.ttl`: `crates_root:bootstrapCrate em:hasNumericalAddress "1" .`
    *   From `zos/v1.ttl`: `vibe:Layer1 a vibe:Layer ; vibe:hasNumericalAddress 1 .`

### 7. `embedding_vectors`

*   **Source:** Glossary entries (e.g., `docs/index/glossary_terms/*.md`)
*   **Description:** A multivector representation of the term, containing embeddings in various dimensions (e.g., 8D, 23D, 41D, 800D).
*   **Example Usage:**
    ```
    "embedding_vectors": {
        "8D": [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        "23D": [0.0, ...],
        "41D": [0.0, ...],
        "800D": [0.0, ...]
    }
    ```

## Manual Augmentation Process

To manually augment `augmented_terms_hot_take.json`, you will need to:

1.  Open `augmented_terms_hot_take.json` in a text editor.
2.  For each `AugmentedTermEntry` in the `augmented_terms` array, identify the `term`.
3.  Consult the ontology files (`numberology.ttl`, `index.ttl`, `zos/v1.ttl`) to find relevant properties for that `term`.
4.  Add the corresponding `emoji_representation`, `semantic_names`, `osi_layer`, `prime_factor`, `is_power_of_two`, and `numerical_address` values to the `AugmentedTermEntry` object. Ensure correct data types (e.g., `null` for `Option` fields if no value, `true`/`false` for boolean, numbers for numerical fields, arrays of strings for `semantic_names`).

This document provides the necessary mapping for manual augmentation.
