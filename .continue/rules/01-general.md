#Project Architecture

This is a Rust application with the following structure:
- The `src` directory contains the main source code.
- The `Cargo.toml` file defines dependencies and project metadata.
- The `src/finance_number` directory contains safe number calculation to avoid errors.

## Coding standards

- Always use Rust for all new files.
- Avoid `unsafe` blocks unless absolutely necessary.


# Mathmetics rule
- Use safty calculation to avoid overflows.
- Use fractions to avoid floating point errors.