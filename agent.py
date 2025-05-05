

import threading
import argparse
import pyfiglet
import socket

from utils.user import generate_username, rendezvous_sync
from utils.network import get_local_ip, mapping_port, init_upnp, check_mapping, listener, sender


def print_sileo():
    ascii_art = pyfiglet.figlet_format("Sileo", font="slant")
    print(ascii_art)


def main(my_id:str, target_id:str):
    
    print_sileo()
    username = generate_username()
    
    local_host = get_local_ip() #Récupère l'ip local
    recv_port = 1501

    #initialisation de l'upnp
    upnp = init_upnp()
    
    #Creation du PAT, par defaut -> port intern: 1501, port extern: 32245
    mapping_port(upnp)
    
    #Check du mapping
    check_mapping(upnp)
    #faire un test plus tard
    
    rendezvous_url = "http://54.36.100.6:5000"
    target_port = 32245
    peer_info = rendezvous_sync(my_id,target_port,target_id,rendezvous_url)

    if peer_info:
        target_ip=peer_info["ip"]
        
        sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        sock.bind((local_host, recv_port))
                
        threading.Thread(target=listener, args=(sock, username), daemon=True).start()
        # Envoi des messages
        sender(sock, (target_ip, target_port), username, upnp)
    
if __name__ == "__main__":
        
    parser = argparse.ArgumentParser(prog='agent.py')
    parser.add_argument("my_id", type=str, help="The username by which you can be contacted")
    parser.add_argument("target_id", type=str, help="Your contact's id")
    args = parser.parse_args()
    main(args.my_id, args.target_id)
