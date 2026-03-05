### linuxcontainerd

**The Virtualization & Isolation Engine.** `linuxcontainerd` is the privileged system daemon responsible for creating and managing isolated execution environments on FinchBerryOS. It serves as the primary backend for the **ContainerKit** framework, translating high-level requests into native Linux kernel operations.

* **Role:** System Container Supervisor
* **Framework Partner:** `ContainerKit.frameworkb`
* **Communication:** LXPC (Linux-XPC)

---

#### 🛠 Core Responsibilities

`linuxcontainerd` handles the low-level heavy lifting required to separate processes, filesystems, and networks.

* **Namespace Orchestration:** Manages the creation and entry of Linux namespaces, including:
    * **Mount (mnt):** Providing isolated filesystem views.
    * **Process (pid):** Hiding system processes from the containerized environment.
    * **Network (net):** Setting up virtual interfaces and private routing tables.
    * **User (user):** Mapping internal root IDs to unprivileged host users for "rootless" operation.
* **Resource Constraints (cgroups):** Directly interfaces with Control Groups to enforce strict limits on:
    * **CPU:** Capping cycles and assigning cores.
    * **Memory:** Setting hard limits and swap boundaries.
    * **I/O:** Prioritizing disk access and bandwidth.
* **Lifecycle Management:** Tracks the health and state of all active containers, ensuring they are cleanly terminated or restarted by `syscored` when necessary.
* **Security Policy Enforcement:** Validates **Entitlements** via `authd` to decide if a process is allowed to request privileged operations (e.g., hardware passthrough or raw socket access).

---

#### 🏗 Interaction Flow

1.  **Request:** An application uses `ContainerKit` to request a new isolated sandbox.
2.  **Verification:** `linuxcontainerd` receives the request via LXPC and verifies the application's identity and permissions with `authd`.
3.  **Execution:** The daemon invokes kernel syscalls (`clone`, `unshare`) and configures the `cgroup` hierarchy.
4.  **Handover:** The newly isolated environment is handed over to the user-level `syscored --user` to manage the local session.



---

#### 📂 File & Socket Location
* **Binary:** `/usr/libexec/linuxcontainerd`
* **Control Socket:** `/var/run/com.finchberry.linuxcontainerd.socket`
* **State Directory:** `/var/lib/containers/`