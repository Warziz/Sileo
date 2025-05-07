

import threading
import argparse
import pyfiglet
import socket
import sys

from utils.user import generate_username, rendezvous_sync
from utils.network import get_local_ip, mapping_port, init_upnp, check_mapping, listener, sender, hole_punching


def print_sileo():
    ascii_art = pyfiglet.figlet_format("Sileo", font="slant")
    print(ascii_art)


def main(choice: int):
    
    print_sileo()
    username = generate_username()
    
    local_host = get_local_ip()
    print(f"[*] Your local adresse IP: {local_host}")
    local_port = 50001
    
    if choice == 1:
        print("[*] UDP Hole punching start...")
    
        rendezvous = ('51.143.219.149',55555)

        sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        sock.bind(('0.0.0.0', local_port))
        sock.sendto(b'0',rendezvous)
    
        while True:
            data = sock.recv(1024).decode()
        
            if data.strip() == 'ready':
                print('[*] Checked in with server, waiting')
                break
    
        data = sock.recv(1024).decode()
        ip,sport,dport = data.split(' ')
        sport = int(sport)
        dport = int(dport)
        
    
        hole_punching(ip,sport,dport,sock)
    
    elif choice == 2:
        
        print("[*] Upnp method start...")
        #initialisation de l'upnp
        upnp = init_upnp()
        #Creation du PAT, par defaut -> port intern: 50001, port extern: 50002
        mapping_port(upnp)
        #Check du mapping
        check_mapping(upnp)

        sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        sock.bind(('0.0.0.0', local_port))
        sock.sendto(b'0',rendezvous) #change value

    else:
        print("[-] Invalide connection method ! Exit...")
        sys.exit(0)

                
    threading.Thread(target=listener, args=(username,sock), daemon=True).start()
    # Envoi des messages
    sender(ip, sport, sock, username)
    
if __name__ == "__main__":
        
    parser = argparse.ArgumentParser(prog='agent.py')
    parser.add_argument("choice", type=int, help="Choose your connection method, with Rendezvous-server is 1 (pure p2p) & 2 for Upnp configuration")
    
    args = parser.parse_args()
    main(args.choice)
    