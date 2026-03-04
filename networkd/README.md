# networkd
Responsible for active network configuration and tunneling.

## Description
While configd knows the state, networkd executes the commands. It manages both physical and virtual network interfaces.

## Core Responsibilities
* **Connectivity:** Establishing Wi-Fi connections and handling DHCP handshakes.
* **VPN Management:** Handling WireGuard tunnels directly through the framework.
* **P2P Backgrounding:** Supporting Bitcoin P2P protocol handshakes and peer management in the background.

## Interactions
* Retrieves interface list from `kmodsysd`.
* Writes IP and connectivity results into `configd`.