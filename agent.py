
import threading
import pyfiglet
import signal
import sys
import traceback

from utils.arg import arguments
from utils.user import generate_username, color_text
from utils.network import (
    get_local_ip, mapping_port, init_upnp, check_mapping,
    listener, sender, hole_punching, init_sock
)


class Agent:
    def __init__(self, method: str, anonymous: bool, search: str,  username: str):
        self.method = method.lower()
        self.anonymous = anonymous
        self.search = search
        self.username = username

        if anonymous == True or username == "":
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
        print(color_text("\n[!] Caught termination signal, cleaning up...","red"))

        if self.sock:
            try:
                self.sock.close()
                print(color_text("[*] Socket closed.","yellow"))
            except Exception as e:
                print(color_text(f"[!] Error closing socket: {e}","red"))

        if self.upnp:
            try:
                self.upnp.deleteportmapping(self.dport, 'UDP')
                print(color_text("[*] UPnP port mapping removed.","yellow"))
            except Exception as e:
                print(color_text(f"[!] Error removing UPnP mapping: {e}","red"))

        sys.exit(0)

    def format_data(self, username, dport, method):
        data = dict(status = "ready", id = username, dport = dport, method = method)
        return data

    def setup_hole_punching(self, data:dict) -> str:
        print(color_text("[*] UDP Hole punching start...","yellow"))
        self.sock = init_sock(data)

        while True:
            data = self.sock.recv(1024).decode()
            if data.strip() == 'ready':
                print(color_text('[*] Checked in with server, waiting',"yellow"))
                break

        data = self.sock.recv(1024).decode()
        self.ip, self.sport, self.dport, client_username = data.split(' ')
        self.sport = int(self.sport)
        self.dport = int(self.dport)

        hole_punching(self.ip, self.sport, self.dport, self.sock)

        return client_username

    def setup_upnp(self, dport):
        print(color_text("[*] UPnP method start...","yellow"))
        self.upnp = init_upnp()
        mapping_port(self.upnp)
        check_mapping(self.upnp)
        self.sock = init_sock(1,dport)

    def start(self):
        self.print_banner()

        print(color_text(f"[*] Your local IP: {get_local_ip()}","yellow"))
        print(color_text(f"[*] Using connection method: {self.method.upper()}","yellow"))

        try:
            if self.method == "hole":
                data = self.format_data(self.username, self.dport, self.method)
                client_username = self.setup_hole_punching(data)
            elif self.method == "upnp":
                print("[-] UPNP not implemented !")
                #self.setup_upnp(self.dport)
            elif self.method == "both":
                try:
                    self.setup_hole_punching()
                except OSError:
                    print(color_text("[-] Hole punching failed, switching to UPnP.","red"))
                    self.setup_upnp()
            else:
                print(color_text("[-] Invalid connection method. Exiting...","red"))
                sys.exit(1)
        except Exception as e:
            print(color_text(f"[-] Unexpected error during setup: {e}","red"))
            traceback.print_exc()
            sys.exit(1)

        # Start listener and sender
        threading.Thread(target=listener, args=(self.username, self.sock, client_username), daemon=True).start()
        sender(self.ip, self.sport, self.sock, self.username, self.upnp)


if __name__ == "__main__":
    args = arguments()
    agent = Agent(args.method, args.anonymous, args.search, args.username)
    agent.start()
