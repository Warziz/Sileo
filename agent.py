

import threading
import argparse
import pyfiglet


from utils.user import generate_username
from utils.network import get_local_ip, mapping_port, init_upnp, check_mapping, listener, sender


def print_sileo():
    ascii_art = pyfiglet.figlet_format("Sileo", font="slant")
    print(ascii_art)


def main(target_ip):
    
    print_sileo()
    username = generate_username()
    
    local_host = get_local_ip() #Récupère l'ip local
    #local_host = '172.30.160.1'
    recv_port = 1501

    #initialisation de l'upnp
    upnp = init_upnp()
    
    #Creation du PAT, par defaut -> port intern: 1501, port extern: 32245
    mapping_port(upnp)
    
    #Check du mapping
    check_mapping(upnp)
    #faire un test plus tard
    
    # Lancer l'écouteur dans un thread
    threading.Thread(target=listener, args=(local_host, recv_port, username), daemon=True).start()

    # Envoi des messages
    target_host = '172.30.160.68'
    target_port = 32245         # port distant d'écoute
    sender(target_ip, target_port, username,upnp) #Se connecte à la machine distante sur le port 1501
    
if __name__ == "__main__":
        
    parser = argparse.ArgumentParser(prog='agent.py')
    parser.add_argument("target_ip", type=str, help="The IP you want to connect for send message")
    args = parser.parse_args()
    main(args.target_ip)
