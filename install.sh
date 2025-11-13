#!/usr/bin/env bash
# HypnoScript runtime installer / updater
#
# SECURITY NOTICE:
# This installer script may be executed via a command like:
#   curl -fsSL <URL> | bash
# Downloading and executing remote code via a pipe to bash can be dangerous.
# You should always review the script before running it, and consider downloading
# and inspecting it first:
#   curl -fsSL <URL> -o install.sh
#   less install.sh
#   bash install.sh
#

set -euo pipefail

# Default repository configuration
REPO_OWNER_DEFAULT="Kink-Development-Group"
REPO_NAME_DEFAULT="hyp-runtime"
GITHUB_BASE_DEFAULT="https://github.com"
API_BASE_DEFAULT="https://api.github.com"

# Allow environment variable overrides (with security warnings)
REPO_OWNER=${HYP_INSTALL_REPO_OWNER:-"$REPO_OWNER_DEFAULT"}
REPO_NAME=${HYP_INSTALL_REPO_NAME:-"$REPO_NAME_DEFAULT"}
GITHUB_BASE=${HYP_INSTALL_GITHUB_BASE:-"$GITHUB_BASE_DEFAULT"}
API_BASE=${HYP_INSTALL_API_BASE:-"$API_BASE_DEFAULT"}
DEFAULT_PREFIX=${HYP_INSTALL_PREFIX:-"/usr/local/bin"}

# Security: Warn if repository configuration has been overridden
REPO_OVERRIDE_WARNING=0
if [[ "$REPO_OWNER" != "$REPO_OWNER_DEFAULT" ]] || \
   [[ "$REPO_NAME" != "$REPO_NAME_DEFAULT" ]] || \
   [[ "$GITHUB_BASE" != "$GITHUB_BASE_DEFAULT" ]] || \
   [[ "$API_BASE" != "$API_BASE_DEFAULT" ]]; then
  REPO_OVERRIDE_WARNING=1
fi

SCRIPT_NAME=$(basename "$0")
SCRIPT_DIR=""
if [[ -n ${BASH_SOURCE[0]:-} && ${BASH_SOURCE[0]} != "-" ]]; then
  SCRIPT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" 2>/dev/null && pwd)
fi

declare -a CURL_AUTH_HEADERS=()
if [[ -n ${GITHUB_TOKEN:-} ]]; then
  CURL_AUTH_HEADERS=(-H "Authorization: Bearer $GITHUB_TOKEN" -H "X-GitHub-Api-Version: 2022-11-28")
fi

log() {
  local level=$1
  shift
  if [[ $QUIET -eq 1 && $level == INFO ]]; then
    return
  fi
  printf '[%s] %s\n' "$level" "$*"
}

info()  { log INFO "$@"; }
warn()  { log WARN "$@"; }
error() { log ERROR "$@"; }

usage() {
  cat <<EOF
HypnoScript Installer

Usage: $SCRIPT_NAME [options]

Options:
  --prefix <dir>       Install destination (default: ${DEFAULT_PREFIX})
  --version <ver>      Install a specific version (tag with or without leading v)
  --force              Reinstall even if the same version is already present
  --check              Only check for updates and exit
  --target <name>      Override release asset suffix (e.g. linux-x64)
  --include-prerelease Allow installing pre-release versions
  --uninstall          Remove the installed HypnoScript runtime
  --quiet              Suppress informational output
  --no-sudo            Do not attempt to elevate privileges automatically
  --help               Show this help message

Environment Variables (Advanced):
  HYP_INSTALL_PREFIX            Override default installation prefix
  HYP_INSTALL_PACKAGE_DIR       Use a local package directory instead of downloading
  GITHUB_TOKEN                  GitHub API token for authentication

  SECURITY WARNING: The following variables allow downloading from custom repositories.
  Only use these if you understand the security implications.

  HYP_INSTALL_REPO_OWNER        Override repository owner (default: $REPO_OWNER_DEFAULT)
  HYP_INSTALL_REPO_NAME         Override repository name (default: $REPO_NAME_DEFAULT)
  HYP_INSTALL_GITHUB_BASE       Override GitHub base URL (default: $GITHUB_BASE_DEFAULT)
  HYP_INSTALL_API_BASE          Override GitHub API base URL (default: $API_BASE_DEFAULT)
  HYP_INSTALL_ALLOW_OVERRIDE    Set to 1 to allow repository overrides in non-interactive mode
EOF
}

