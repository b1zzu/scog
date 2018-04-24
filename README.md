# Sync Config Over Git

Sync config files through multiple pc using a remote git repository

## Config

```yaml
repository: "git@github.com:davbizz/myproject.git"
files:
- file: /etc:
  owner: root
```

## How should it work

Clone the remote repository in `$HOME/.scog`
```
scog clone REPOSITORY
```

Checkout the branch, if it don't exists create it. The branch can not start with an underscore.
```
scog checkout BRANCH
```

Checkout new branch called _%date, copy local files to _%date branch, commit them, push the branch,
checkout the previous branch, pull new changes from remote, copy file from repository to local.
```
scog pull
```

Copy local file to repository, commit them, pull new changes from server with --only-ff (Conflicts must be fixed manually),
push new changes, copy files form repository to local.
```
scog push
```