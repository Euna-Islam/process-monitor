#cargo build
cmd /c cargo clean
cmd /c "cargo build"

#copy config files
#New-Item -Path "target\debug\text_files" -ItemType Directory -ErrorAction SilentlyContinue

Copy-Item -Path "text_files" -Destination "target\debug\text_files" -Recurse -Force