PREFIX="$DEFAULT_PREFIX"
PREFIX_SPECIFIED=0
REQUESTED_VERSION=""
FORCE=0
CHECK_ONLY=0
TARGET_OVERRIDE=""
AUTO_SUDO=1
INCLUDE_PRERELEASE=0
QUIET=0
UNINSTALL=0

while [[ $# -gt 0 ]]; do
  case "$1" in
    --prefix)
      PREFIX="$2"
      PREFIX_SPECIFIED=1
      shift 2
      ;;
    --version)
      REQUESTED_VERSION="$2"
      shift 2
      ;;
    --force)
      FORCE=1
      shift
      ;;
    --check)
      CHECK_ONLY=1
      shift
      ;;
    --target)
      TARGET_OVERRIDE="$2"
      shift 2
      ;;
    --include-prerelease)
      INCLUDE_PRERELEASE=1
      shift
      ;;
    --quiet)
      QUIET=1
      shift
      ;;
    --no-sudo)
      AUTO_SUDO=0
      shift
      ;;
    --uninstall|--remove)
      UNINSTALL=1
      shift
      ;;
    --help|-h)
      usage
      exit 0
      ;;
    *)
      error "Unknown argument: $1"
      usage
      exit 1
      ;;
  esac
done

if [[ $UNINSTALL -eq 1 && $CHECK_ONLY -eq 1 ]]; then
  error "--uninstall cannot be combined with --check"
  exit 1
fi

require() {
  command -v "$1" >/dev/null 2>&1 || {
    error "Required command not found: $1"
    exit 1
  }
}

trim_v() {
  local ver="$1"
  ver=${ver#v}
  ver=${ver#V}
  printf '%s' "$ver"
}

current_version=""
if command -v hypnoscript >/dev/null 2>&1; then
  current_version=$(hypnoscript --version 2>/dev/null | awk 'NF { for (i=1;i<=NF;i++) if ($i ~ /^v?[0-9]+(\.[0-9]+)*$/) { gsub(/^v/, "", $i); print $i; exit } }') || true
  if [[ -n "$current_version" ]]; then
    info "Detected installed version: $current_version"
  fi
fi

read_package_version() {
  local dir="$1"
  if [[ -f "$dir/VERSION.txt" ]]; then
    awk 'NF { print $1; exit }' "$dir/VERSION.txt"
    return
  fi
  if [[ -x "$dir/hypnoscript" ]]; then
    "$dir/hypnoscript" --version 2>/dev/null | awk 'NF { for (i=1;i<=NF;i++) if ($i ~ /^v?[0-9]+(\.[0-9]+)*$/) { gsub(/^v/, "", $i); print $i; exit } }' || true
    return
  fi
  printf ''
}

detect_os() {
  case "$(uname -s)" in
    Linux)  echo linux ;;
    Darwin) echo macos ;;
    *) error "Unsupported operating system"; exit 1 ;;
  esac
}

detect_arch() {
  case "$(uname -m)" in
    x86_64|amd64) echo x64 ;;
    arm64|aarch64) echo arm64 ;;
    *) error "Unsupported architecture"; exit 1 ;;
  esac
}

resolve_prefix() {
  local dir="$1"
  if [[ -w "$dir" ]]; then
    echo "$dir"
    return
  fi
  if [[ $AUTO_SUDO -eq 0 ]]; then
    error "Install directory $dir is not writable"
    exit 1
  fi
  if command -v sudo >/dev/null 2>&1 && [[ ${EUID:-0} -ne 0 ]]; then
    echo "sudo:$dir"
  else
    echo "$dir"
  fi
}

LOCAL_PACKAGE_DIR=""
if [[ -n ${HYP_INSTALL_PACKAGE_DIR:-} ]]; then
  LOCAL_PACKAGE_DIR="$HYP_INSTALL_PACKAGE_DIR"
elif [[ -n "$SCRIPT_DIR" ]]; then
  if [[ -f "$SCRIPT_DIR/hypnoscript" ]]; then
    LOCAL_PACKAGE_DIR="$SCRIPT_DIR"
  elif [[ -f "$SCRIPT_DIR/../hypnoscript" ]]; then
    LOCAL_PACKAGE_DIR=$(cd "$SCRIPT_DIR/.." && pwd)
  fi
fi

