# **Sileo** — Secure, Ephemeral & Peer-to-Peer Messaging

> **Fast. Confidential. Traceless.**  
> Sileo is an ephemeral instant messaging service, it's made for fast and anonymous communication. All the discussion is cipher, erase at the end and it's **peer to peer** (P2P).
> Thanks to **Hole Punching** and **UPnP** (Implemented old python version but not in the recent one) techniques, your messages pass directly between you and your interlocutor — no server stores your exchanges.

---

## **Main features**
- **100% Peer-to-Peer** — No intermediate : you talk directly to your contact.
- **Double connection method** :
  - `hole` : **Hole Punching** pur for NAT traversal.
  - `upnp` : Automatic configuration of your router to open the necessary ports.
- **End to End Encryption** (E2EE) for confidentiality.
- **Ephemeral messages** : no local or remote storage, nothing is retained.
- **Anonymous** : A random or chosen pseudonym, with no connection to your real identity.

---

## **Installation**
### From Source
```bash
git clone https://github.com/Warziz/sileo.git
cd Sileo/sileo
cargo build
./sileo.exe
```

## Available Arguments

| Argument           | Type  | Défaut  | Description                                                                |
| -------------------| ----- | ------- | -------------------------------------------------------------------------- |
| `method`           | `str` | `both`  | Connection method : `both`, `hole`, `upnp`.                                |
| `anonymous`        | `bool`| `True`  | Use random pseudonym (anonymous mode).                                     |
| `username`         | `str` | *None*  | Choose your pseudonym.                                                     |
| `server-ip`        | `str` | *None*  | Appointment server IP address (must be the same for both contacts).        |
| `server-port`      | `int` | *None*  | Appointment server port.                                                   |
| `source-port`      | `int` | `50001` | Source port for Hole Punching or intern port for UPnP.                                     |
| `destination-port` | `int` | `50002` | Destination port for Hole Punching or extern port for UPnP.                                |

## :warning: Meeting Server :warning:
The meeting server is still a PoC and use for testing. Improvement are coming...