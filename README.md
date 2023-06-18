# fsync-rs
Basic File Syncing Tool with custom source directory and auto generated output directory from a base. (Format Day-Month-Year)

Copies files that arent in output directory or got changed and deletes files that were deleted.

# Preview
![img](https://github.com/EKQRCalamity/fsync-rs/blob/main/preview.png)

# Arguments

|       Argument       |        Parameters       |   Example   |            Function              |
|----------------------|-------------------------|-------------|----------------------------------|
| -c / --config        | None                    |             | Enter Config Setup               |
| -h / --help          | None                    |             | Show help message                |
| -fin / --format      | Date Format String      | %d-%m-%Y    | Set Source Directory Date Format |
| -fout / --format_out | Date Format String      | %d-$m-$h-$s | Set Output Directory Date Format |
| -df / --dformat      | None                    |             | Set my default Date Formats      |
| -t / --timestamp     | Time Format (Optional)  | %H:%M:%S    | Enable timestamp and set format  |
| -q / --quiet         | None                    |             | Disable most console outputs     |