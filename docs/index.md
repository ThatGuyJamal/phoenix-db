# Phoenix Db Development Plan

## Table of Contents

- [Overview](#overview)
- [Requirements](#requirements)
- [Goals](#goals)
- [Design](#design)
- [Commands](#commands)
- [Implementation](#implementation)
- [Release](#release)

## Overview

Phoenix DB is a key-value databases developed by [CodingWithJamal](https://codingwithjamal.vercel.app) as an exericse to learn computer science
algorithms and research system level programming.

Note: Right now the database is not production ready by any means and is only used for learning purposes.

## Requirements

- [Rust](https://www.rust-lang.org)

## Goals

- `Memory` - Store data in memory
- `Disk` - Store data on disk
- `TCP` - Use a TPC protocol for communication with clients
- `tls` - Time to live for data
- `Threads` - Use threads for concurrency and parallelism

## Design

- `Client` - A client that connects to the database
- `Server` - A server that runs the database

- `Key` - A unique identifier for a type of data
- `Value` - The data stored in the database

### Binary Data

All data sent over the network is converted to binary. The binary data will then be stored in the database.

The first few bytes will contain metadata generated from the client to help with processing. Example:

```
type|size|key|value
```

This can allow us to save time allocating space for operations when writing to the database. (I hope lol)

# Commands

- `SET key value` - Set the value of a key
- `GET key` - Get the value of a key
- `DEL key` - Delete the value of a key

- `LIST` - List all keys
- `EXIT` - Exit the database
- `HELP` - List all commands

## Implementation

1. Build the TCP protocol
2. Build the TCP server
3. Build the TCP client
4. Build the CLI server
5. Build the rust client
6. Write tests
7. Write documentation
8. Release Alpha version

## Release

Releasing this database to project will involve some work. The easist way to allow users to install the database is to use the [cargo install](https://doc.rust-lang.org/cargo/commands/cargo-install.html) command. This will give the user a CLI to run the database after cargo builds the binary application.
