#cargo clean
cmd /c cargo clean
#cargo build
cmd /c "cargo build"

#copy config files
Copy-Item -Path "config_files" -Destination "target\debug\config_files" -Recurse -Force