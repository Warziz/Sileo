import socket
import sys
import miniupnpc
import traceback

from datetime import datetime, timezone

from .user import color_text
from .crypto import Cipher


class NetworkManager:
    def __init__(self, sport: int, dport: int, protocol: str = "UDP"):
        self.sport = sport
        self.dport = dport
        self.protocol = protocol
        self.upnp = None

    # -------- Socket & IP Functions -------- #
    def init_sock(self) -> socket.socket:
        self.sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        self.sock.bind(("0.0.0.0", self.sport))
        return self.sock

    @staticmethod
    def get_local_ip() -> str:
        hostname = socket.gethostname()
        return socket.gethostbyname(hostname)

    # -------- UPNP Functions -------- #
    def init_upnp(self):
        upnp = miniupnpc.UPnP()
        upnp.discoverdelay = 200
        upnp.discover()
        upnp.selectigd()

        external_ip = upnp.externalipaddress()
        print(color_text(f"[*] IP Publique : {external_ip}","yellow"))
        return upnp

    def mapping_port(self, upnp: miniupnpc):
        if not upnp:
            raise RuntimeError("UPnP non initialisé. Appelez init_upnp() d'abord.")
        upnp.addportmapping(
            self.dport, self.protocol, upnp.lanaddr, self.sport, "Sileo", ""
        )
        print(color_text(f"[*] Port {self.dport} redirigé vers {upnp.lanaddr}:{self.sport}","yellow"))

    def check_mapping(self, upnp: miniupnpc):
        if not upnp:
            raise RuntimeError("UPnP non initialisé.")
        for i in range(10):
            mapping = upnp.getspecificportmapping(i, self.protocol)
            if mapping:
                print(color_text(f"[*] Port {i} : {mapping}","yellow"))

    def delete_mapping(self, upnp: miniupnpc):
        if not upnp:
            raise RuntimeError("UPnP non initialisé.")
        self.upnp.deleteportmapping(self.dport, self.protocol)
        print(color_text(f"[*] Port {self.dport} fermé.","yellow"))

    # -------- Hole Punching Functions -------- #
    def hole_punching(self, ip: str):
        print(color_text("\n[+] Got peer", "green"))
        print(color_text(f"[*] ip: {ip}", "yellow"))
        print(color_text(f"[*] source port: {self.sport}", "yellow"))
        print(color_text(f"[*] destination port: {self.dport}", "yellow"))

        print(color_text("[!] Punching Hole", "magenta"))
        self.sock.sendto(b"CTRL:PUNCH", (ip, self.sport))
        print(color_text("[+] Ready to exchange !", "green"))

    def listener(self, username: str, client_username: str, aes_key: bytes):
        while True:
            utc_now = datetime.now(timezone.utc)
            time_str = utc_now.strftime("%Y%m%d-%H%M")
            try:
                data = self.sock.recv(1024)  # changer taille
                if data == b"CTRL:PUNCH":
                    continue
                else:
                    decrypt = Cipher.decrypt_message(aes_key=aes_key, data=data)
                    sys.stdout.write("\r" + " " * 80 + "\r")
                    sys.stdout.write(
                        color_text(
                            f"[{time_str}] - {client_username} > {decrypt}\n", "cyan"
                        )
                    )
                    sys.stdout.write(
                        color_text(f"[{time_str}] - {username}(you) > ", "green")
                    )
                    sys.stdout.flush()
            except Exception as e:
                print(color_text(f"Erreur réception: {e}", "red"))
                traceback.print_exc()
                break

    def sender(self, target_addr: str, port: int, username: str, aes_key: bytes):
        print(color_text(f"Connexion avec {target_addr}...", "yellow"))
        while True:
            utc_now = datetime.now(timezone.utc)
            time_str = utc_now.strftime("%Y%m%d-%H%M")
            msg = input(color_text(f"[{time_str}] - {username}(you) > ", "green"))
            encrypted = Cipher.encrypt_message(aes_key=aes_key, message=msg)
            self.sock.sendto(encrypted, (target_addr, port))
