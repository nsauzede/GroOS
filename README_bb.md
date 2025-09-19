# Yocto procedure for BeagleBone (White)

sudo apt update
sudo apt install gawk wget git diffstat unzip texinfo gcc build-essential chrpath socat cpio python3 python3-pip python3-pexpect xz-utils debianutils iputils-ping python3-git python3-jinja2 libegl-mesa0 libsdl1.2-dev pylint xterm python3-subunit python3-cffi libcairo2-dev


mkdir ~/yocto-bbw && cd ~/yocto-bbw

git clone -b kirkstone git://git.yoctoproject.org/poky.git

git clone -b kirkstone git://git.openembedded.org/meta-openembedded

git clone -b kirkstone git://git.yoctoproject.org/meta-arm

git clone -b kirkstone git://git.yoctoproject.org/meta-ti

git clone -b master https://github.com/meta-rust/meta-rust.git

nano conf/bblayers.conf
BBLAYERS ?= " \
  /home/nico/yocto-bbw/poky/meta \
  /home/nico/yocto-bbw/poky/meta-poky \
  /home/nico/yocto-bbw/poky/meta-yocto-bsp \
  /home/nico/yocto-bbw/meta-openembedded/meta-oe \
  /home/nico/yocto-bbw/meta-openembedded/meta-python \
  /home/nico/yocto-bbw/meta-openembedded/meta-networking \
  /home/nico/yocto-bbw/meta-arm/meta-arm \
  /home/nico/yocto-bbw/meta-arm/meta-arm-toolchain \
  /home/nico/yocto-bbw/meta-ti/meta-ti-bsp \
  /home/nico/yocto-bbw/meta-rust \
  /home/nico/yocto-bbw/meta-bbw \
  "

nano conf/local.conf
MACHINE = "beaglebone"
DISTRO = "poky"
IMAGE_INSTALL:append = " python3 python3-core kernel-modules openssh dropbear rust"
DISTRO_FEATURES:append = " ipv4"
EXTRA_IMAGE_FEATURES = "ssh-server-dropbear tools-sdk"
KERNEL_MODULE_AUTOLOAD:append = " w1-gpio w1-therm"
BB_NUMBER_THREADS = "4"
PARALLEL_MAKE = "-j 4"

rm -rf ~/yocto-bbw/meta-bbw
mkdir -p ~/yocto-bbw/meta-bbw/recipes-bsp/device-tree/files
cd ~/yocto-bbw/meta-bbw
mkdir conf
cat > conf/layer.conf << 'EOF'
# We have a conf and classes directory, add to BBFILES
BBFILES += "${LAYERDIR}/recipes*//.bb ${LAYERDIR}/recipes*//.bbappend"
# Layer collection name
BBFILE_COLLECTIONS += "bbw"
BBFILE_PATTERN_bbw = "^${LAYERDIR}/"
BBFILE_PRIORITY_bbw = "6"
# Layer compatibility
LAYERSERIES_COMPAT_bbw = "kirkstone"
EOF

mkdir -p meta-bbw/recipes-bsp/device-tree/files
cd meta-bbw/recipes-bsp/device-tree/files

cat > w1-gpio-bbw.dts << 'EOF'
/dts-v1/;
/plugin/;
/ {
compatible = "ti,beaglebone", "ti,beaglebone-white", "ti,am33xx";
fragment@0 {
target = <&am33xx_pinmux>;
overlay {
w1_pins: pinmux_w1_pins {
pinctrl-single,pins = <
0x0a4 0x37  /* P8_11 (GPIO 45, gpio1_13), mode 7 (GPIO), pull-up */

;
};
};
};

fragment@1 {
target-path = "/";
overlay {
onewire {
compatible = "w1-gpio";
pinctrl-names = "default";
pinctrl-0 = <&w1_pins>;
gpios = <&gpio1 13 0>;  /* GPIO 45, active high */
status = "okay";
};
};
};
};
EOF

