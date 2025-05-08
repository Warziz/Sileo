
import threading
import pyfiglet
import signal
import sys

from utils.arg import arguments
from utils.user import generate_username
from utils.network import (
    get_local_ip, mapping_port, init_upnp, check_mapping,
    listener, sender, hole_punching, init_sock
)


class Agent:
    def __init__(self, method: str, anonymous: bool, search: str):
        self.method = method.lower()
        self.anonymous = anonymous
        self.search = search

        self.username = generate_username()
        self.sock = None
        self.upnp = None
        self.ip = None
        self.sport = 50001
        self.dport = 50002

        signal.signal(signal.SIGINT, self.cleanup)

    def print_banner(self):
        ascii_art = pyfiglet.figlet_format("Sileo", font="slant")
        print(ascii_art)

    def cleanup(self, sig, frame):
        print("\n[!] Caught termination signal, cleaning up...")

        if self.sock:
            try:
                self.sock.close()
                print("[*] Socket closed.")
            except Exception as e:
                print(f"[!] Error closing socket: {e}")

        if self.upnp:
            try:
                self.upnp.deleteportmapping(self.dport, 'UDP')
                print("[*] UPnP port mapping removed.")
            except Exception as e:
                print(f"[!] Error removing UPnP mapping: {e}")

        sys.exit(0)

    def setup_hole_punching(self):
        print("[*] UDP Hole punching start...")
        self.sock = init_sock()

        while True:
            data = self.sock.recv(1024).decode()
            if data.strip() == 'ready':
                print('[*] Checked in with server, waiting')
                break

        data = self.sock.recv(1024).decode()
        self.ip, self.sport, self.dport = data.split(' ')
        self.sport = int(self.sport)
        self.dport = int(self.dport)

        hole_punching(self.ip, self.sport, self.dport, self.sock)

    def setup_upnp(self):
        print("[*] UPnP method start...")
        self.upnp = init_upnp()
        mapping_port(self.upnp)
        check_mapping(self.upnp)
        self.sock = init_sock()

    def start(self):
        self.print_banner()

        print(f"[*] Your local IP: {get_local_ip()}")
        print(f"[*] Using connection method: {self.method.upper()}")

        try:
            if self.method == "hole":
                self.setup_hole_punching()
            elif self.method == "upnp":
                self.setup_upnp()
            elif self.method == "both":
                try:
                    self.setup_hole_punching()
                except OSError:
                    print("[-] Hole punching failed, switching to UPnP.")
                    self.setup_upnp()
            else:
                print("[-] Invalid connection method. Exiting...")
                sys.exit(1)
        except Exception as e:
            print(f"[-] Unexpected error during setup: {e}")
            sys.exit(1)

        # Start listener and sender
        threading.Thread(target=listener, args=(self.username, self.sock), daemon=True).start()
        sender(self.ip, self.sport, self.sock, self.username)


if __name__ == "__main__":
    args = arguments()
    agent = Agent(args.method, args.anonymous, args.search)
    agent.start()
