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

Phoenix DB is a key-value databases developed by [CodingWithJamal](https://codingwithjamal.vercel.app) as an exericse to
learn computer science
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

# Commands

- `INSERT key value` - Set the value of a key
    - `INSERT *` - Allows multiple data inserts in one request using an array
- `LOOKUP key` - Get the value of a key
    - `LOOPUP *` - Gets all the keys in the database
- `DELETE key` - Delete the value of a key
    - `DELETE *` - Deletes all the data in the database

- `CREATE` - Create a new database
- `DESTROY` - Destroy a database
- `EXIT` - Exit the database
- `HELP` - List all commands

## Implementation

1. Build the TCP protocol  
   1.1 Data versioning
2. Build the TCP server  
   2.1 Command handling  
   2.2 TLS support
3. Build the CLI server  
   3.1 Support all native commands
   3.2 Support Repl (Read Evaluate Print Loop)
   3.3 Support DB passwords using a local file
4. Build the TCP client  
   4.1 Command API wrapper  
   4.2 Good error handling  
   4.3 Support Different languages (eg. Rust, JavaScript, Python)
5. Write tests
6. Write documentation
7. Release Alpha version

## Release

Releasing this database to project will involve some work. The easist way to allow users to install the database is to
use the [cargo install](https://doc.rust-lang.org/cargo/commands/cargo-install.html) command. This will give the user a
CLI to run the database after cargo builds the binary application.
