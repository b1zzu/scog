# sync

Sync config files through multiple pc

## Target

Create a simple sync script that read the configuration from a .yaml file. The .yaml file will describe the directory to
sync and the repository to which sync. It should be possible to demonize the script, in that case it will listen for 
changes to the files and commit them on each change.

```yaml
repository: "git@github.com:davbizz/myproject.git"
branch: mypc
files:
- /etc/
```