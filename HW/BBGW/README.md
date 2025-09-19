find /proc/device-tree/ -name '*w1*'
cat /proc/device-tree/onewire/gpios


root@BBGW:~# find /proc/device-tree/ -name '*w1*'      
root@BBGW:~# lsmod|grep w1                             
w1_therm               28672  0                        
w1_gpio                16384  0                        
root@BBGW:~#                                           

# Install DT Compiler
sudo apt install device-tree-compiler

# Create DT Overlay (BBGW for P9_12, GPIO 60):
cat > w1-gpio-bbgw.dts << EOF
/dts-v1/;
/plugin/;
/ {
compatible = "ti,beaglebone", "ti,beaglebone-black", "ti,beaglebone-green", "ti,am33xx";
fragment@0 {
target = <&am33xx_pinmux>;
overlay {
w1_pins: pinmux_w1_pins {
pinctrl-single,pins = <0x078 0x37>;  // P9_12 (GPIO 60, gpio1_28), mode 7 (GPIO), pull-up
};
};
};
fragment@1 {
target-path = "/";
overlay {
w1_bus {
compatible = "w1-gpio";
gpios = <&gpio1 28 GPIO_ACTIVE_HIGH>;  // GPIO 60
status = "okay";
};
};
};
};
EOF

# compile
dtc -@ -I dts -O dtb -o /lib/firmware/w1-gpio-bbgw.dtbo w1-gpio-bbgw.dts


#Create DT Overlay (BBW for P8_11, GPIO 45):
cat > /home/debian/w1-gpio-bbw.dts << EOF
/dts-v1/;
/plugin/;
/ {
compatible = "ti,beaglebone", "ti,beaglebone-white", "ti,am33xx";
fragment@0 {
target = <&am33xx_pinmux>;
overlay {
w1_pins: pinmux_w1_pins {
pinctrl-single,pins = <0x0a4 0x37>;  // P8_11 (GPIO 45, gpio1_13), mode 7 (GPIO), pull-up
};
};
};
fragment@1 {
target-path = "/";
overlay {
w1_bus {
compatible = "w1-gpio";
gpios = <&gpio1 13 GPIO_ACTIVE_HIGH>;  // GPIO 45
status = "okay";
};
};
};
};
EOF

# compile
dtc -@ -I dts -O dtb -o /lib/firmware/w1-gpio-bbw.dtbo /home/debian/w1-gpio-bbw.dts

# Enable DTBO:
- For BBGW:
sudo nano /boot/uEnv.txt
Add:
uboot_overlay_addr4=/lib/firmware/w1-gpio-bbgw.dtbo
Or:
uboot_overlay_addr4=/lib/firmware/w1-gpio-bbw.dtbo

sudo reboot
