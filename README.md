# sync

Sync config files through multiple pc

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