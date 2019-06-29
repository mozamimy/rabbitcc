#!/bin/bash

set -o pipefail
set -o nounset
# set -o xtrace

try() {
  expected="$1"
  input="$2"

  ./target/debug/rabbitcc "$input" > tmp/tmp.s
  gcc -o bin/tmp tmp/tmp.s
  ./bin/tmp
  actual="$?"

  if [[ "$actual" = "$expected" ]]; then
    echo "$input => $actual"
  else
    echo "$input => $expected expected, but got $actual"
    exit 1
  fi
}

try 0 0
try 42 42
try 21 "5+20-4"
try 9 "9+0"

echo OK
