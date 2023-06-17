# fsync-rs
Basic File Syncing Tool with custom source directory and auto generated output directory from a base. (Format Day-Month-Year)

Copies files that arent in output directory and delete files that were deleted.

# Arguments

|   Argument  |     Parameters     |    Example  |            Function              |
|-------------|--------------------|-------------|----------------------------------|
| -c          | None               |             | Enter Config Setup               |
| -h          | None               |             | Show help message                |
| -format     | Date Format String | %d-%m-%Y    | Set Source Directory Date Format |
| -format_out | Date Format String | %d-$m-$h-$s | Set Output Directory Date Format |
| -dformat    | None               |             | Set my default Date Formats      |
