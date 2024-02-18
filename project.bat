@echo off

if "%1" == "t" (
    cd platinum_core
    cargo t
    cd ..
) else if "%1" == "r" (
    cd cii
    cargo r
    cd ..
) else if "%1" == "push" {
    git add *
    git status
    git commit -m %2
    git push -u origin main
}