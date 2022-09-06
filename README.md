# CLI working group

This repo is for coordinating the work of the Rust CLI Working Group,
also known as "Rust CLIQuE" (Rust CLI Quality Enhancement).

- [Working groups?](https://internals.rust-lang.org/t/announcing-the-2018-domain-working-groups/6737)
- [Announcement of this WG](https://internals.rust-lang.org/t/announcing-the-cli-working-group/6872/1)
- Chat with us on [Zulip](https://rust-lang.zulipchat.com/#narrow/stream/220302-wg-cli)


## Vision

Let's make this a true statement:

Rust makes writing crossplatform, tested, modern command line applications frictionless
while incorporating industry best practices and providing great documentation.

## Role of WG-CLI

- Identify gaps between here and the vision
- Mentoring people working to fill the gaps
- Keeping the lights on for core CLI crates
- Venue for CLI authors to collaborate

## Focus Areas

- [Argument parsing](https://github.com/rust-cli/team/labels/A-argparse)
  - [argpaerse-rosetta-rs](https://github.com/rosetta-rs/argparse-rosetta-rs)
- [Ease of solving CLI related problems (e.g. filesystem interactions)](https://github.com/rust-cli/team/labels/A-ergonomics)
- [Terminal output styling](https://github.com/rust-cli/team/labels/A-styling)
- [Interactive terminal (prompts, progress bars)](https://github.com/rust-cli/team/labels/A-interaction)
- [TUI (full control of terminal)](https://github.com/rust-cli/team/labels/A-tui)
- [Program diagnostics (errors, panics, logging, etc)](https://github.com/rust-cli/team/labels/A-diagnostic)
- [Configuration management](https://github.com/rust-cli/team/labels/A-config)
- [One-shot testing of CLIs (no interaction)](https://github.com/rust-cli/team/labels/A-testing-cli)
  - [assert_cmd](https://github.com/assert-rs/assert_cmd) / [assert_fs](https://github.com/assert-rs/assert_cmd)
  - [snapbox](https://github.com/assert-rs/trycmd/tree/main/crates/snapbox)
  - [trycmd](https://github.com/assert-rs/trycmd/)
- [Testing of time and user dependent CLIs](https://github.com/rust-cli/team/labels/A-testing-tui)
- [Documenting your CLI (e.g. man pages)](https://github.com/rust-cli/team/labels/A-doc)
- [Licensing, packaging, etc](https://github.com/rust-cli/team/labels/A-distribution)
- [Documenting how to create CLIs](https://github.com/rust-cli/team/labels/A-book)
  - [book](https://github.com/rust-cli/book)

While most of these cross-domains with GUIs, web backends, etc, they are also commonly in the critical path a CLI and so in-scope.
