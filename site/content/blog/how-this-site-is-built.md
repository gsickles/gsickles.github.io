# How This Site Is Built

This website is built using a custom **Rust-based HTML templating system** with a declarative macro system for generating static HTML.

## The Stack

- **Rust** - For the HTML generation
- **Custom Macros** - Declarative syntax for writing HTML
- **No Build Tools** - Just `cargo run` to generate the site
- **Vanilla JavaScript** - No frameworks, just plain JS
- **Marked.js** - For rendering markdown content

## The Templating System

The system uses Rust macros with this syntax:

```rust
div[id="content", class="container"] {
    h1 { "Hello, World!" }
    p { "This is a paragraph." }
    a[href="https://example.com"] { "Link" }
}
```

This generates properly formatted HTML with tab indentation.

## Key Features

- **Compile-time asset inclusion** - CSS, JS, and markdown files are embedded using `include_str!`
- **Type-safe HTML** - Rust's compiler catches errors at build time
- **Single file output** - Everything is inlined into one `index.html`

## Implementation

The system explores:
- Declarative macro systems
- Compile-time code generation
- Type-safe templating
- Zero-runtime dependencies

## The Result

A self-contained website with no external dependencies. The entire site is in one HTML file with all assets inlined.

## Source Code

The generator consists of two files:
- `index.rs` - The page structure and content
- `html.rs` - The macro definitions

