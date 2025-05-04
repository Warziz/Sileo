
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
     upnp.discover() # Recherche des périphériques UPnP sur le réseau
     upnp.selectigd()

     # Vérification de l'IP publique
     external_ip = upnp.externalipaddress()
     print(f"IP Publique : {external_ip}")

     return upnp

def mapping_port(upnp, internal_port=1501, external_port=32245, protocol="TCP"):

     # Ajout d'une redirection de port
     upnp.addportmapping(external_port, protocol, upnp.lanaddr, internal_port, "Test UPnP Mapping","")
     print(f"Port {external_port} redirigé vers {upnp.lanaddr}:{internal_port}")

def check_mapping(upnp, protocol="TCP"):

     # Liste des ports actuellement mappés
     for i in range(10):
          mapping = upnp.getspecificportmapping(i, protocol)
          if mapping:
               print(f"Port {i} : {mapping}")

def delete_mapping(upnp, external_port=32245, protocol="TCP"):
     # Suppression de la redirection (facultatif)
     upnp.deleteportmapping(external_port, protocol)
     print(f"Port {external_port} fermé.")
     
def listener(host:str, port:int, username:str):
    server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    server.bind((host, port))
    server.listen(1)
    print(f"En attente de connexion sur {host}:{port}...")
    conn, addr = server.accept()
    print(f"Connexion établie avec {addr}")

    while True:
        try:
            data = conn.recv(1024)
            if not data:
                break
            message = data.decode('utf-8')
            # Efface la ligne de saisie utilisateur
            sys.stdout.write('\r' + ' ' * 80 + '\r')
            sys.stdout.write(f"AnonymeUser >> {message}\n")
            sys.stdout.write(f"{username}(you) >> ")
            sys.stdout.flush()
        except ConnectionResetError:
            break

def sender(target_host:str, target_port:int, username:str, upnp):
    client = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    client.connect((target_host, target_port))
    print(f"Connecté à {target_host}:{target_port}")

    while True:
        msg = input(f"{username}(you)>> ")
        if msg.lower() == "exit":
            delete_mapping(upnp) 
            break
        client.send(msg.encode('utf-8'))
    client.close()