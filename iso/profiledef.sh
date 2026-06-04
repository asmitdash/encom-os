#!/usr/bin/env bash
# archiso profile for Encom OS.
# Consumed by `mkarchiso -v -o out/ iso/` inside the Docker build.

iso_name="encom-os"
iso_label="ENCOM_OS"
iso_publisher="Encom OS <https://github.com/asmitdash/encom-os>"
iso_application="Encom OS Live/Install"
iso_version="0.0.1"
install_dir="encom"
buildmodes=('iso')
bootmodes=('bios.syslinux.mbr' 'bios.syslinux.eltorito' 'uefi-x64.systemd-boot.esp' 'uefi-x64.systemd-boot.eltorito')
arch="x86_64"
pacman_conf="pacman.conf"
airootfs_image_type="squashfs"
airootfs_image_tool_options=('-comp' 'xz' '-Xbcj' 'x86' '-b' '1M' '-Xdict-size' '1M')
file_permissions=(
  ["/etc/shadow"]="0:0:400"
  ["/usr/local/bin/encom-firstboot-launcher"]="0:0:755"
)
