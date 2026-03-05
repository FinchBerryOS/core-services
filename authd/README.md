### authd

**The System Gatekeeper.** `authd` is the central authority for authorization, session tokens, and entitlement verification on FinchBerryOS. It works in close coordination with `identityd` and `securityd` to ensure that every LXPC request is performed by a verified and authorized actor.

* **Role:** Authorization & Session Manager
* **Framework Partner:** `IdentityKit.frameworkb`
* **Communication:** LXPC (Linux-XPC)

---

#### 🛠 Core Responsibilities

`authd` is responsible for moving the system beyond simple user/password checks into a robust, capability-based security model.

* **Token Issuance:** Generates cryptographically signed, short-lived session tokens upon successful login via `IdentityKit`.
* **Entitlement Verification:** The definitive judge of "Rights." When a daemon receives a request, it asks `authd` to verify if the client process possesses the required **Entitlements** (e.g., `com.finchberry.storage.mount`).
* **Privilege Elevation:** Manages the logic for temporary privilege escalation (e.g., a user-level process requesting a privileged container from `linuxcontainerd`).
* **Session Lifecycle:** Tracks active sessions across namespaces. It informs `syscored` when a session is timed out or revoked, ensuring orphan processes are cleaned up.
* **Policy Enforcement:** Evaluates system-wide security policies defined in `Foundation` to determine if specific actions (like system updates or hardware configuration changes) are permitted under current conditions.

---

#### 🏗 Interaction Flow

1.  **Login:** During the login phase, `identityd` confirms the user exists, and `authd` issues a unique **Session Token**.
2.  **Request:** An app calls a privileged function in `NetKit`. `NetKit` (the client side) attaches the session token to the LXPC message.
3.  **Check:** The `networkd` daemon receives the message and sends a "Check Entitlement" request to `authd` via LXPC.
4.  **Verdict:** `authd` verifies the token and the process's entitlements, returning an `ALLOW` or `DENY` response to `networkd`.



---

#### 📂 File & Socket Location
* **Binary:** `/usr/libexec/authd`
* **Control Socket:** `/var/run/com.finchberry.authd.socket`
* **Security Database:** `/var/db/auth.db` (Managed via SecurityKit)