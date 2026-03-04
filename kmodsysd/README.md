# kmodsysd
Kernel Module System Daemon – The interface between hardware and software.

## Description
kmodsysd is the primary hardware parser. It monitors udev events and manages the system's driver landscape.

## Core Responsibilities
* **Udev Parsing:** Analyzing Netlink events from the kernel to identify hardware.
* **Driver Management:** Automatically loading and unloading kernel modules (.ko) via libkmod.
* **I/O Registry:** Building and maintaining the hierarchical hardware tree for `IOKit.frameworkb`.
* **PCI Passthrough:** Dynamically reassigning hardware (e.g., GPUs) to the VFIO driver at runtime.

## Interactions
* Reports new hardware to `configd` and `diskarbitrationd`.
* Services requests from the `IOKit.frameworkb`.