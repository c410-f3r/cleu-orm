#!/usr/bin/env bash

. "$(dirname "$0")/common.sh" --source-only

PACKAGES=(
    cleu-orm-derive
    cleu-orm
)

for package in "${PACKAGES[@]}"; do
    pushd "${package}"
    /bin/echo -e "\e[0;33m***** Publishing ${package} *****\e[0m\n"
    cargo publish
    sleep 25
    popd
done