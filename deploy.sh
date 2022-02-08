#!/bin/bash

PROJECT_NAME="devil"

project_path=$PWD

if [ ! -f "$PWD/Cargo.toml" ]; then
    project_path=$2
fi

if [ ! -f "$project_path/Cargo.toml" ]; then
    echo "Cargo.toml not found"
    echo "Please specify the path to the project"
    exit 1
fi

if [ ! -d "$1" ]; then
    echo "Directory $1 not found"
    echo "Please specify the target path"
    exit 1
fi

echo "Deploying from $project_path to $1"
echo -n "Confirm? (y/N) "

read answer

if [ "$answer" != "y" ]; then
    echo "Aborting"
    exit 1
fi

cd $project_path
cargo build --release

sudo /bin/bash <<EOF
cd $project_path

systemctl stop $PROJECT_NAME

cp target/release/$PROJECT_NAME $1/
cp -r migrations $1/

systemctl start $PROJECT_NAME
EOF