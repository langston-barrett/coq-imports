#!/usr/bin/env bash

# "integration test"

set -e

if [[ ! -f Test.v.bak ]]; then
  cp tests/Test.v{,.bak}
else
  cp tests/Test.v{.bak,}
fi

cargo build
./target/debug/coq-imports -vvv insert tests/Test.v "UniMath.CategoryTheory.Equivalences.Foo" && cat tests/Test.v
