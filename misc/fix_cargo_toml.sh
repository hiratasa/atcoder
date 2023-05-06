#!/bin/bash

matches=$(awk '/\[\[bin\]\]/{n=NR; getline; if(/name.*=.*/ && getline && /path.*=.*/) print n ":" $0;}' Cargo.toml)

declare -a delete_lines=()
while read -r match; do
    line_number=$(echo "$match" | cut -d':' -f1)
    path=$(echo "$match" | awk -F '"' '{print $2}')
    if [[ -e "$path" ]]; then
        :
    else
        echo "Path $path does not exist, removing corresponding block"
        delete_lines+=("$line_number" "$(($line_number+1))" "$(($line_number+2))")
    fi
done <<< "$matches"

if [ ${#delete_lines[@]} -ne 0 ]; then
    line_numbers=$(IFS=,; echo "${delete_lines[*]}")
    sed "${line_numbers//,/d;}d" -i Cargo.toml
    sed -i '/./,/^$/!d' Cargo.toml
fi