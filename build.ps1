#cargo clean
cmd /c cargo clean
#cargo build
cmd /c "cargo build"

#copy config files
Copy-Item -Path "text_files" -Destination "target\debug\text_files" -Recurse -Force