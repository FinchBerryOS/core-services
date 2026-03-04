# securityd
The security custodian of FinchBerryOS.

## Description
Manages sensitive data such as certificates, private keys, and passwords in a highly isolated environment.

## Core Responsibilities
* **Keychain Management:** Secure storage for user passwords and application secrets.
* **Cryptographic Services:** Performing signing and encryption operations for other daemons.
* **Certificate Validation:** Validating TLS certificates for NetKit and system updates.

## Interactions
* Works closely with the `Security.frameworkb`.
* Provides authentication keys for `networkd` (e.g., VPN credentials).