
import socket
import sys
import miniupnpc
import json

from datetime import datetime, timezone

from .user import color_text

#------------- Init Functions -------------#

def init_sock(data:dict) -> socket:
    
    #A passer en paramètre    
    rendezvous = ('51.143.219.149',55555)
    local_port = 50001
        
    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    sock.bind(('0.0.0.0', local_port))
    
    sock.sendto(json.dumps(data).encode(),rendezvous)
    
    return sock

def get_local_ip()->str:
    hostname = socket.gethostname()
    local_host = socket.gethostbyname(hostname)
    return local_host

#------------- UPNP Functions -------------#

def init_upnp() -> miniupnpc:
     # Initialisation de l'UPnP client
     upnp = miniupnpc.UPnP()
     upnp.discoverdelay = 200 
     upnp.discover()
     upnp.selectigd()

     # Vérification de l'IP publique
     external_ip = upnp.externalipaddress()
     print(f"[*] IP Publique : {external_ip}")

     return upnp

def mapping_port(upnp, internal_port=50001, external_port=50002, protocol="UDP"):

     # Ajout d'une redirection de port
     upnp.addportmapping(external_port, protocol, upnp.lanaddr, internal_port, "Sileo","")
     print(f"[*] Port {external_port} redirigé vers {upnp.lanaddr}:{internal_port}")

def check_mapping(upnp, protocol="UDP"):

     # Liste des ports actuellement mappés
     for i in range(10):
          mapping = upnp.getspecificportmapping(i, protocol)
          if mapping:
               print(f"[*] Port {i} : {mapping}")

def delete_mapping(upnp, external_port=50002, protocol="UDP"):
     # Suppression de la redirection (facultatif)
     upnp.deleteportmapping(external_port, protocol)
     print(f"[*] Port {external_port} fermé.")


#------------- Hole punching Functions -------------#

def hole_punching(ip, sport:int, dport:int, sock: socket.socket):
    print(color_text("\n[+] Got peer","green"))
    print(color_text(f"[*] ip: {ip}","yellow"))
    print(color_text(f"[*] source port: {sport}","yellow"))
    print(color_text(f"[*] destiantion port: {dport}","yellow"))

    print(color_text("[!] Punching Hole","magenta"))
    sock.sendto(b'0',(ip,dport))
    
    print(color_text("[+] Ready to exchange !","green")) 
     
def listener(username: str, sock: socket.socket):

    utc_now = datetime.now(timezone.utc)
    time_str = utc_now.strftime("%Y%m%d-%H%M")

    while True:
        try:
            data= sock.recv(1024)
            message = data.decode('utf-8')
            sys.stdout.write('\r' + ' ' * 80 + '\r')
            sys.stdout.write(color_text(f"[{time_str}] - AnonymeUser > {message}\n", "cyan"))
            sys.stdout.write(color_text(f"[{time_str}] - {username}(you) > ","green"))
            sys.stdout.flush()
        except Exception as e:
            print(color_text(f"Erreur réception: {e}","red"))
            break

def sender(target_addr:str, sport:int, sock:socket.socket, username: str, upnp):
    
    utc_now = datetime.now(timezone.utc)
    time_str = utc_now.strftime("%Y%m%d-%H%M")
    
    print(color_text(f"Connexion avec {target_addr}...","yellow"))

    while True:
        msg = input(color_text(f"[{time_str}] - {username}(you) > ","green"))
        if msg.lower() == "exit":
            delete_mapping(upnp)
            break
        sock.sendto(msg.encode('utf-8'), (target_addr,sport))