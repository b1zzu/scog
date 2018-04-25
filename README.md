# Sync Config Over Git

Sync config files through multiple pc using a remote git repository

## How to build

Install Rust and Cargo

```bash
cargo build
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

Checkout new branch called `_backup_%branch_%date`, copy local files to `_backup_%branch_%date` branch, commit them, push the branch,
checkout the previous branch, pull new changes from remote with `--only-ff` (Conflicts must be fixed manually), copy file from repository to local.
```bash
scog pull
```

Copy local file to repository, commit them, execute same steps of `scog pull`, push new changes.
```bash
scog push
```

## Config

```yaml
files:
- file: /home/davide/.bashrc:
  owner: davide
```