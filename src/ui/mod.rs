name: DesktopHLE CI

on:
  push:
    branches: [ "main", "master" ]
  pull_request:
    branches: [ "main", "master" ]

env:
  CARGO_TERM_COLOR: always

jobs:
  build:
    name: Build & Test
    runs-on: ubuntu-latest

    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@stable

      - name: Install Display Server (para la ventana)
        run: |
          sudo apt-get update
          sudo apt-get install -y xvfb libgl1-mesa-dev

      - name: Build DesktopHLE
        run: cargo build --verbose

      - name: Run Tests
        run: xvfb-run cargo test --verbose
