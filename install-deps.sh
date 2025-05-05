#!/usr/bin/env bash
set -euo pipefail

echo "## リポジトリデータベースを同期"
sudo pacman -Sy --noconfirm  

echo "## 依存関係を一括インストール"
sudo pacman -S --noconfirm --needed \
    base-devel \
    arch-install-scripts \
    xorriso \
    squashfs-tools
