
import socket
import sys
import miniupnpc


def get_local_ip():
    hostname = socket.gethostname()
    local_host = socket.gethostbyname(hostname)
    print(f"Your local adresse IP: {local_host}")
    return local_host

def init_upnp():
     # Initialisation de l'UPnP client
     upnp = miniupnpc.UPnP()
     upnp.discoverdelay = 200 
     upnp.discover()
     upnp.selectigd()

     # Vérification de l'IP publique
     external_ip = upnp.externalipaddress()
     print(f"IP Publique : {external_ip}")

     return upnp

def mapping_port(upnp, internal_port=1501, external_port=32245, protocol="UDP"):

     # Ajout d'une redirection de port
     upnp.addportmapping(external_port, protocol, upnp.lanaddr, internal_port, "Sileo","")
     print(f"Port {external_port} redirigé vers {upnp.lanaddr}:{internal_port}")

def check_mapping(upnp, protocol="UDP"):

     # Liste des ports actuellement mappés
     for i in range(10):
          mapping = upnp.getspecificportmapping(i, protocol)
          if mapping:
               print(f"Port {i} : {mapping}")

def delete_mapping(upnp, external_port=32245, protocol="UDP"):
     # Suppression de la redirection (facultatif)
     upnp.deleteportmapping(external_port, protocol)
     print(f"Port {external_port} fermé.")
     
def listener(sock: socket.socket, username: str):

    while True:
        try:
            data, addr = sock.recvfrom(1024)
            message = data.decode('utf-8')
            sys.stdout.write('\r' + ' ' * 80 + '\r')
            sys.stdout.write(f"{addr[0]} >> {message}\n")
            sys.stdout.write(f"{username}(you) >> ")
            sys.stdout.flush()
        except Exception as e:
            print(f"Erreur réception: {e}")
            break

def sender(sock: socket.socket, target_addr: tuple, username: str, upnp):
    
    print(f"Connexion avec {target_addr}...")
    
    """
    for _ in range(5):
        sock.sendto(b"HELLO", target_addr)
        time.sleep(0.5)
    """
    while True:
        msg = input(f"{username}(you)>> ")
        if msg.lower() == "exit":
            delete_mapping(upnp)
            break
        sock.sendto(msg.encode('utf-8'), target_addr)