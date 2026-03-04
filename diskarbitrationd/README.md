# diskarbitrationd
Manages all mass storage devices and file systems.

## Description
This service detects new block devices and ensures that partitions are mounted safely according to defined system rules.

## Core Responsibilities
* **Mount Management:** Dynamically mounting USB drives, NVMe storage, and SD cards.
* **FS Checking:** Automatically verifying file system integrity before mounting.
* **Disk Arbitration:** Notifying the system about available storage capacities and mount points.

## Interactions
* Listens for block device events from `kmodsysd`.
* Reports mount paths to `configd`.