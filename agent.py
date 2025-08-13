import threading
import pyfiglet
import signal
import sys
import traceback
import json
from binascii import hexlify

from utils.arg import arguments
from utils.crypto import DiffieHellman
from utils.user import generate_username, color_text
from utils.network import NetworkManager


class Agent:
    def __init__(
        self,
        method: str,
        anonymous: bool,
        search: str,
        username: str,
        server_ip: str,
        server_port: int,
        source_port: int,
        destination_port: int,
    ):
        self.method = method.lower()
        self.anonymous = anonymous
        self.search = search
        self.username = username
        self.server = server_ip
        self.server_port = server_port
        self.sport = source_port
        self.dport = destination_port

        if self.server is None and self.server_port is None:
            self.server = "51.143.219.149"
            self.server_port = 55555
            self.rendezvous = ("51.143.219.149", 55555)
        else:
            self.rendezvous = (server_ip, server_port)

        if anonymous or username == "":
            self.username = generate_username()

        self.sock = None
        self.upnp = None
        self.ip = None

        if self.sport is None:
            self.sport = 50001

        if self.dport is None:
            self.dport = 50002

        self.net = NetworkManager(sport=self.sport, dport=self.dport)

        signal.signal(signal.SIGINT, self.cleanup)

    def print_banner(self):
        """
        printing banner Sileo
        """
        
        ascii_art = pyfiglet.figlet_format("Sileo", font="slant")
        print(ascii_art)

    def cleanup(self, sig, frame):
        """
        function use to cleanup socker and port mapping if ctrl+c is press
        """

        print(color_text("\n[!] Caught termination signal, cleaning up...", "red"))

        if self.sock:
            try:
                self.sock.close()
                print(color_text("[*] Socket closed.", "yellow"))
            except Exception as e:
                print(color_text(f"[!] Error closing socket: {e}", "red"))

        if self.upnp:
            try:
                self.upnp.deleteportmapping(self.dport, "UDP")
                print(color_text("[*] UPnP port mapping removed.", "yellow"))
            except Exception as e:
                print(color_text(f"[!] Error removing UPnP mapping: {e}", "red"))

        sys.exit(0)

    def format_data(self, status, username, dport, method):
        """
        formatting data in dict for sending it to the RendezVous server
        """
        data = dict(status=status, id=username, dport=dport, method=method)
        return data

    def setup_hole_punching(self, data: dict) -> str:
        print(color_text("[*] UDP Hole punching start...", "yellow"))
        self.sock = self.net.init_sock()

        self.sock.sendto(json.dumps(data).encode(), self.rendezvous)

        data = self.sock.recv(4096).decode()
        p, g = data.strip().split(" ")
        p = int(p)
        g = int(g)

        dh = DiffieHellman(p=p)
        dh.default_generator
        dh.private_key = dh.gen_private_key(dh.p)
        dh_public = dh.get_public_key()

        pubkey_payload = {"status": "pubkey", "method": "hole", "pubkey": dh_public}
        self.sock.sendto(json.dumps(pubkey_payload).encode(), self.rendezvous)

        ready_payload = {
            "status": "ready",
            "method": "hole",
            "username": self.username,
            "sport": self.sport,
        }
        self.sock.sendto(json.dumps(ready_payload).encode(), self.rendezvous)

        # Wait for peer
        while True:
            data = self.sock.recv(4096).decode()
            if data.strip() == "ready":
                print(color_text("[*] Checked in with server, waiting", "yellow"))
                continue

            try:
                ip, sport, pubkey_other, peer_username = data.strip().split(" ")
                break
            except Exception as e:
                print(color_text(f"[-] Error parsing peer info: {data} ({e})", "red"))

        # Create shared key
        dh.derive_shared_key(int(pubkey_other))
        shared_key = dh.get_key()
        print(
            color_text(
                f"[+] Clé partagée dérivée : {hexlify(shared_key).decode()}", "green"
            )
        )

        self.ip = ip
        self.port = int(sport)

        print(f"Username distant: {peer_username}")
        self.net.hole_punching(self.ip)

        return peer_username, shared_key

    def setup_upnp(self, data: dict):
        print(color_text("[*] UPnP method start...", "yellow"))
        self.upnp = self.net.init_upnp()
        self.net.mapping_port(self.upnp)
        self.net.check_mapping(self.upnp)
        
        self.sock = self.net.init_sock()
        self.sock.sendto(json.dumps(data).encode(), self.rendezvous)
        
        data = self.sock.recv(4096).decode()
        p, g = data.strip().split(" ")
        p = int(p)
        g = int(g)

        dh = DiffieHellman(p=p)
        dh.default_generator
        dh.private_key = dh.gen_private_key(dh.p)
        dh_public = dh.get_public_key()

        pubkey_payload = {"status": "pubkey", "method": "upnp", "pubkey": dh_public}
        self.sock.sendto(json.dumps(pubkey_payload).encode(), self.rendezvous)

        ready_payload = {
            "status": "ready",
            "method": "upnp",
            "username": self.username,
            "dport": self.dport,
        }
        self.sock.sendto(json.dumps(ready_payload).encode(), self.rendezvous)

        while True:
            data = self.sock.recv(4096).decode()
            if data.strip() == "ready":
                print(color_text("[*] Checked in with server, waiting", "yellow"))
                continue

            try:
                dist_ip, dport, pubkey_other, peer_username = data.strip().split(" ")
                break
            except Exception as e:
                print(color_text(f"[-] Error parsing peer info: {data} ({e})", "red"))

        # Create shared key
        dh.derive_shared_key(int(pubkey_other))
        shared_key = dh.get_key()
        print(
            color_text(
                f"[+] Clé partagée dérivée : {hexlify(shared_key).decode()}", "green"
            )
        )

        self.ip = dist_ip
        self.port = int(dport)

        print(f"Username distant: {peer_username}")
        return peer_username, shared_key        

    def start(self):
        self.print_banner()

        print(color_text(f"[*] Your local IP: {self.net.get_local_ip()}", "yellow"))
        print(
            color_text(f"[*] Using connection method: {self.method.upper()}", "yellow")
        )

        try:
            if self.method == "hole":
                data = self.format_data(
                    status="check",
                    username=self.username,
                    dport=None,
                    method=self.method,
                )
                client_username, aes_key = self.setup_hole_punching(data)
            elif self.method == "upnp":
                print("[-] UPNP not implemented !")
                
                # prepare data (dict)
                data = self.format_data(
                    status="check",
                    username=self.username,
                    dport=self.dport,
                    method=self.method
                )
                
                client_username, aes_key = self.setup_upnp(data)
            elif self.method == "both":
                try:
                    data = self.format_data(
                    status="check",
                    username=self.username,
                    dport=None,
                    method=self.method,
                )
                    self.setup_hole_punching(data)
                except OSError:
                    print(
                        color_text(
                            "[-] Hole punching failed, switching to UPnP.", "red"
                        )
                    )
                    self.setup_upnp()
            else:
                print(color_text("[-] Invalid connection method. Exiting...", "red"))
                sys.exit(1)
        except Exception as e:
            print(color_text(f"[-] Unexpected error during setup: {e}", "red"))
            traceback.print_exc()
            sys.exit(1)

        # Start listener and sender
        print(color_text(f"[*] AES KEY: {aes_key}", "yellow"))
        threading.Thread(
            target=self.net.listener,
            args=(self.username, client_username, aes_key),
            daemon=True,
        ).start()
        self.net.sender(self.ip, self.port, self.username, aes_key)


if __name__ == "__main__":
    args = arguments()
    agent = Agent(
        args.method,
        args.anonymous,
        args.search,
        args.username,
        args.server_ip,
        args.server_port,
        args.source_port,
        args.destination_port,
    )
    agent.start()
