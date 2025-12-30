import socket
import json
from Crypto.Util import number
from collections import defaultdict

"""
def gen_prime(keylenght=2048):
    return number.getPrime(keylenght)


def generator(g=2):
    valid_generators = [2, 3, 5, 7]
    if g not in valid_generators:
        raise ValueError("Invalide Generator !")
    else:
        return g
"""

def init_sock():
    print("[*] Start listening")
    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    sock.bind(("0.0.0.0", 55555))

    return sock


def parser(data: bytes, address: tuple) -> dict:
    print(f"[+] Connection from: {address}")

    try:
        decoded_data = json.loads(data.decode())
    except json.JSONDecodeError as e:
        print(f"[-] JSON decode error: {e}")
        return {}

    decoded_data["ip_pub"] = address[0]
    decoded_data["sport"] = address[1]

    return decoded_data

def find_username(username:str, searcher_username:str):
    for client in clients:
        if client["username"] == username and client["search"] == searcher_username:
            return client
    return None

def get_conn(sock: socket.socket, info: dict, address: tuple):

    if info["status"] == "pubkey":
        # Réception de la clé publique du client
        pending_keys[address] = info["pubkey"]

    elif info["status"] == "ready":
        # Vérifie si la clé publique a été reçue avant
        pubkey = pending_keys.get(address)
        if not pubkey:
            print(f"[-] Clé publique manquante pour {address}")
        #faire le check des utilisateurs recherché ici.
        sock.sendto(b"ready", address)
        
        client_data = {
            "ip_pub": info["ip_pub"], 
            "port": info["sport"] if info["method"] == "hole" else info["dport"], 
            "username": info["username"], 
            "search": info["search"],
            "pubkey": pubkey
        }
        clients.append(client_data)
        print(clients)
 

        if len(clients) >= 2:
            data1, data2 = clients.pop(0), clients.pop(0)

            # Envoie des infos croisées
            # Format: IP, port, public_key, username
            msg1 = f"{data2["ip_pub"]} {data2["port"]} {data2["pubkey"]} {data2["username"]}"
            msg2 = f"{data1["ip_pub"]} {data1["port"]} {data1["pubkey"]} {data1["username"]}"
            sock.sendto(msg1.encode(), (data1["ip_pub"],data1["sport"]))
            sock.sendto(msg2.encode(), (data2["ip_pub"],data2["sport"]))

            print(f"[*] Clients connectés via {info['method']}")


clients = []  # Stocke (ip, port, username, public_key)
pending_keys = {}  # address -> public_key temporairement

def main(sock: socket.socket):
    while True:
        data, address = sock.recvfrom(4096)
        info = parser(data, address)
        print(f"[+] Reçu de {address}: {info}")

        get_conn(sock, info, address)


if __name__ == "__main__":

    sock = init_sock()
    main(sock)
