# ros-instrumentation

# Overview

This project aims to show how to run the rCore operating systems with  instrumentation to observe the interaction between rCore operating system and application in detail.

# Platform

Ubuntu20.04

Docker

# Steps to run the project

1. Clone the project on GitHub locally.Command:

   $  git clone git@github.com:gtsfs7090/rOS-Instrumentation.git

2. Enter the rOS-Instrumentation directory.Command:

   $  cd rOS-Instrumentation

3. Switch branch to master.Command:

  $  git checkout master

4. Build Dockerfile to generate the image ros-instrument:1.0,Command:

   $  docker build -t ros-instrument:1.0 .

5. Execute the ros-instrument:1.0 image. Command:

   $  docker run --name ro1 -it ros-instrument:1.0 sh

6. Enter the /home/os directory. Command:

   $  cd /home/os

7. Execute rCore with instrumentation. Command:

   $  qemu-system-riscv64 \
    -machine virt \
    -nographic \
    -bios ../bootloader/rustsbi-qemu.bin \
-device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000
