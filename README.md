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

Add auto Wifi:
/etc/network/interfaces:
# Wireless interfaces
auto wlan0
iface wlan0 inet dhcp
        wireless_mode managed
        wireless_essid MYSSID
        wpa-driver wext
        wpa-conf /etc/wpa_supplicant.conf
/etc/wpa_supplicant.conf:
ctrl_interface=/var/run/wpa_supplicant
ctrl_interface_group=0
update_config=1
country=MYCOUNTRY
network={
        ssid="MYSSID"
        psk="MYPSK"
        key_mgmt=WPA-PSK
        priority=1
}

Add auto temp_server service:
cat > /etc/init.d/wwwtemp <<'EOF'
#!/bin/sh
### BEGIN INIT INFO
# Provides:          wwwtemp
# Required-Start:    $remote_fs $syslog
# Required-Stop:     $remote_fs $syslog
# Default-Start:     2 3 4 5
# Default-Stop:      0 1 6
# Short-Description: Start/stop wwwtemp
# Description:       A simple init script to start/stop wwwtemp on boot
### END INIT INFO

# Path to the binary
DAEMON=/usr/bin/wwwtemp
# Name of the service
NAME=wwwtemp
# PID file to track the process
PIDFILE=/var/run/$NAME.pid

# Check if the binary exists
test -x $DAEMON || exit 0

case "$1" in
  start)
    echo "Starting $NAME..."
    if [ -f $PIDFILE ]; then
      echo "$NAME is already running."
      exit 1
    fi
    start-stop-daemon --start --quiet --background --make-pidfile --pidfile $PIDFILE --exec $DAEMON
    if [ $? -eq 0 ]; then
      echo "$NAME started."
    else
      echo "Failed to start $NAME."
      exit 1
    fi
    ;;
  stop)
    echo "Stopping $NAME..."
    start-stop-daemon --stop --quiet --pidfile $PIDFILE
    if [ $? -eq 0 ]; then
      rm -f $PIDFILE
      echo "$NAME stopped."
    else
      echo "Failed to stop $NAME."
      exit 1
    fi
    ;;
  restart)
    $0 stop
    sleep 1
    $0 start
    ;;
  *)
    echo "Usage: $0 {start|stop|restart}"
    exit 1
    ;;
esac

exit 0
EOF

chmod +x /etc/init.d/wwwtemp
cp /home/root/temp_server_armv7 /usr/bin/wwwtemp
chmod +x /usr/bin/wwwtemp
update-rc.d wwwtemp defaults

# Support Netgear WLAN Grey USB adapter
firmware needed: ar5523.bin

