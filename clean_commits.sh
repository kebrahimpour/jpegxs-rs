#!/bin/bash

# Script to remove AI co-author signatures from git history

echo "🔍 Removing AI co-author signatures from commit history..."

# Create a backup branch first
git branch backup-before-cleanup-$(date +%Y%m%d-%H%M%S)

# Use git filter-branch to remove Co-authored-by lines with AI assistants
git filter-branch --force --msg-filter '
    sed "/Co-authored-by:.*Claude/d; /Co-authored-by:.*Copilot/d; /Co-authored-by:.*AI/d"
' --tag-name-filter cat -- --all

echo "✅ AI signatures removed from commit messages"
echo ""
echo "⚠️  IMPORTANT: This has rewritten history!"
echo ""
echo "Next steps:"
echo "1. Review the changes with: git log --oneline"
echo "2. Force push to remote with: git push --force-with-lease origin main"
echo "3. Other contributors will need to re-clone or reset their local copies"
echo ""
echo "If something went wrong, restore from backup branch"
