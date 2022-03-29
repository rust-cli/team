# CLI working group

This repo is for coordinating the work of the Rust CLI Working Group,
also known as "Rust CLIQuE" (Rust CLI Quality Enhancement).

- [Working groups?](https://internals.rust-lang.org/t/announcing-the-2018-domain-working-groups/6737)
- [Announcement of this WG](https://internals.rust-lang.org/t/announcing-the-cli-working-group/6872/1)
- Chat with us
  - [Zulip](https://rust-lang.zulipchat.com/#narrow/stream/220302-wg-cli)
  - [Discord](https://discord.gg/dwq4Zme)


## Our goal

Let's make this a true statement:

Rust makes writing crossplatform, tested, modern command line applications frictionless
while incorporating industry best practices and providing great documentation.

## What's a CLI?

For our intents and purposes, a CLI is any program that

* Launches in a terminal
* Accepts configuration from various sources, such as command line arguments, environment variables, or configuration files
* Runs to completion with minimal/no user interaction
* Accepts input from `stdin`, files, or network
* Performs processing on some input (files, network, `stdin`) based on the configuration specified
* Communicates via standard outputs (files, network, `std{out,err}`)

(We [specifically][i4] don't want to focus on "TUI" apps right now.)

[i4]: https://github.com/rust-cli/meta/issues/4
