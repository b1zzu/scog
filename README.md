# bog

Sync config files through multiple pc using a remote git repository

## Target

Create a simple sync script that read the configuration from a .toml file. The .toml file will describe the directory to
sync and the repository to which sync. It should be possible to demonize the script, in that case it will listen for 
changes to the files and commit them on each change.

```toml
[main]
repository ="git@github.com:davbizz/myproject.git"
branch = "mypc"

[files]
  [files.mydir]
  dir = /etc
  user = root
```

## How should it work

Clone the remote repository in `$HOME/.bog`
```
bog clone REPOSITORY
```

Checkout the branch, if it don't exists create it. The branch can not start with an underscore.
```
bog checkout BRANCH
```

Checkout new branch called _%date, copy local files to _%date branch, commit them, push the branch,
checkout the previous branch, pull new changes from remote, copy file from repository to local.
```
bog pull
```

Copy local file to repository, commit them, pull new changes from server with --only-ff (Conflicts must be fixed manually),
push new changes, copy files form repository to local.
```
bog push
```