# FinchBerryOS Core Services

Welcome to the central nervous system of **FinchBerryOS**. This repository contains the essential system daemons that power the operating system's low-level logic, hardware orchestration, and state management.

Inspired by the robust architecture of Darwin/macOS but engineered for the modern Linux kernel, these services replace traditional monolithic init systems with a modular, entitlement-aware service mesh built in **Go**.

## 🏗 Architecture Overview

FinchBerryOS services are delivered as **`.serviced`** bundles. This approach ensures strict isolation and provides a clear separation between system logic and graphical applications (`.appd`).

### Design Principles:
* **XPC-First Communication:** All services communicate via structured, type-safe XPC (Cross-Process Communication) managed by `syscored`.
* **The "Udev-Whisperer" Model:** Hardware events are parsed once by `kmodsysd` and broadcasted system-wide.
* **Entitlement-Based Security:** Services are sandboxed. For instance, core system daemons (except for the WindowServer) are strictly forbidden from accessing the GPU.

---

## 🛰 The Core Daemons

| Service Bundle | Role | Key Responsibility |
| :--- | :--- | :--- |
| **`syscored.serviced`** | **The Heart** | The Init system (PID 1). Manages service lifecycles and acts as the XPC message broker. |
| **`configd.serviced`** | **The Secretary** | Maintains the `SCDynamicStore`. The single source of truth for system state (IPs, Power, Users). |
| **`kmodsysd.serviced`** | **The Mechanic** | The Udev whisperer. Handles hardware identification, driver loading (`libkmod`), and the I/O Registry. |
| **`diskarbitrationd.serviced`** | **The Porter** | Manages storage devices. Handles mounting, file system integrity, and disk events. |
| **`networkd.serviced`** | **The Diplomat** | Executes network logic. Manages Wi-Fi, DHCP, WireGuard tunnels, and Bitcoin P2P backgrounding. |
| **`securityd.serviced`** | **The Vault** | The guardian of secrets. Manages Keychains, cryptographic operations, and TLS certificates. |
| **`logd.serviced`** | **The Memory** | The high-performance unified logging engine for the entire OS. |

---

## 📦 Bundle Structure

Each service is organized into a standardized bundle format to ensure portability and atomic updates:

```text
Service.serviced/
└── Contents/
    ├── FBOS/           # Compiled Go binaries
    ├── Resources/       # Service-specific assets or caches
    ├── Info.json        # Metadata, dependencies, and launch constraints
    └── Entitlements.json # Security definitions (Capabilities, IPC access)