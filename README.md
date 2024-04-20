# openapic

Simple openapi schema compiler in Rust. Read any openapi.yaml schema and generate code in multiple languages.

Right now it can read openapi schema and render it to Go structs.

# Why and design

We want to make a simple and more embeddable openapi generator, see article: https://chebykin.org/posts/openapic for more details.

Currently the CLI is build as two separate binaries:
 - frontendc - compiler entrypoint for processing OpenAPI schema
 - backendc - actual renderers to specific language.

The code in this repo is still in PoC stage, and we're open for comments and discussions about the architecture and project itself. Jump to our [Discord](https://discord.gg/Z7VPSCCn4g) channel for discussions!
## Usage and Testing

There are test schemas in `tests/` directory for running frontendc:
```bash
$ cargo run -- --input tests/api-large.yaml --output tests/out
```

To test the renderer, first, you need to run frontendc and generate the request:

```bash
$ cargo run -- --input tests/api-large.yaml --output tests/out > tests/req.json
```

And then you can run the backend:

```
$ cargo run --bin backend-go -Z macro-backtrace < tests/req.json
```

## Contributing

Contributions are always welcome! There's a lot of stuff to implement right now, feel free to take a look at issues.