cd ../
mkdir device-tree
cat > device-tree/w1-gpio-bbw.bb << 'EOF'
SUMMARY = "1-Wire DT overlay for DS18B20 on BeagleBone White P8_11"
LICENSE = "MIT"
LIC_FILES_CHKSUM = "file://${COMMON_LICENSE_DIR}/MIT;md5=0835ade698e0bcf8506ecda2f7b4f302"
SRC_URI = "file://w1-gpio-bbw.dts"
inherit device-tree
COMPATIBLE_MACHINE = "beaglebone"
EOF

cd ~/yocto-bbw/build
nano conf/bblayers.conf
Append:
${TOPDIR}/../meta-bbw \

(exit etc..)
cd ~/yocto-bbw
source poky/oe-init-build-env build

bitbake core-image-minimal


nico@dell:~/yocto-bbw/build$
nico@dell:~/yocto-bbw/build$ ls -l deploy-ti/images/beaglebone/core-image-minimal-beaglebone.wic.xz
lrwxrwxrwx 2 nico nico 58 Sep 20 10:48 deploy-ti/images/beaglebone/core-image-minimal-beaglebone.wic.xz -> core-image-minimal-beaglebone-20250920084225.rootfs.wic.xz

nico@dell:~/yocto-bbw/build$ ls -l tmp/work/beaglebone-poky-linux-gnueabi/core-image-minimal/1.0-r0/deploy-core-image-minimal-image-complete/core-image-minimal-beaglebone.wic.xz
lrwxrwxrwx 2 nico nico 58 Sep 20 10:48 tmp/work/beaglebone-poky-linux-gnueabi/core-image-minimal/1.0-r0/deploy-core-image-minimal-image-complete/core-image-minimal-beaglebone.wic.xz -> core-image-minimal-beaglebone-20250920084225.rootfs.wic.xz

nico@dell:~/yocto-bbw/build$ ls -l deploy-ti/images/beaglebone/*.wic*
-rw-r--r-- 2 nico nico     4047 Sep 20 10:48 deploy-ti/images/beaglebone/core-image-minimal-beaglebone-20250920084225.rootfs.wic.bmap
-rw-r--r-- 2 nico nico 99315240 Sep 20 10:48 deploy-ti/images/beaglebone/core-image-minimal-beaglebone-20250920084225.rootfs.wic.xz
lrwxrwxrwx 2 nico nico       60 Sep 20 10:48 deploy-ti/images/beaglebone/core-image-minimal-beaglebone.wic.bmap -> core-image-minimal-beaglebone-20250920084225.rootfs.wic.bmap
lrwxrwxrwx 2 nico nico       58 Sep 20 10:48 deploy-ti/images/beaglebone/core-image-minimal-beaglebone.wic.xz -> core-image-minimal-beaglebone-20250920084225.rootfs.wic.xz
nico@dell:~/yocto-bbw/build$ ls -l tmp/work/beaglebone-poky-linux-gnueabi/core-image-minimal/1.0-r0/deploy-core-image-minimal-image-complete/*.wic*
-rw-r--r-- 2 nico nico     4047 Sep 20 10:48 tmp/work/beaglebone-poky-linux-gnueabi/core-image-minimal/1.0-r0/deploy-core-image-minimal-image-complete/core-image-minimal-beaglebone-20250920084225.rootfs.wic.bmap
-rw-r--r-- 2 nico nico 99315240 Sep 20 10:48 tmp/work/beaglebone-poky-linux-gnueabi/core-image-minimal/1.0-r0/deploy-core-image-minimal-image-complete/core-image-minimal-beaglebone-20250920084225.rootfs.wic.xzlrwxrwxrwx 2 nico nico       60 Sep 20 10:48 tmp/work/beaglebone-poky-linux-gnueabi/core-image-minimal/1.0-r0/deploy-core-image-minimal-image-complete/core-image-minimal-beaglebone.wic.bmap -> core-image-minimal-beaglebone-20250920084225.rootfs.wic.bmap
lrwxrwxrwx 2 nico nico       58 Sep 20 10:48 tmp/work/beaglebone-poky-linux-gnueabi/core-image-minimal/1.0-r0/deploy-core-image-minimal-image-complete/core-image-minimal-beaglebone.wic.xz -> core-image-minimal-beaglebone-20250920084225.rootfs.wic.xz



