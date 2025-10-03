#!/bin/bash

# Check if the correct number of parameters is provided
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <project_name>"
    exit 1
fi

# Assign parameters to variables
project_name="$1"

# Check if destination directory exists, if so, abort
if [ -d "$project_name" ]; then
    echo "Project directory '$project_name' already exists. Aborting."
    exit 1
fi

# Create a new Rust project using Cargo
cargo new "$project_name"

# Confirm creation
if [ $? -ne 0 ]; then
    echo "Failed to create Rust project."
    exit 1
fi

echo "Rust project '$project_name' created successfully."

# Copy the template.rs file into the project as main.rs
if [ ! -f ./template.rs ]; then
    echo "Temple file 'template.rs' not found in current directory."
    exit 1
fi

cp ./template.rs "$project_name/src/main.rs"
echo "Template file 'template.rs' copied to '$project_name/src/main.rs'."

# Create extra data files
touch "$project_name/data.txt" "$project_name/test_data.txt"
echo "Extra files 'data.txt' and 'test_data.txt' created."
