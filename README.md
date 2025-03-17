# Serverness

Serverness is a collection of tools and libraries for interacting with server-side APIs. This repository contains:

- **Rust SDK**: Native Rust library for API interactions
- **CLI**: Command-line interface for server operations
- **Rust Mocking Library**: Tools for API mocking in tests
- **Interactive Shell (WIP)**: Shell interface built with Nushell

## API Generation

Our API implementations are automatically generated from OpenAPI specifications using:

- **Dropshot**: API framework for Rust server implementations
- **Progenitor**: Code generation tool for Rust clients

The workflow is designed for reproducibility:

1. `serverness.json` contains the canonical OpenAPI specification
2. Generated code is checked into version control
3. To update, run `cargo xtask generate` after modifying the spec

This approach ensures consistent SDK/CLI updates when the API specification changes.

## Interactive Shell (Work in Progress)

We're building an interactive shell experience using:

- Nushell as the foundational framework (leveraging its library capabilities)
- Custom command generation from our OpenAPI spec

Current status:

- Command generation pipeline is under development in our [progenitor soft-fork](https://github.com/serverness/progenitor)
- Shell implementation is in active development

> Note: The shell component is not yet ready for production use

## Repository Structure

`serverness.json` serves as the single source of truth for API definitions. All generated components (SDK, CLI) are derived from this specification through our build process.

## Contributing

Please check our [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on:

- Modifying the API specification
- Extending the mocking framework
- Developing shell commands
