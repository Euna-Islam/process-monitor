#cargo clean
cmd /c cargo clean
#cargo build
cmd /c "cargo build --release"

#copy config files
Copy-Item -Path "config_files" -Destination "target\release\config_files" -Recurse -Force