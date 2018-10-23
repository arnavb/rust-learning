for directory in */; do
    project_name=${directory%*/}
    printf "Checking project: $project_name"
    cd $directory
    cargo clean

    if cargo check; then
        printf "$project_name successfully compiled!"
    else
        printf "$project_name did not successfully compile!"
        exit 1
    fi

    cargo clean
    cd ..
    printf "\n\n"
done
