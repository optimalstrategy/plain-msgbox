# plain-msgbox
A crate for generating plain message boxes like the one below (formatting is manual):

```bash
╭────────────────────────────────╮
│ Call stack size:     1024      │
│ Interning threshold: 20        │
│ Optimization level:  1         │
│ Optimizations:                 │
│   Constant Folding:       true │
│   Peephole Optimizations: true │
│   Tail Call Optimization: true │
│   Dead Code Elimination:  true │
│ (misc): cfg_export = false     │
│ (misc): caching    = true      │
<Config>─────────────────────────╯
```

## Usage
Add this to your Cargo.toml:

```toml
[dependencies]
plain_msgbox = { git = "https://github.com/optimalstrategy/plain-msgbox" }
```

Then generate some boxes:

```rust
use plain_msgbox::generate_box;

let msg = generate_box(&[
    format!("A vec:    {:?}", vec![1, 2, 3]),
    format!("A tuple:  {:?}", (1, 2, 3)),
    format!("A string: {}", "abcdefghi"),
]);
assert_eq!(msg, "\
╭─────────────────────╮
│ A vec:    [1, 2, 3] │
│ A tuple:  (1, 2, 3) │
│ A string: abcdefghi │
╰─────────────────────╯");
```
