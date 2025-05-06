

import threading
import argparse
import pyfiglet
import socket

from utils.user import generate_username, rendezvous_sync
from utils.network import get_local_ip, mapping_port, init_upnp, check_mapping, listener, sender, hole_punching


def print_sileo():
    ascii_art = pyfiglet.figlet_format("Sileo", font="slant")
    print(ascii_art)


def main(choice: int):
    
    print_sileo()
    username = generate_username()
    
    local_host = get_local_ip() #Récupère l'ip local
    recv_port = 50001
    target_port = 50002
    
    if choice == 1:
        print("[*] UDP Hole punching start...")
    
        rendezvous = ('54.36.100.6',55555)

        sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        sock.bind((local_host, recv_port))
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
    
        hole_punching(ip,sport,dport,local_host)
    else:

        print("[*] Upnp method start...")
        #initialisation de l'upnp
        #upnp = init_upnp()
        #Creation du PAT, par defaut -> port intern: 50001, port extern: 50002
        #mapping_port(upnp)
        #Check du mapping
        #check_mapping(upnp)
        
    #peer_info = rendezvous_sync(my_id,target_port,target_id,rendezvous_url)

    #if peer_info:
    #    target_ip=peer_info["ip"]
        
    #    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    #    sock.bind((local_host, recv_port))
                
    threading.Thread(target=listener, args=(local_host, sport,username), daemon=True).start()
        # Envoi des messages
    sender(ip, local_host, dport, sport, username)
    
if __name__ == "__main__":
        
    parser = argparse.ArgumentParser(prog='agent.py')
    parser.add_argument("choice", type=int, help="Choose your connection method, with Rendezvous-server is 1 (pure p2p) & 2 for Upnp configuration")
        
    args = parser.parse_args()
    main(args.choice)
    