#!/usr/bin/env bash
set -e

echo "🚀 Local build & publish for rustyweb-cli (with caching)"

# Load environment variables from .env
if [ -f .env ]; then
  echo "📄 Loading .env..."
  export $(grep -v '^#' .env | xargs)
else
  echo "❌ .env file not found!"
  exit 1
fi

# -------------------------------
# Build WASM
# -------------------------------
if [ -d "rust/wasm/pkg" ]; then
  echo "✅ WASM already built. Skipping..."
else
  echo "🛠 Building WASM..."
  cd rust/wasm
  wasm-pack build --release
  cd ../../
fi

# -------------------------------
# Build Native
# -------------------------------
if [ -d "rust/native/target/release" ]; then
  echo "✅ Native already built. Skipping..."
else
  echo "🛠 Building Native..."
  cd rust/native
  cargo build --release
  cd ../../
fi

# -------------------------------
# Build CLI
# -------------------------------
CLI_TARGET="rust/cli/target/release/rustyweb-cli"
if [ -f "$CLI_TARGET" ]; then
  echo "✅ CLI already built. Skipping..."
else
  echo "🛠 Building CLI..."
  cd rust/cli
  cargo build --release
  cd ../../
fi

# -------------------------------
# Bump package.json version (patch)
# -------------------------------
PACKAGE_JSON="packages/rustyweb/package.json"
OLD_VERSION=$(jq -r '.version' packages/rustyweb/package.json)
IFS='.' read -r MAJOR MINOR PATCH <<< "$OLD_VERSION"
PATCH=$((PATCH + 1))
NEW_VERSION="$MAJOR.$MINOR.$PATCH"
jq ".version=\"$NEW_VERSION\"" packages/rustyweb/package.json > tmp.json && mv tmp.json packages/rustyweb/package.json
echo "⬆️ Bumping version: $OLD_VERSION → $NEW_VERSION"

# -------------------------------
# Commit & Tag
# -------------------------------
git add "$PACKAGE_JSON"
git commit -m "Bump version to $NEW_VERSION"
git tag "v$NEW_VERSION"
git push origin HEAD
git push origin "v$NEW_VERSION"

# -------------------------------
# Generate .npmrc for GitHub Packages
# -------------------------------
echo "🔧 Generating .npmrc for GitHub Packages..."
if [ -z "${GITHUB_TOKEN:-}" ]; then
  echo "❌ GITHUB_TOKEN not set in .env"
  exit 1
fi

NAMESPACE="nia-cloud-official"  # replace with your GitHub username/org if needed
cat > packages/rustyweb/.npmrc <<EOL
//npm.pkg.github.com/:_authToken=${GITHUB_TOKEN}
@${NAMESPACE}:registry=https://npm.pkg.github.com
EOL
echo "✅ .npmrc generated in packages/rustyweb/"

# -------------------------------
# Publish to npm (GitHub Packages)
# -------------------------------
echo "📦 Publishing to npm..."
cd packages/rustyweb
sudo npm publish --access public
cd ../../

echo "🎉 Publish complete! Version $NEW_VERSION is live."

