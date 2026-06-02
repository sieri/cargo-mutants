//! substitute methods by name

use std::borrow::Cow;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use regex::Regex;

struct MethodSubsitution<'a> {
    match_pattern: &'a str,
    replace_with: &'a [&'a str],
}

impl<'a> MethodSubsitution<'a> {
    pub const fn new(match_pattern: &'a str, replace_with: &'a [&'a str]) -> Self {
        Self {
            match_pattern,
            replace_with,
        }
    }

    pub fn check(&self, text: &'a str) -> Vec<Cow<'a, str>> {
        let re = Regex::new(self.match_pattern).unwrap();
        if re.is_match(text) {
            self.replace_with
                .iter()
                .map(|s| re.replace(text, *s))
                .collect()
        } else {
            vec![]
        }
    }
}

/// list of default subsitutions
const DEFAULT_SUBSITUTIONS: [MethodSubsitution<'_>; 25] = [
    MethodSubsitution::new(r"$checked_add", &["checked_sub", "checked_mul"]),
    MethodSubsitution::new(r"$checked_sub", &["checked_add", "checked_div"]),
    MethodSubsitution::new(r"$checked_mul", &["checked_add", "checked_div"]),
    MethodSubsitution::new(r"$checked_div", &["checked_mul"]),
    MethodSubsitution::new(r"$checked_div", &["checked_mul"]),
    MethodSubsitution::new(r"$strict_add", &["strict_sub", "strict_mul"]),
    MethodSubsitution::new(r"$strict_sub", &["strict_add", "strict_div"]),
    MethodSubsitution::new(r"$strict_mul", &["strict_add", "strict_div"]),
    MethodSubsitution::new(r"$strict_div", &["strict_mul"]),
    MethodSubsitution::new(r"$strict_div", &["strict_mul"]),
    MethodSubsitution::new(r"$overflow_add", &["overflow_sub", "overflow_mul"]),
    MethodSubsitution::new(r"$overflow_sub", &["overflow_add", "overflow_div"]),
    MethodSubsitution::new(r"$overflow_mul", &["overflow_add", "overflow_div"]),
    MethodSubsitution::new(r"$overflow_div", &["overflow_mul"]),
    MethodSubsitution::new(r"$overflow_div", &["overflow_mul"]),
    MethodSubsitution::new(r"$saturating_add", &["saturating_sub", "saturating_mul"]),
    MethodSubsitution::new(r"$saturating_sub", &["saturating_add", "saturating_div"]),
    MethodSubsitution::new(r"$saturating_mul", &["saturating_add", "saturating_div"]),
    MethodSubsitution::new(r"$saturating_div", &["saturating_mul"]),
    MethodSubsitution::new(r"$saturating_div", &["saturating_mul"]),
    MethodSubsitution::new(r"$wrapping_add", &["wrapping_sub", "wrapping_mul"]),
    MethodSubsitution::new(r"$wrapping_sub", &["wrapping_add", "wrapping_div"]),
    MethodSubsitution::new(r"$wrapping_mul", &["wrapping_add", "wrapping_div"]),
    MethodSubsitution::new(r"$wrapping_div", &["wrapping_mul"]),
    MethodSubsitution::new(r"$wrapping_div", &["wrapping_mul"]),
];

/// Find methods substitutions in the `DEFAULT_SUBSITUTIONS` list
pub fn find_substitution(text: &str) -> Vec<TokenStream> {
    DEFAULT_SUBSITUTIONS
        .iter()
        .flat_map(|s| s.check(text).into_iter())
        .map(|s| {
            let a = format_ident!("{}", s);
            quote! {#a}
        })
        .collect()
}
