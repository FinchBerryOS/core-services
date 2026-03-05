### notifyd

**The System Notification Center.** `notifyd` is the central nervous system for asynchronous event signaling on FinchBerryOS. It provides a lightweight, high-performance mechanism for daemons and applications to "post" and "subscribe" to system-wide status changes without the overhead of direct polling or complex IPC state management.

* **Role:** System Event Broker & Signal Dispatcher
* **Framework Partner:** `Foundation.frameworkb` / `CoreSystem`
* **Communication:** LXPC (Linux-XPC)

---

#### 🛠 Core Responsibilities

`notifyd` ensures that the various components of FinchBerryOS stay synchronized by broadcasting state changes in real-time.

* **Event Broadcasting:** Manages a registry of named system events (e.g., `com.finchberry.network.link-up`) and dispatches them to all interested listeners.
* **Subscription Management:** Tracks which processes are listening for which tokens. It supports:
    * **Signal Delivery:** Waking up a process via a standard Unix signal.
    * **Socket Delivery:** Sending a small LXPC packet directly to a listener's port.
    * **Mach-style Ports:** Optimized notification delivery for high-frequency events.
* **State Mirroring:** In coordination with `configd`, `notifyd` can trigger notifications specifically when registry keys are modified, serving as the "Watch" trigger for system configuration.
* **Power Efficiency:** By allowing daemons to sleep until a specific notification is received, `notifyd` significantly reduces CPU wakeups and battery consumption.

---

#### 🏗 Interaction Flow (The "Power Handler" Example)

1. **Post**: `kioskwindow` starts up and updates the power handler in `configd`. `configd` then calls `notify_post("com.finchberry.system.power-handler-changed")`.
2. **Dispatch**: `notifyd` receives the post and looks up its internal registration table.
3. **Notify**: `notifyd` finds that `powerd` is a subscriber and sends a lightweight LXPC "ping" to `powerd`.
4. **React**: `powerd` wakes up, sees the notification, and refreshes its cache from `configd` to know that `kioskwindow` is now in charge.



---

#### ⚡ Notification Types

| Type | Description | Use Case |
| :--- | :--- | :--- |
| **Distributed** | Broadcast to all listeners across the system. | Network state changes, Disk mounts. |
| **Localized** | Only delivered to listeners within a specific User Namespace. | Clipboard updates, User preference changes. |
| **System** | High-priority events from the kernel or `syscored`. | Shutdown warnings, Thermal alerts. |

---

#### 📂 File & Socket Location
* **Binary:** `/usr/libexec/notifyd`
* **Control Socket:** `/var/run/com.finchberry.notifyd.socket`