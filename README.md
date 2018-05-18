# Sync Config Over Git

Sync config files through multiple pc using a remote git repository

## How to build

Install Rust and Cargo

Build scog
```bash
cargo build
```

Execute
```bash
./target/debug/scog
```

To build the optimized code:
```bash
cargo build --release
./target/release/scog
```

## How it works

Clone the remote repository in `$HOME/.scog`
```bash
scog clone REPOSITORY
```

Checkout the branch, if it don't exists create it. The branch can not start with an underscore.
```bash
scog checkout BRANCH
```

Checkout new branch called `_backup_%branch_%date`, copy local files to `_backup_%branch_%date` branch, commit them,
checkout the previous branch, pull new changes from remote only if fast forward is possible, copy file from repository to local disk.
```bash
scog pull
```

Copy local files to repository, commit them, execute same steps of `scog pull`, push new changes.
```bash
scog push
```

## Config

```yaml
sections:
- path: /home/davide/.bashrc:
```