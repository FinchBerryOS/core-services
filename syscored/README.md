# syscored
This bundle contains the central system controller for FinchBerryOS.

## Description
syscored acts as the primary init system (analogous to Apple's launchd). It is the first process started by the kernel (PID 1).

## Core Responsibilities
* **Process Management:** Starting, monitoring, and restarting all system daemons.
* **XPC Broker:** Facilitating Inter-Process Communication (IPC) between services and applications.
* **Sandboxing:** Enforcing entitlements when launching .serviced and .appd bundles.
* **Boot Sequencing:** Managing service dependencies during the system startup phase.

## Interactions
* Communicates with all daemons for health monitoring.
* Manages XPC sockets located at `/var/run/com.finchberry.syscored`.