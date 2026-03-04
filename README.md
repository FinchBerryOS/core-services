# FinchBerryOS Core Services & Launch Infrastructure

Welcome to the central nervous system of **FinchBerryOS**. This repository manages the system binaries, high-level service bundles, and the launch configurations that orchestrate the transition from the Linux kernel to a fully functional, sovereign userland.

## 🏗 Service Hierarchy & Architecture

FinchBerryOS employs a tiered architectural approach to separate critical boot logic from high-level system services and UI components.

### 1. The Boot & Core Layer (Naked Binaries)
Essential system components located in traditional Unix paths. These are "naked" binaries without bundled resources.
* **`/sbin/syscored`**: The heart of the OS (PID 1). Managed directly by the kernel.
* **`/usr/sysbin/`**: Core system daemons providing base functionality (e.g., `configd`, `networkd`).

### 2. The High-Level Layer (Bundles)
Located in `/System/Library/CoreServices`, this layer contains complex services packaged as bundles to include resources, localized strings, and metadata.
* **`.serviceb` (Service Bundles)**: Integrated background services with resource requirements (e.g., `Bluetooth.serviceb`).
* **`.appb` (App Bundles)**: Critical system-level UI applications (e.g., `WindowServer.appb`, `LoginUI.appb`).

### 3. Launch Configurations
`syscored` scans these directories to manage the lifecycle, dependencies, and entitlements of all services.
* **`/System/Library/LaunchDaemons/` & `LaunchAgents/`**: Immutable system service definitions.
* **`/Library/LaunchDaemons/` & `LaunchAgents/`**: Administrator or third-party service definitions.

---

## 🛰 The Core Service Map

| Location | Binary/Bundle | Role | Responsibility |
| :--- | :--- | :--- | :--- |
| **`/sbin/`** | `syscored` | **PID 1 / Heart** | Init system, service supervisor, and XPC Broker. |
| **`/usr/sysbin/`** | `configd` | **Daemon** | Maintains the `SCDynamicStore` (System state). |
| **`/usr/sysbin/`** | `networkd` | **Daemon** | Networking stack, WireGuard, and IP management. |
| **`/usr/sysbin/`** | `kmodsysd` | **Daemon** | Hardware registry and kernel module orchestration. |

---

## 📂 Filesystem Structure

```text
/
├── sbin/
│   └── syscored               # Kernel-invoked Init process
├── usr/
│   └── sysbin/                # Core "Naked" Daemons
│       ├── configd
│       ├── networkd
│       └── logd
├── System/
│   └── Library/
│       ├── CoreServices/      # High-Level Bundles (.serviceb, .appb)
│       │   ├── Bluetooth.serviceb
│       │   └── WindowServer.appb
│       ├── LaunchDaemons/     # System service configurations
│       └── LaunchAgents/      # System user-agent configurations
└── Library/
    ├── LaunchDaemons/         # Third-party service configurations
    └── LaunchAgents/          # Third-party user-agent configurations