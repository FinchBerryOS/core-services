# configd
System Configuration Daemon – The guardian of the dynamic system state.

## Description
configd maintains the SCDynamicStore, an in-memory database that represents the current state of the operating system.

## Core Responsibilities
* **State Management:** Storing IP addresses, hostnames, active user sessions, and power status.
* **Watch System:** Allows applications to "subscribe" to keys and receive XPC notifications when values change.
* **Plugin Architecture:** Modular extensions for specific system configurations (e.g., DNS, DHCP).

## Interactions
* Receives hardware updates from `kmodsysd`.
* Provides network state data to the `NetKit.framework`.