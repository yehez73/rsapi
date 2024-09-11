# Backend API

This project is an API built using Rust programming language.

## Installation

### From Source

#### Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- [PostgreSQL](https://www.postgresql.org/download/)

#### Building

##### Manually

```sh
git clone https://github.com/yehez73/rsapi.git
cd rsapi
cargo build --bin ainodocs
```

##### Run it with automatic recompilation when any Rust files are changed
```sh
cargo watch -x run
```

## Usage
Base URL = http://localhost:8080

### Add this first

- /division/add
- /application/add
- /role/add
- /user/add -> (application_uuid, division_uuid, role_uuid required)

### Then

/login (user_email, password) (required all)