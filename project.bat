@echo off

if "%1" == "test" (
    cd platinum_core
    cargo test
    cd ..
) else if "%1" == "run" (
    cd cii
    cargo run
    cd ..
) else if "%1" == "push" (
    git add *
    git status
    git commit -m %2
    git push -u origin main
)