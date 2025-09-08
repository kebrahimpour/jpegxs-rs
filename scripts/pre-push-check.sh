#!/bin/bash

echo "🔍 Running pre-push checks..."

echo "📝 1. Checking formatting..."
if ! cargo fmt --check 2>/dev/null; then
    echo "❌ Format check failed. Run 'cargo fmt' to fix."
    exit 1
fi
echo "✅ Format check passed"

echo "🔎 2. Running clippy..."
if ! cargo clippy --all-targets --all-features -- -D warnings 2>/dev/null; then
    echo "❌ Clippy check failed. Fix the warnings above."
    exit 1
fi
echo "✅ Clippy check passed"

echo "🧪 3. Running tests..."
if ! cargo test --quiet 2>/dev/null; then
    echo "❌ Tests failed. Fix the failing tests."
    exit 1
fi
echo "✅ All tests passed"

echo "✨ All pre-push checks passed! Ready to push."