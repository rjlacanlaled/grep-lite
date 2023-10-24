<div align="center">
    <hr />
        <h2 align="center" style="Grep-lite.</h2>
        <img alt="GitHub" src="https://img.shields.io/github/license/txpipe/oura" />
        <img alt="Crates.io" src="https://img.shields.io/crates/v/oura" />
        <img alt="GitHub Workflow Status" src="https://img.shields.io/github/actions/workflow/status/txpipe/oura/validate.yml" />
    <hr/>
</div>

## Introduction

Rust-in-action's grep lite project modified to accept arbitrary search term and file.

## Tutorial

```sh
# It accepts 2 params: search_term and file_path
cargo run -- search_term file_path

# To test using the test.txt file
cargo run -- From ./test.txt
```
