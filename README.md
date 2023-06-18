# fsync-rs
Basic File Syncing Tool with custom source directory and auto generated output directory from a base. (Format Day-Month-Year)

Copies files that arent in output directory or got changed and deletes files that were deleted.

# Preview
![img](https://github.com/EKQRCalamity/fsync-rs/blob/main/preview.png)

# Build
First of all you will need to have cargo installed on your system. Rustc standalone wont be enough.
If you have cargo installed clone this repository and build it.
```bash
git clone https://github.com/EKQRCalamity/fsync-rs
cd fsync-rs
cargo build --release
```
Or run it directly
```bash
cargo run
```

# Usage

When you are in the directory of the file (I have it in my bin folder so no need for that) you can simply run
```bash
fsync
```
After that you will be prompted for the config. You currently cant specify a specific config and should create one from the directory you executed it from. If you need to reenter the config setup use the -c parameter.

## Arguments

|       Argument       |        Parameters       |   Example   |            Function              |
|----------------------|-------------------------|-------------|----------------------------------|
| -c / --config        | None                    |             | Enter Config Setup               |
| -h / --help          | None                    |             | Show help message                |
| -fin / --format      | Date Format String      | %d-%m-%Y    | Set Source Directory Date Format |
| -fout / --format_out | Date Format String      | %d-$m-$h-$s | Set Output Directory Date Format |
| -df / --dformat      | None                    |             | Set my default Date Formats      |
| -t / --timestamp     | Time Format (Optional)  | %H:%M:%S    | Enable timestamp and set format  |
| -q / --quiet         | None                    |             | Disable most console outputs     |

*** Note -t should only be used as last parameter if you don't give it an format parameter.
