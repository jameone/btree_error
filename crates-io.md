# Binary Tree Error (btree_error)

![CodeBuild]
[![Version badge]][crates.io]
[![Docs badge]][docs.rs]

[CodeBuild]: https://codebuild.us-east-1.amazonaws.com/badges?uuid=eyJlbmNyeXB0ZWREYXRhIjoiaWV1SWhZbW5QTEhoL0lnZEpKb1ZxZGNQUnlDZStkQ01yTWhSMm5wUFNTc0xLRlUyQ1JUdkwvKzRhRTQ0c1YxOGNRTzJORjY4T2d1WFRsSWRJMy9hS0Q0PSIsIml2UGFyYW1ldGVyU3BlYyI6IkE5dE1Fa2xwdUZNVmU2eFYiLCJtYXRlcmlhbFNldFNlcmlhbCI6MX0%3D&branch=main
[Version badge]: https://img.shields.io/crates/v/btree_error
[crates.io]: https://crates.io/crates/btree_error
[Docs badge]: https://img.shields.io/badge/docs.rs-rustdoc-blue
[docs.rs]: https://docs.rs/btree_error/

To reduce repeated implementations of a simple error enum, this crate
has been separated from the `bforest` crates.

## Example
```rust
use btree_error::Error;
use btree_graph::BTreeGraph;

fn main() {
    let mut graph: BTreeGraph<String, String> = BTreeGraph::new();
    // Add nodes.
    graph.add_vertex(String::from("Tarzan"));
    graph.add_vertex(String::from("Jane"));
    // Add a relationship.
    let err: Error = graph.add_edge(String::from("Tarzan"), String::from("Sabor"), String::from("Hates")).unwrap_err();

    // Assert error was returned.
    assert_eq!(err, Error::VertexDoesNotExist);
}
```

## Usage

It is doubtful anyone will use this crate by itself, but for completeness,
add the following to your `Cargo.toml` file:
```toml
[dependencies]
btree_error = "0.1.0"
```

## Implementation

Please see the [API](src/error/mod.rs) for a the full definition.

## License

This work is dually licensed under MIT OR Apache-2.0.