if [[ -n "$LOCAL_PACKAGE_DIR" && ! -d "$LOCAL_PACKAGE_DIR" ]]; then
  warn "Configured local package directory $LOCAL_PACKAGE_DIR not found"
  LOCAL_PACKAGE_DIR=""
fi

if [[ -n "$LOCAL_PACKAGE_DIR" ]]; then
  info "Found local package directory: $LOCAL_PACKAGE_DIR"
fi

# Security: Display warning if repository configuration has been overridden
if [[ $REPO_OVERRIDE_WARNING -eq 1 ]]; then
  warn "=========================================="
  warn "SECURITY WARNING: Repository configuration overridden via environment variables"
  warn "  REPO_OWNER: $REPO_OWNER (default: $REPO_OWNER_DEFAULT)"
  warn "  REPO_NAME: $REPO_NAME (default: $REPO_NAME_DEFAULT)"
  warn "  GITHUB_BASE: $GITHUB_BASE (default: $GITHUB_BASE_DEFAULT)"
  warn "  API_BASE: $API_BASE (default: $API_BASE_DEFAULT)"
  warn ""
  warn "This installer will download binaries from the specified repository."
  warn "Only proceed if you trust this source. Malicious binaries could"
  warn "compromise your system even if checksums match."
  warn "=========================================="

  if [[ -t 0 ]]; then
    read -p "Continue anyway? [y/N] " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
      error "Installation aborted by user"
      exit 1
    fi
  else
    error "Repository override detected in non-interactive mode. Aborting for safety."
    error "Set HYP_INSTALL_ALLOW_OVERRIDE=1 to bypass this check (not recommended)."
    if [[ ${HYP_INSTALL_ALLOW_OVERRIDE:-0} -ne 1 ]]; then
      exit 1
    fi
    warn "HYP_INSTALL_ALLOW_OVERRIDE=1 detected. Proceeding with custom repository."
  fi
fi

fetch_latest_version() {
  require curl
  local url
  if [[ $INCLUDE_PRERELEASE -eq 1 ]]; then
    url="$API_BASE/repos/$REPO_OWNER/$REPO_NAME/releases"
  else
    url="$API_BASE/repos/$REPO_OWNER/$REPO_NAME/releases/latest"
  fi
  local json
  if ! json=$(curl -fsSL ${CURL_AUTH_HEADERS[@]+"${CURL_AUTH_HEADERS[@]}"} "$url"); then
    error "Failed to fetch release information from GitHub API"
    return 1
  fi

  # Validate that we received JSON with at least one release
  if ! printf '%s' "$json" | grep -q '"tag_name"'; then
    error "Invalid or empty response from GitHub API (no releases found)"
    return 1
  fi

  local tag
  if [[ $INCLUDE_PRERELEASE -eq 1 ]]; then
    tag=$(printf '%s' "$json" | sed -n 's/.*"tag_name":"\([^"]*\)"[^}]*"prerelease":false.*/\1/p' | head -n1)
    [[ -n "$tag" ]] || tag=$(printf '%s' "$json" | sed -n 's/.*"tag_name":"\([^"]*\)".*/\1/p' | head -n1)
  else
    tag=$(printf '%s' "$json" | sed -n 's/.*"tag_name":"\([^"]*\)".*/\1/p' | head -n1)
  fi

  if [[ -z "$tag" ]]; then
    error "Failed to parse release tag from GitHub API response"
    return 1
  fi

  trim_v "$tag"
}

