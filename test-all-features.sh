#!/bin/bash
set -e

RAND_VERSIONS="rand-0_8 rand-0_9"
BITVEC_VERSIONS="bitvec-0_17 bitvec-1_0"

echo "=== Running fmt check ==="
cargo fmt --all -- --check

for rand in $RAND_VERSIONS; do
  for bitvec in $BITVEC_VERSIONS; do
    echo ""
    echo "=== Testing $rand + $bitvec ==="
    cargo clippy --no-default-features --features $rand,$bitvec -- -D warnings
    cargo test --no-default-features --features $rand,$bitvec
  done
done

echo ""
echo "=== All checks passed! ==="

