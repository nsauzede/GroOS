mkdir ~/yocto-rpi1 && cd ~/yocto-rpi1
# yocto core
git clone git://git.yoctoproject.org/poky.git
# for extra recipes like links:
git clone git://git.openembedded.org/meta-openembedded
# Raspberry Pi BSP layer
git clone git://git.yoctoproject.org/meta-raspberrypi

# will create and cd to build/
source poky/oe-init-build-env build

bitbake core-image-minimal

cd tmp/deploy/images/raspberrypi/
cp core-image-minimal-raspberrypi.rootfs.ext3 core-image-minimal-raspberrypi.rootfs_512M.ext3
qemu-img resize core-image-minimal-raspberrypi.rootfs_512M.ext3 512M
qemu-system-arm   -M raspi0   -kernel zImage   -dtb bcm2708-rpi-b.dtb  \
-drive file=core-image-minimal-raspberrypi.rootfs_512M.ext3,format=raw \
  -append "root=/dev/mmcblk0 console=ttyAMA0,115200"   -nographic

=> works fine until late kernel panic on bcm-specific power model missing


# Doesn't work with rpi1 target???
#bitbake qemu-helper-native
#runqemu nographic
