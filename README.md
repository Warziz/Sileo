# **Sileo** — Secure, Ephemeral & Peer-to-Peer Messaging

> **Fast. Confidential. Traceless.**  
> Sileo est une application de messagerie instantanée éphémère conçue pour des communications rapides, chiffrées et sans traces, directement **de pair à pair** (P2P).  
> Grâce à des techniques de **Hole Punching** et **UPnP**, vos messages transitent directement entre vous et votre interlocuteur — aucun serveur ne conserve vos échanges.

---

## ✨ **Caractéristiques principales**
- **100% Peer-to-Peer** — Pas d’intermédiaire : vous parlez directement à votre contact.
- **Double mode de connexion** :
  - `hole` : **Hole Punching** pur pour NAT traversal.
  - `upnp` : Configuration automatique de votre routeur pour ouvrir les ports nécessaires.
  - `both` : Test automatique des deux méthodes.
- **Chiffrement de bout en bout** (E2EE) pour préserver la confidentialité.
- **Messages éphémères** : aucun stockage local ou distant, rien n’est conservé.
- **Anonymat** : pseudonyme aléatoire ou choisi, aucun lien avec votre identité réelle.
- **Serveur de rendez-vous personnalisable** pour trouver vos pairs.

---

## 📦 **Installation**
```bash
git clone https://github.com/Warziz/sileo.git
cd sileo
pip3 install -r requirements.txt
python3 agent.py [OPTIONS]
```

## ⚙️ Arguments disponibles

| Argument             | Alias    | Type  | Défaut  | Description                                                                |
| -------------------- | -------- | ----- | ------- | -------------------------------------------------------------------------- |
| `--method`           | `-m`     | `str` | `both`  | Méthode de connexion : `both`, `hole`, `upnp`.                             |
| `--anonymous`        | `-a`     | flag  | `True`  | Utiliser un pseudonyme aléatoire (mode anonyme).                           |
| `--search`           | `-s`     | `str` | *None*  | Rechercher un utilisateur avec qui discuter.                               |
| `--username`         | `-u`     | `str` | *None*  | Choisir un pseudonyme personnalisé.                                        |
| `--server-ip`        | `-srv`   | `str` | *None*  | IP du serveur de rendez-vous (doit être identique pour les deux contacts). |
| `--server-port`      | `-srv_p` | `int` | *None*  | Port du serveur de rendez-vous.                                            |
| `--source-port`      | `-sp`    | `int` | `50001` | Port source pour Hole Punching ou UPnP.                                    |
| `--destination-port` | `-dp`    | `int` | `50002` | Port destination pour Hole Punching ou UPnP.                               |

## 🚀 Exemples d’utilisation

Mode Hole Punching pur

```
python3 agent.py -m hole -u "ShadowFox"
```

Mode UPnP avec port personnalisé
```
python3 agent.py -m upnp --source_port 60001 --destination_port 60002
```

Connexion anonyme avec serveur de rendez-vous spécifique
```
python3 agent.py -a -srv 192.168.1.10 -srv_p 5000
```