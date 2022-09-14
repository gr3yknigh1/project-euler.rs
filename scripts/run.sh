#!/usr/bin/sh

output="${1##*/}"
output="${output%.*}"
output=/tmp/$(basename -- "$output")

rustc $1 -o $output
$output