perform_uninstall() {
  local bin_path=""
  if [[ $PREFIX_SPECIFIED -eq 1 ]]; then
    if [[ -f "$PREFIX" ]]; then
      bin_path="$PREFIX"
    elif [[ -f "$PREFIX/hypnoscript" ]]; then
      bin_path="$PREFIX/hypnoscript"
    else
      warn "No hypnoscript binary found at prefix $PREFIX"
    fi
  fi

  if [[ -z "$bin_path" ]]; then
    bin_path=$(command -v hypnoscript 2>/dev/null || true)
  fi

  if [[ -z "$bin_path" ]]; then
    info "HypnoScript does not appear to be installed."
    exit 0
  fi

  if [[ ! -e "$bin_path" ]]; then
    info "Nothing to uninstall (missing binary at $bin_path)."
    exit 0
  fi

  local bin_dir
  if command -v realpath >/dev/null 2>&1; then
    bin_dir=$(realpath "$(dirname "$bin_path")")
  else
    bin_dir=$(cd "$(dirname "$bin_path")" && pwd -P)
  fi

  local prefix_root
  if command -v realpath >/dev/null 2>&1; then
    prefix_root=$(realpath "$bin_dir/..")
  else
    prefix_root=$(cd "$bin_dir/.." && pwd -P)
  fi

  local share_dir="$prefix_root/share/hypnoscript"
  local meta_file="$share_dir/installation.json"

  # Security check: Validate share_dir doesn't contain suspicious path components
  case "$share_dir" in
    *..*)
      error "Share directory path contains invalid components: $share_dir"
      exit 1
      ;;
    //*|*//*)
      error "Share directory path contains suspicious patterns: $share_dir"
      exit 1
      ;;
  esac

  # Security check: Ensure share_dir is a subdirectory of prefix_root
  if [[ "$share_dir" != "$prefix_root/share/hypnoscript" ]]; then
    error "Share directory path mismatch (possible manipulation attempt)"
    exit 1
  fi

  if [[ -f "$meta_file" ]]; then
    local recorded_prefix
    recorded_prefix=$(awk -F '"' '/"prefix"/ { print $4; exit }' "$meta_file" 2>/dev/null || printf '')
    if [[ -n "$recorded_prefix" && "$recorded_prefix" != "$bin_dir" ]]; then
      warn "Metadata prefix ($recorded_prefix) does not match resolved bin dir ($bin_dir)."
    fi
  fi

  local remover_prefix=""
  if [[ ! -w "$bin_dir" ]]; then
    if [[ $AUTO_SUDO -eq 0 ]]; then
      error "Insufficient permissions to remove $bin_path."
      exit 1
    fi
    if command -v sudo >/dev/null 2>&1 && [[ ${EUID:-0} -ne 0 ]]; then
      remover_prefix="sudo "
    elif [[ ${EUID:-0} -ne 0 ]]; then
      error "Insufficient permissions to remove $bin_path"
      exit 1
    fi
  fi

  info "Removing HypnoScript binary at $bin_path"
  if ! ${remover_prefix}rm -f "$bin_path"; then
    error "Failed to remove $bin_path"
    exit 1
  fi

  if [[ -d "$share_dir" ]]; then
    # Security check: Verify directory contains HypnoScript metadata before deletion
    local contains_metadata=0
    if [[ -f "$share_dir/installation.json" ]] || \
       [[ -f "$share_dir/VERSION.txt" ]] || \
       [[ -f "$share_dir/install.sh" ]]; then
      contains_metadata=1
    fi

    if [[ $contains_metadata -eq 0 ]]; then
      warn "Share directory exists but does not contain expected HypnoScript files. Skipping deletion for safety."
    else
      info "Removing HypnoScript metadata at $share_dir"
      ${remover_prefix}rm -rf "$share_dir"
    fi
  fi

  info "Uninstallation complete"
  exit 0
}

install_version=$REQUESTED_VERSION
if [[ -z "$install_version" ]]; then
  if [[ -n "$LOCAL_PACKAGE_DIR" ]]; then
    install_version=$(read_package_version "$LOCAL_PACKAGE_DIR")
  fi
fi
if [[ -z "$install_version" ]]; then
  install_version=$(fetch_latest_version) || {
    error "Failed to resolve latest version from GitHub API"
    exit 1
  }
fi
info "Target version: $install_version"

if [[ $UNINSTALL -eq 1 ]]; then
  perform_uninstall
fi

OS=$(detect_os)
ARCH=$(detect_arch)
TARGET_SUFFIX=${TARGET_OVERRIDE:-"${OS}-${ARCH}"}
info "Using target suffix: $TARGET_SUFFIX"

if [[ $CHECK_ONLY -eq 1 ]]; then
  if [[ -z "$current_version" ]]; then
    info "HypnoScript is not installed. Latest version: $install_version"
    exit 1
  fi
  if [[ "$current_version" == "$install_version" ]]; then
    info "HypnoScript is up to date."
    exit 0
  fi
  info "Update available: $current_version -> $install_version"
  exit 2
fi

if [[ -n "$current_version" && $FORCE -eq 0 && "$current_version" == "$install_version" ]]; then
  info "Version $install_version already installed. Use --force to reinstall."
  exit 0
fi

