# Known Issues

## `FloatLiteral::evaluate()` fallback loses leading zeros (latent, currently unreachable)

**Location:** [`src/evaluator/float_obj.rs`](../src/evaluator/float_obj.rs)

`FloatLiteral::evaluate()` re-parses `self.token.literal` as `f64` on every
evaluation. Only if that parse fails does it fall back to reconstructing the
value from `integer_part` (`i64`) and `float_part` (`u64`):

```rust
let str = format!("{}.{}", self.integer_part, self.float_part);
str.parse::<f64>().unwrap_or(0.0)
```

Because `float_part` is a plain `u64`, leading zeros in the fractional part
are lost. For example `1.05` is stored as `integer_part = 1`,
`float_part = 5`, which the fallback reconstructs as `"1.5"` instead of
`"1.05"`.

### Why this doesn't affect current users

`TokenType::Float` is deliberately excluded from the token-literal stripping
whitelist in [`src/ast.rs`](../src/ast.rs) (`Parser::next_token`), which is
used for `aloe build` (`BuildFlag::SizeOptimized`). This means
`token.literal` is always present in practice, so the fallback branch is
never actually reached today — the bug is latent, not observable via the CLI
or REPL.

The fallback exists as a safety net (e.g. for `token.literal` being stripped
in a future optimization, or malformed artifacts) and would need
`float_part` to preserve leading zeros (e.g. store it as a `String`, or pad
based on a stored digit count) to be correct.

### Regression tests

Covered in [`src/test/evaluator_test.rs`](../src/test/evaluator_test.rs):

- `test_float_literal_fallback_loses_leading_zeros_in_fractional_part` —
  documents the current (buggy) fallback output for `1.05` (`1.5`).
- `test_float_literal_fallback_without_fractional_part_is_correct` —
  sanity check that the fallback is correct when there's no fractional part.

If the fallback is fixed, update the first test to assert the correct value
(`1.05`) instead of the lossy one (`1.5`).

## Token-literal stripping for `aloe build` (`BuildFlag::SizeOptimized`)

**Location:** [`src/ast.rs`](../src/ast.rs) (`Parser::next_token`)

`Parser::set_strip_token_value(true)` clears `Token::literal` for tokens
whose text carries no semantic meaning beyond their `TokenType`, to shrink
artifact size. The whitelist currently covers: `Illegal`, `Eof`, all
keywords, delimiters (`(`, `)`, `{`, `}`, `[`, `]`, `,`, `:`, `;`), `Dot`,
`ScopeResolution`, `IteratorAssign`, and `QuestionMark`.

**Do not add to this whitelist without checking:**

- Whether the token's literal is parsed into a value at parse time
  (e.g. `Integer`, `Float`, all arithmetic/comparison/logical operators used
  in `PrefixExpression`/`InfixExpression`).
- Whether any `to_string()`/`Display`-style method reads `token.literal`
  directly instead of a dedicated field (e.g.
  `FunctionExpression::to_string()` reads `parameter.token.literal`, not
  `Identifier::value`).
- Whether the evaluator reads `token.literal` at runtime (currently only
  `FloatLiteral::evaluate()` does, see above).

Stripping a token type eagerly, before its literal has been consumed, was
the root cause of a previous bug: `aloe build` panicked with
`IntegerCanNotBeParsed` because `Integer` literals were cleared before
`parse_integer_literal()` could read them.
