# This script is used to run the release build many times at once.
cargo build --release

cd target/release
for ($i = 0; $i -lt 5; $i++) {
    # Output to a different file each time.
    # Create a job that runs ./eternity.exe and redirects the output to a file.
    Start-Job -ScriptBlock { .\eternity.exe } -RedirectStandardOutput "output$i.txt"
}