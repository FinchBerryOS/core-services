# logd
Central logging daemon for the FinchBerry Unified Logging System.

## Description
logd collects all log messages from the system and applications, prioritizes them, and stores them in a structured format.

## Core Responsibilities
* **Centralization:** Aggregates logs from all .serviced and .appd units via XPC.
* **Performance:** Utilizes memory ring-buffers for high-efficiency logging without disk I/O bottlenecks.
* **Structuring:** Allows filtering by subsystems (e.g., com.finchberry.netkit) and categories.

## Interactions
* Serves as the backend for the `CoreSystem.frameworkb` (cs_log).