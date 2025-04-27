import socket
import threading

def listener(host, port):
    server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    server.bind((host, port))
    server.listen(1)
    print(f"En attente de connexion sur {host}:{port}...")
    conn, addr = server.accept()
    print(f"Connecté avec {addr}")

    while True:
        data = conn.recv(1024)
        if not data:
            break
        print(f"\nMessage reçu de {addr}: {data.decode('utf-8')}")
    conn.close()

def sender(target_host, target_port):
    client = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    client.connect((target_host, target_port))
    print(f"Connecté à {target_host}:{target_port}")

    while True:
        msg = input(">> ")
        if msg.lower() == "exit":
            break
        client.send(msg.encode('utf-8'))
    client.close()

if __name__ == "__main__":
        
    # Exemple : 192.168.1.2, ports 1500 (envoi), 1501 (réception)
    local_host = '172.30.160.1'
    #send_port = 1500
    recv_port = 1501

    # Lancer l'écouteur dans un thread
    threading.Thread(target=listener, args=(local_host, recv_port), daemon=True).start()

    # Envoi des messages
    target_host = '172.30.160.68'  # IP de la machine distante
    target_port = 1501         # port distant d'écoute
    sender(target_host, target_port)
