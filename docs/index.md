# Phoenix Db Development Plan

## Table of Contents

- [Overview](#overview)
- [Build Requirements](#build-requirements)
- [Features](#features)
- [Design](#design)
- [Commands](#commands)
- [Roadmap](#roadmap)
- [Release](#release)

## Overview

Phoenix DB is a key-value databases developed by [CodingWithJamal](https://codingwithjamal.vercel.app) as an exercise to
learn computer science
algorithms and research system level programming.

## Build Requirements

- [Rust](https://www.rust-lang.org)

## Features

- [Key-Value Store](https://en.wikipedia.org/wiki/Key-value_stores)
- [Async IO](https://tokio.rs/)
- [TTL Data](https://en.wikipedia.org/wiki/Time-to-live)
- [Multithreaded](https://en.wikipedia.org/wiki/Multithreaded_programming)
- [CLI](https://en.wikipedia.org/wiki/Command-line_interface)

## Design

- `Client` - Any program that implement's the database network protocol
- `Server` - The database host and manager program

# Commands

- `INSERT`
- `INSERT *`
- `LOOKUP`
- `LOOKUP *`
- `DELETE`
- `DELETE *`
- `CREATE`
- `DESTROY`
- `EXIT`

## Roadmap

[View here](https://github.com/users/ThatGuyJamal/projects/6/views/1?layout=board)

## Release

Releasing this database to project will involve some work. The easiest way to allow devs to try the database is to
use the [cargo install](https://doc.rust-lang.org/cargo/commands/cargo-install.html) command. This will give the user a
CLI to run the database after cargo builds the binary application.

