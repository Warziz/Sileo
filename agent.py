

import threading
import pyfiglet
import sys

from utils.arg import arguments
from utils.user import generate_username, rendezvous_sync
from utils.network import get_local_ip, mapping_port, init_upnp, check_mapping, listener, sender, hole_punching, init_sock


def print_sileo():
    ascii_art = pyfiglet.figlet_format("Sileo", font="slant")
    print(ascii_art)


def main(method: str, anonymous: bool, search: str):
    
    print_sileo()
    username = generate_username()
    
    local_host = get_local_ip()
    print(f"[*] Your local adresse IP: {local_host}")

    sock = init_sock()
    
    method = method.lower()
    
    if method == "both":
        try:
            print("[*] UDP Hole punching start...")
    
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
        
        except OSError:
            
            print("[-] Fail to connect with UDP Hole Punching ! ")
            print("[*] Switch to Upnp")
            print("[*] Upnp method start...")    
            
            #initialisation de l'upnp
            upnp = init_upnp()
            #Creation du PAT, par defaut -> port intern: 50001, port extern: 50002
            mapping_port(upnp)
            #Check du mapping
            check_mapping(upnp)

            sock = init_sock()
            
        except Exception as e:
            print(f"[-] Something goes wrong: {e}")
            sys.exit(1)
    elif method == "hole":
        try:
            print("[*] UDP Hole punching start...")
    
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
        except Exception as e:
            print(f"[-] Something goes wrong: {e}")
            sys.exit(1)
                    
    elif method == "upnp":
        try:
            print("[*] Upnp method start...")
            #initialisation de l'upnp
            upnp = init_upnp()
            #Creation du PAT, par defaut -> port intern: 50001, port extern: 50002
            mapping_port(upnp)
            #Check du mapping
            check_mapping(upnp)

            sock = init_sock()
        except Exception as e:
            print(f"[-] Something goes wrong: {e}")
            sys.exit(1)            
    else:
        print("[-] Invalide connection method ! Exit...")
        sys.exit(0)

                
    threading.Thread(target=listener, args=(username,sock), daemon=True).start()
    # Envoi des messages
    sender(ip, sport, sock, username)
    
if __name__ == "__main__":
        
    args = arguments()
    main(args.method,args.anonymous,args.search)
    