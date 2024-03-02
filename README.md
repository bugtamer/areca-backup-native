# areca-backup-native

Areca Backup native launchers for modern operative systems

`areca-backup-native` allow you to build Windows native binaries files for running Areca in a convenient way.
The built binaries are based on the logic of `areca.sh`, `areca_cl.sh` and `areca_run.sh`.


## How to build Windows binaries

1. Run:

   `PS areca-backup-native\launchers>` `.\build.bat`

   this generates binaries with Areca icon, this doesn't:

   `PS areca-backup-native\launchers>` `cargo build --bins`

2. Move these binaries from the _Output folder of `build.bat`_ to the _Areca destination_ folder.

| Binary          | Output folder of `build.bat`                | Areca destination |
| --------------- | ------------------------------------------- | ----------------- |
| `areca.exe`     | `areca-backup-native-launcher\target\debug` | `areca`           |
| `areca_cl.exe`  | `areca-backup-native-launcher\target\debug` | `areca\bin`       |


## Launch Areca

When these binaries run, they will try to find the Areca installation folder in this order:

1. `ARECA_HOME` environment variable (system or user scope)
2. `current working directory` of the binary execution


## Related Areca sites

- [Download Areca Backup](https://sourceforge.net/projects/areca/files/areca-stable/)
- [Areca Backup legacy documentation](https://github.com/bugtamer/areca-backup-legacy-documentation)


## Documentation

- [Setting a Rust Executable's Icon in Windows](https://anthropicstudios.com/2021/01/05/setting-a-rust-windows-exe-icon/)
  (If the URL is broken, try this [backup](/launchers/icon-for-exe.md))
- [How to install Rust Programming Language](https://www.rust-lang.org/tools/install)