# `rds-admin`

## What is it? (Work in progress!)

A WebSocket server that executes whitelisted commands on a host as per
requests from authorized clients. There may be many clients connected at the
same time but only one command or set of commands may be in execution at any
given time. All connected clients shall see the current, shared execution
status, i.e. what's being run, the corresponding _stdout_ and _stderr_ and
finally the exit status.

## Why is it?

To help me (or my friends) maintain a _Rust Dedicated Server_ (the video game,
hence the name `rds-admin`) instance, and for me to learn _Rust_ (the
programming language). Mostly for me to learn Rust, since the command execution
purpose of this program would be best served by some existing solution like
`ssh`.

## Usage

1. Start the server:

   ```bash
   cargo run -- config.toml
   ```

   The program, `rds-admin`, will use the provided configuration if it's valid.
   Otherwise a valid default configuration will be written to the given path,
   and the default configuration will be used.

   The program is not complete. TODO!

2. Make a WebSocket connection to the server and send messages there.

   A real deployment would have some kind of auth before accepting the messages
   and there should be some real frontend to this too. TODO!

