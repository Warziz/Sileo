import socket
import threading
import argparse
import pyfiglet
import sys

from utils.user import generate_username

def print_sileo():
    ascii_art = pyfiglet.figlet_format("Sileo", font="slant")
    print(ascii_art)


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

def sender(target_host:str, target_port:int, username:str):
    client = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    client.connect((target_host, target_port))
    print(f"Connecté à {target_host}:{target_port}")

    while True:
        msg = input(f"{username}(you)>> ")
        if msg.lower() == "exit":
            break
        client.send(msg.encode('utf-8'))
    client.close()

def get_local_ip():
    hostname = socket.gethostname()
    local_host = socket.gethostbyname(hostname)
    print(f"Your local adresse IP: {local_host}")
    return local_host


def main(target_ip):
    
    print_sileo()
    username = generate_username()
    
    local_host = get_local_ip() #Récupère l'ip local
    #local_host = '172.30.160.1'
    recv_port = 1501

    # Lancer l'écouteur dans un thread
    threading.Thread(target=listener, args=(local_host, recv_port, username), daemon=True).start()

    # Envoi des messages
    target_host = '172.30.160.68'
    target_port = 1501         # port distant d'écoute
    sender(target_ip, target_port, username) #Se connecte à la machine distante sur le port 1501
    
if __name__ == "__main__":
        
    parser = argparse.ArgumentParser(prog='agent.py')
    parser.add_argument("target_ip", type=str, help="The IP you want to connect for send message")
    args = parser.parse_args()
    main(args.target_ip)
