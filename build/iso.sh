#!/usr/bin/env bash
# Build an Encom OS ISO inside a privileged Docker container.
# Works from any host that can run Docker (Linux, macOS, Windows w/ WSL2).
#
# Usage:
#   ./build/iso.sh                # 'full' flavor (default)
#   ENCOM_FLAVOR=portable ./build/iso.sh
#
# Output: out/encom-os-<version>-x86_64.iso

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
FLAVOR="${ENCOM_FLAVOR:-full}"
OUT_DIR="${REPO_ROOT}/out"
WORK_DIR="${REPO_ROOT}/work"

mkdir -p "${OUT_DIR}" "${WORK_DIR}"

if ! command -v docker >/dev/null 2>&1; then
  echo "error: docker is required" >&2
  exit 1
fi

echo "==> Building Encom OS ISO (flavor=${FLAVOR})"

docker run --rm --privileged \
  -v "${REPO_ROOT}:/build" \
  -w /build \
  -e ENCOM_FLAVOR="${FLAVOR}" \
  archlinux:base-devel \
  bash -c '
    set -euo pipefail
    pacman -Sy --noconfirm archiso
    rm -rf work out/*.iso || true
    if [ "${ENCOM_FLAVOR}" = "portable" ]; then
      # Portable image: no Ollama, no local-model storage.
      :
    else
      # Full image: include Ollama package.
      grep -qx "ollama" iso/packages.x86_64 || echo "ollama" >> iso/packages.x86_64
    fi
    mkarchiso -v -w work -o out iso/
  '

echo "==> Done. ISO in ${OUT_DIR}/"
ls -la "${OUT_DIR}"
