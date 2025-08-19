import socket
import json
from Crypto.Util import number


# know_port = 50002


def gen_prime(keylenght=2048):
    return number.getPrime(keylenght)


def generator(g=2):
    valid_generators = [2, 3, 5, 7]
    if g not in valid_generators:
        raise ValueError("Invalide Generator !")
    else:
        return g


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


def get_conn(sock: socket.socket, p: int, info: dict, address: tuple):
    if info["status"] == "check":
        g = generator()
        sock.sendto(f"{p} {g}".encode(), address)

    elif info["status"] == "pubkey":
        # Réception de la clé publique du client
        pending_keys[address] = info["pubkey"]

    elif info["status"] == "ready":
        # Vérifie si la clé publique a été reçue avant
        pubkey = pending_keys.get(address)
        if not pubkey:
            print(f"[-] Clé publique manquante pour {address}")

        sock.sendto(b"ready", address)
        if info["method"] == "hole":
            client_data = (info["ip_pub"], info["sport"], info["username"], pubkey)
            clients.append((address, client_data))
        else:
            client_data = (info["ip_pub"], info["dport"], info["username"], pubkey)
            clients.append((address, client_data))

        if len(clients) >= 2:
            (addr1, data1), (addr2, data2) = clients.pop(0), clients.pop(0)

            # Envoie des infos croisées
            # Format: IP, port, public_key, username
            msg1 = f"{data2[0]} {data2[1]} {data2[3]} {data2[2]}"
            msg2 = f"{data1[0]} {data1[1]} {data1[3]} {data1[2]}"
            sock.sendto(msg1.encode(), addr1)
            sock.sendto(msg2.encode(), addr2)

            print(f"[*] Clients connectés via {info['method']}")


clients = []  # Stocke (ip, port, username, public_key)
pending_keys = {}  # address -> public_key temporairement


def main(sock: socket.socket, p: int):
    while True:
        data, address = sock.recvfrom(4096)
        info = parser(data, address)
        print(f"[+] Reçu de {address}: {info}")

        get_conn(sock, p, info, address)


if __name__ == "__main__":
    p = gen_prime()
    sock = init_sock()
    main(sock, p)
