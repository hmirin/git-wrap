/// CLI parsing utilities

/// Check if --yes or -y is in args, or GIT_WRAP_YES env var is set.
///
/// Three ways to bypass safety checks:
/// - `--yes` / `-y` flag: one-time override for a single command
/// - `GIT_WRAP_YES=1` env var: session-wide override (for CI/CD)
/// - config `false`: permanent opt-out per check
pub fn has_yes(args: &[String]) -> bool {
    args.iter().any(|a| a == "--yes" || a == "-y")
        || std::env::var("GIT_WRAP_YES")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false)
}

/// Strip --yes and -y from args before passing to git.
pub fn strip_yes(args: &[String]) -> Vec<String> {
    args.iter()
        .filter(|a| a.as_str() != "--yes" && a.as_str() != "-y")
        .cloned()
        .collect()
}
