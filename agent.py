
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
from utils.network import (
    get_local_ip, mapping_port, init_upnp, check_mapping,
    listener, sender, hole_punching, init_sock
)


class Agent:
    def __init__(self, method: str, anonymous: bool, search: str,  username: str, server_ip:str, server_port:int):
        self.method = method.lower()
        self.anonymous = anonymous
        self.search = search
        self.username = username
        self.server = server_ip
        self.server_port = server_port

        if self.server and self.server_port == None:
            self.server = "51.143.219.149"
            self.server_port = 55555
            self.rendezvous = (self.server,self.server_port)

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

    def format_data(self,status,username, dport, method):
        data = dict(status = status, id = username, dport = dport, method = method)
        return data

    def setup_key(self, p:int, g:int, sock):

        P = p

        ka = DiffieHellman(p=P)
        ka.default_generator
        ka.private_key = ka.gen_private_key(ka.p)
        ka_public = ka.get_public_key()
        data = dict(kp = ka_public)
        sock.sendto(json.dump(data).encode(),self.rendezvous)
        
        return ka
        #kb_public = sock.recv(4096).decode()
    
    def generate_key(self,kb_public, ka):
        ka.derive_shared_key(kb_public)
        print("Key:", hexlify(ka.get_key()))
        

    def setup_hole_punching(self, data:dict) -> str:
        print(color_text("[*] UDP Hole punching start...","yellow"))
        print(data)
        
        self.sock = init_sock()
        self.sock.sendto(json.dumps(data).encode(),self.rendezvous) #deplacer le rendez
        #setup chiffrement

        data = self.sock.recv(4096).decode()
        p,g = data.split(' ')
        ka = self.setup_key(p,g,self.sock)

        data = self.format_data('ready',self.username,self.dport,self.method)
        self.sock.sendto(json.dumps(data).encode(),self.rendezvous)
        
        while True:
            data = self.sock.recv(4096).decode()
            if data.strip() == 'ready':
                print(color_text('[*] Checked in with server, waiting',"yellow"))
                break

        data = self.sock.recv(4096).decode()
        self.ip, self.sport, self.dport, client_username, key_pub = data.split(' ')
        self.sport = int(self.sport)
        self.dport = int(self.dport)
        key = self.generate_key(key_pub,ka)
        

        print(f"Username distant: {client_username}")
        hole_punching(self.ip, self.sport, self.dport, self.sock)

        return client_username

    def setup_upnp(self, data:dict):
        print(color_text("[*] UPnP method start...","yellow"))
        self.upnp = init_upnp()
        mapping_port(self.upnp)
        check_mapping(self.upnp)
        self.sock = init_sock(data)

    def start(self):
        self.print_banner()

        print(color_text(f"[*] Your local IP: {get_local_ip()}","yellow"))
        print(color_text(f"[*] Using connection method: {self.method.upper()}","yellow"))

        try:
            if self.method == "hole":
                data = self.format_data('check',self.method)
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
