# FinchBerryOS Core Services & Launch Infrastructure

Welcome to the central nervous system of **FinchBerryOS**. This repository manages the system binaries, high-level service bundles, and the launch configurations that orchestrate the transition from the Linux kernel to a fully functional, sovereign userland.

## 🏗 Service Hierarchy & Architecture

FinchBerryOS employs a tiered architectural approach to separate critical boot logic from high-level system services and UI components.

### 1. The Boot & Machine Layer (Naked Binaries)
Essential system components and "invisible" workers. These are naked binaries located in protected system paths, hidden from the user's default execution path (`$PATH`).
* **`/sbin/syscored`**: The heart of the OS (PID 1). Invoked directly by the kernel.
* **`/usr/libexec/`**: The system's "engine room." Contains core daemons and internal maintenance tools (e.g., `rszprt`).

### 2. The High-Level Layer (Bundles)
Located in `/System/Library/CoreServices`, this layer contains complex services packaged as bundles to include resources, assets, and metadata.
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
| **`/usr/libexec/`** | `configd` | **Daemon** | Maintains the `SCDynamicStore` (System state). |
| **`/usr/libexec/`** | `networkd` | **Daemon** | Networking stack, WireGuard, and IP management. |
| **`/usr/libexec/`** | `rszprt` | **One-Shot Tool** | Internal partition and filesystem resizing utility. |
| **`CoreServices/`** | `*.serviceb` | **Service Bundle** | Resource-heavy background services. |
| **`CoreServices/`** | `*.appb` | **App Bundle** | System-critical UI services (WindowServer, etc.). |

---

## 📂 Filesystem Structure



```text
/
├── sbin/
│   └── syscored               # Kernel-invoked Init process
├── usr/
│   └── libexec/               # The "Engine Room" (Internal Binaries)
│       ├── configd            # System Configuration Daemon
│       ├── networkd           # Network Management Daemon
│       ├── logd               # Unified Logging Daemon
│       └── rszprt             # Partition Resizer (One-Shot)
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