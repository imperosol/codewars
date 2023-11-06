/// [kyu]
/// 4
///
/// [description]
/// Write a function which makes a list of strings representing all of the ways you can balance n pairs of parentheses
//
/// # Examples
/// ```
/// balancedParens 0 -> [""]
/// balancedParens 1 -> ["()"]
/// balancedParens 2 -> ["()()","(())"]
/// balancedParens 3 -> ["()()()","(())()","()(())","(()())","((()))"]
/// ```

fn generate_parens(results: &mut Vec<String>, n: u16, existing: &str, opened: u16, closed: u16) {
    if opened == n && closed == n {
        results.push(existing.to_string());
        return;
    }
    if opened < n {
        generate_parens(
            results,
            n,
            &(existing.to_string() + "("),
            opened + 1,
            closed,
        )
    }
    if closed < opened {
        generate_parens(
            results,
            n,
            &(existing.to_string() + ")"),
            opened,
            closed + 1,
        )
    }
}

fn balanced_parens(n: u16) -> Vec<String> {
    let mut res = Vec::new();
    generate_parens(&mut res, n, "", 0, 0);
    res
}

fn main() {
    let tests = [
        (0, vec![""]),
        (1, vec!["()"]),
        (2, vec!["(())", "()()"]),
        (3, vec!["((()))", "(()())", "(())()", "()(())", "()()()"]),
    ];
    for (n, exp) in tests.into_iter() {
        let mut parens = balanced_parens(n);
        parens.sort();
        let expected = exp.iter().map(|s| s.to_string()).collect::<Vec<String>>();
        assert_eq!(parens, expected);
    }
}
