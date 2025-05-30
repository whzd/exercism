//! Tests for reverse-string
//!
//! Generated by [script][script] using [canonical data][canonical-data]
//!
//! [script]: https://github.com/exercism/rust/blob/b829ce2/bin/init_exercise.py
//! [canonical-data]: https://raw.githubusercontent.com/exercism/problem-specifications/main/exercises/reverse-string/canonical_data.json

use reverse_string::*;

/// Process a single test case for the property `reverse`
fn process_reverse_case(input: &str, expected: &str) {
    assert_eq!(&reverse(input), expected)
}

#[test]
/// empty string
fn test_an_empty_string() {
    process_reverse_case("", "");
}

#[test]
/// a word
fn test_a_word() {
    process_reverse_case("robot", "tobor");
}

#[test]
/// a capitalized word
fn test_a_capitalized_word() {
    process_reverse_case("Ramen", "nemaR");
}

#[test]
/// a sentence with punctuation
fn test_a_sentence_with_punctuation() {
    process_reverse_case("I'm hungry!", "!yrgnuh m'I");
}

#[test]
/// a palindrome
fn test_a_palindrome() {
    process_reverse_case("racecar", "racecar");
}

#[test]
fn test_an_even_sized_word() {
    process_reverse_case("drawer", "reward");
}

#[test]
/// wide characters
fn test_wide_characters() {
    process_reverse_case("子猫", "猫子");
}

#[test]
#[cfg(feature = "grapheme")]
/// grapheme clusters
fn test_grapheme_clusters() {
    process_reverse_case("uüu", "uüu");
}
