# FrambOS
Setup simple IoT Yocto system on Raspberry Pi 1 modelB rev 1
Inspired by this article: https://kaizen-solutions.net/kaizen-insights/articles-et-conseils-de-nos-experts/creer-un-systeme-linux-embarque-sur-mesure-avec-yocto/


```shell
mkdir ~/yocto-rpi1 && cd ~/yocto-rpi1
# yocto core
git clone -b kirkstone git://git.yoctoproject.org/poky.git
# for extra recipes like links:
git clone -b kirkstone git://git.openembedded.org/meta-openembedded
# Raspberry Pi BSP layer
git clone -b kirkstone git://git.yoctoproject.org/meta-raspberrypi

# will create and cd to build/
source poky/oe-init-build-env build

# Edit build/conf/bblayers.conf to include the layers
BBLAYERS += "${TOPDIR}/../meta-openembedded/meta-oe"
# if links dep missing try the following
BBLAYERS += "${TOPDIR}/../meta-openembedded/meta-networking"
BBLAYERS += "${TOPDIR}/../meta-openembedded/meta-python"
BBLAYERS += "${TOPDIR}/../meta-raspberrypi"

# Edit build/conf/local.conf for Raspberry Pi 1 configuration
MACHINE = "raspberrypi"
DISTRO = "poky"
# optional for debug
ENABLE_UART = "1"
# enable 1-Wire on default GPIO4
RPI_EXTRA_CONFIG = "\ndtoverlay=w1-gpio\n"
# Add simple web browser Lynx (see above for possible dep missing fix)
# IMAGE_INSTALL:append = " lynx"
# optional parallel build
BB_NUMBER_THREADS = "4"
PARALLEL_MAKE = "-j 4"

# If needed clean all
bitbake -c cleanall core-image-minimal
# bitbake -c cleanall lynx  # If it exists from prior attempts

# prerequisite, eg: for arch:
# sudo pacman -S chrpath cpio diffstat rpcsvc-proto
# prerequisite, eg: for ubuntu:
# sudo apt install chrpath cpio diffstat lz4 zstd
# Build the image
bitbake core-image-minimal

# Flash the image to SD Card -- BEWARE to use the correct device!
sudo umount /dev/sdX*  # Unmount any partitions
sudo dd if=build/tmp/deploy/images/raspberrypi/core-image-minimal-raspberrypi.wic of=/dev/sdX bs=4M status=progress conv=fsync
sudo sync
sudo eject /dev/sdX

# boot the Raspberry Pi
# check ifconfig or ip addr
systemctl enable ssh
# check w1_gpio and wire modules
lsmod | grep w1
# should show 28-XXXXXXXXXXXX
ls /sys/bus/w1/devices/
# read temperature
# (look for t=25000 in Celsius * 1000).
cat /sys/bus/w1/devices/28-XXXXXXXXXXXX/w1_slave
# if not detected:
# Check /boot/config.txt (on SD boot partition)
# includes dtoverlay=w1-gpio
# Manually load: modprobe w1-gpio and modprobe w1-therm.
# GPIO status: gpio readall
# (install wiringpi if needed, but not in minimal image).

# read temp on shell
cat /sys/bus/w1/devices/28-000001cda180/w1_slave | grep "t=" | cut -d'=' -f2 | awk '{print $1/1000}'

```


RS
- cross compile
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env
rustup target add armv6-unknown-linux-gnueabihf
sudo apt update
sudo apt install gcc-arm-linux-gnueabihf g++-arm-linux-gnueabihf
cargo new temp_server
cd temp_server
cargo add actix-web --features=openssl
cargo add openssl-sys
edit cargo.toml:
[dependencies]
actix-web = "4"

- setup rust in Yocto
TODO