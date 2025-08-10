use std::sync::{Arc, Mutex};
use crate::term_quiz_master::quiz_logic::Model;

pub fn run_match_prime_pattern_command(model_arc: Arc<Mutex<Model>>, table: bool) {
    println!("Searching for terms with prime reciprocal patterns...");
    // Updated to use the first 20 prime numbers
    const PRIME_RECIPROCALS: [f32; 20] = [
        1.0 / 2.0, 1.0 / 3.0, 1.0 / 5.0, 1.0 / 7.0, 1.0 / 11.0,
        1.0 / 13.0, 1.0 / 17.0, 1.0 / 19.0, 1.0 / 23.0, 1.0 / 29.0,
        1.0 / 31.0, 1.0 / 37.0, 1.0 / 41.0, 1.0 / 43.0, 1.0 / 47.0,
        1.0 / 53.0, 1.0 / 59.0, 1.0 / 61.0, 1.0 / 67.0, 1.0 / 71.0,
    ];
    const TOLERANCE: f32 = 0.01; // Define a small tolerance for float comparison

    let model = model_arc.lock().unwrap();
    let mut found_matches = false;

    // Data structure to store the best match for each (dimension, reciprocal) cell
    // Outer Vec for dimensions (8), inner Vec for reciprocals (PRIME_RECIPROCALS.len())
    // Stores (Option<(Term ID, Term Text, Actual Value, Distance)>)
    let mut best_matches_table: Vec<Vec<Option<(usize, String, f32, f32)>>> =
        vec![vec![None; PRIME_RECIPROCALS.len()]; 8]; // Adjusted size here

    for question in model.questions.iter() {
        let mut matches_for_term = Vec::new();
        for (dim_idx, &value) in question.embedding.iter().enumerate() {
            for (reciprocal_idx, &reciprocal) in PRIME_RECIPROCALS.iter().enumerate() {
                let distance = (value - reciprocal).abs();
                if distance < TOLERANCE {
                    // Store detailed match for default output
                    matches_for_term.push(format!("Matches 1/{} ({:.4}) in Dimension {} (Value: {:.4})",
                        (1.0 / reciprocal).round() as usize, reciprocal, dim_idx, value));

                    // Update best_matches_table for table output
                    let current_best = &best_matches_table[dim_idx][reciprocal_idx];
                    if current_best.is_none() || distance < current_best.as_ref().unwrap().3 {
                        best_matches_table[dim_idx][reciprocal_idx] =
                            Some((question.id, question.text.clone(), value, distance));
                    }
                }
            }
        }

        if !matches_for_term.is_empty() {
            found_matches = true;
            println!("- ID: {}, Term: \"{}\"", question.id, question.text);
            for m in matches_for_term {
                println!("  - {}", m);
            }
        }
    }

    if !found_matches {
        println!("No terms found matching prime reciprocal patterns within tolerance {:.4}.", TOLERANCE);
    }

    // --- Optional Table Output ---
    if table { // Check the `table` flag
        println!("\n--- Summary Table of Closest Terms to Prime Reciprocals ---");
        println!("Tolerance: {:.4}", TOLERANCE);

        // Print header row (Dimensions)
        print!("{:>15}", "1/Prime \\ Dim"); // Increased width
        for i in 0..8 {
            print!("{:>15}", format!("Dim {}", i)); // Increased width
        }
        println!();

        // Print data rows
        for (reciprocal_idx, &reciprocal) in PRIME_RECIPROCALS.iter().enumerate() {
            print!("{:>15.4}", reciprocal); // Prime reciprocal value, increased width
            for dim_idx in 0..8 {
                if let Some((_id, term_text, _value, _distance)) = &best_matches_table[dim_idx][reciprocal_idx] {
                    // Truncate term_text if too long
                    let display_text = if term_text.len() > 14 { // Adjusted truncation length
                        term_text[0..11].to_string() + "..." // Adjusted truncation length
                    } else {
                        term_text.clone()
                    };
                    print!("{:>15}", display_text);
                } else {
                    print!("{:>15}", "-"); // No match for this cell, increased width
                }
            }
            println!();
        }
        println!("----------------------------------------------------------");
    }
}