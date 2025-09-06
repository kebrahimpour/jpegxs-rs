#!/bin/bash
set -euo pipefail

echo "Setting up JPEG XS development environment"

# Clone the reference implementation as a submodule
if [ ! -d "reference/jxs" ]; then
    mkdir -p reference
    cd reference
    git clone https://github.com/TangKii/jxs.git
    cd jxs
    echo "Reference implementation cloned successfully"
    cd ../..
else
    echo "Reference implementation already exists"
fi

echo "Setup complete! Next steps:"
echo "1. Fork https://github.com/TangKii/jxs to your GitHub account (kebrahimpour)"
echo "2. Update the remote URL in reference/jxs to point to your fork"
echo "3. Run 'docker-compose up' to start the development environment"