INSTALL_TARGET=$(resolve_prefix "$PREFIX")

UNPACK_DIR=""
TMPDIR=""
if [[ -n "$LOCAL_PACKAGE_DIR" ]]; then
  UNPACK_DIR="$LOCAL_PACKAGE_DIR"
else
  ASSET="hypnoscript-$TARGET_SUFFIX.tar.gz"
  DOWNLOAD_URL="$GITHUB_BASE/$REPO_OWNER/$REPO_NAME/releases/download/v$(trim_v "$install_version")/$ASSET"
  CHECKSUM_URL="$DOWNLOAD_URL.sha256"

  TMPDIR=$(mktemp -d)
  cleanup() { rm -rf "$TMPDIR"; }
  trap cleanup EXIT

  info "Downloading $ASSET"
  require curl
  curl -fsSL ${CURL_AUTH_HEADERS[@]+"${CURL_AUTH_HEADERS[@]}"} -o "$TMPDIR/$ASSET" "$DOWNLOAD_URL"
  if curl -fsSL ${CURL_AUTH_HEADERS[@]+"${CURL_AUTH_HEADERS[@]}"} -o "$TMPDIR/$ASSET.sha256" "$CHECKSUM_URL" 2>/dev/null; then
    if command -v sha256sum >/dev/null 2>&1; then
      (cd "$TMPDIR" && sha256sum -c "$ASSET.sha256")
    elif command -v shasum >/dev/null 2>&1; then
      (cd "$TMPDIR" && shasum -a 256 -c "$ASSET.sha256")
    else
      warn "Skipping checksum verification (sha256sum/shasum not available)"
    fi
  else
    warn "Checksum file not found; skipping verification"
  fi

  mkdir -p "$TMPDIR/unpack"
  tar -xzf "$TMPDIR/$ASSET" -C "$TMPDIR/unpack"
  UNPACK_DIR="$TMPDIR/unpack"
fi

if [[ ! -f "$UNPACK_DIR/hypnoscript" ]]; then
  error "Package does not contain hypnoscript binary"
  exit 1
fi

DEST_DIR=${INSTALL_TARGET#sudo:}
SUDO_PREFIX=""
if [[ $INSTALL_TARGET == sudo:* ]]; then
  SUDO_PREFIX="sudo "
fi

info "Installing to $DEST_DIR"
if [[ ! -w "$DEST_DIR" ]]; then
  ${SUDO_PREFIX}mkdir -p "$DEST_DIR"
fi
${SUDO_PREFIX}install -m 0755 "$UNPACK_DIR/hypnoscript" "$DEST_DIR/hypnoscript"

META_DIR="$DEST_DIR/../share/hypnoscript"
${SUDO_PREFIX}mkdir -p "$META_DIR"

if [[ -f "$UNPACK_DIR/VERSION.txt" ]]; then
  ${SUDO_PREFIX}install -m 0644 "$UNPACK_DIR/VERSION.txt" "$META_DIR/VERSION.txt"
fi

if [[ -f "$UNPACK_DIR/install.sh" ]]; then
  ${SUDO_PREFIX}install -m 0755 "$UNPACK_DIR/install.sh" "$META_DIR/install.sh"
elif [[ -n "$SCRIPT_DIR" && -f "$SCRIPT_DIR/install.sh" ]]; then
  ${SUDO_PREFIX}install -m 0755 "$SCRIPT_DIR/install.sh" "$META_DIR/install.sh"
fi

info_tmp=$(mktemp 2>/dev/null) || info_tmp="/tmp/hyp-install-info-$$"
: >"$info_tmp"
cat >"$info_tmp" <<EOF
{
  "version": "$(trim_v "$install_version")",
  "prefix": "$DEST_DIR",
  "target": "$TARGET_SUFFIX",
  "installed_at": "$(date -u +"%Y-%m-%dT%H:%M:%SZ" 2>/dev/null || date)",
  "source": "installer"
}
EOF
${SUDO_PREFIX}install -m 0644 "$info_tmp" "$META_DIR/installation.json"
rm -f "$info_tmp"

trap - EXIT
if [[ -n "$TMPDIR" ]]; then
  cleanup
fi

info "Installation complete"
if command -v hypnoscript >/dev/null 2>&1; then
  hypnoscript --version || true
else
  echo "Add $DEST_DIR to your PATH to use hypnoscript"
fi
