import socket
import json
from Crypto.Util import number


know_port = 50002

def gen_prime(keylenght=2048):
    return number.getPrime(keylenght)

def generator(g=2):
    valid_generators = [2,3,5,7]
    if g not in valid_generators:
        raise ValueError("Invalide Generator !")
    else:
        return g

def init_sock():

    print("[*] Start listening")
    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    sock.bind(('0.0.0.0',55555))

    return sock

def parser(data: bytes, address: tuple) -> dict:
    print(f'[+] Connection from: {address}')
    
    decoded_data = json.loads(data.decode())
    
    return {
        'ip_pub': address[0],
        'sport': address[1],
        'dport': int(decoded_data.get('dport')),
        'username': decoded_data.get('id'),
        'status': decoded_data.get('status'),
        'method': decoded_data.get('method')
    }



def hole_punching_conn(client:list, sock:socket.socket):

    if len(client) == 2:
        print('[+] Got 2 clients, sending details to each')
                    
        c1_addr, c1_port, c1_username = client.pop()
        c1 = c1_addr, c1_port
        c2_addr, c2_port, c2_username = client.pop()
        c2 = c2_addr, c2_port
            
        sock.sendto(f"{c1_addr} {c1_port} {know_port} {c1_username}".encode(), c2)
        sock.sendto(f"{c2_addr} {c2_port} {know_port} {c2_username}".encode(), c1)    

def upnp_conn(client: list,sock:socket.socket):
    

    if len(client) == 2:
        print('[+] Got 2 clients, sending details to each')
    
        c1_addr, c1_port, c1_username, c1_pubkey = client.pop()
        c1 = c1_addr, c1_port
        c2_addr, c2_port, c2_username, c2_pubkey = client.pop()
        c2 = c2_addr, c2_port
                
        sock.sendto(f"{c1_addr} {c1_port} {c1_username} {c1_pubkey}".encode(), c2)
        sock.sendto(f"{c2_addr} {c2_port} {c2_username} {c2_pubkey}".encode(), c1)

def get_conn(sock: socket.socket):
    
    while True:
        client=[]
        
        while True:
            data,address = sock.recvfrom(1024)
            
            info = parser(data, address)
            print(info)
            if info['method'] == "hole":
                if info['status'] == "check":
                    sock.sendto(f'{gen_prime()} {generator()}'.encode(), address)
                    while True:
                        kp = sock.recv(4096)
                        data = json.load(kp.decode())
                        key_recv = data.get('kp')
                        break
                
                if info['status'] == "ready":
                    sock.sendto(b'ready',address)

                share = (info['ip_pub'], info['sport'], info['username'],key_recv)
                client.append(share)
                hole_punching_conn(client,sock)

            elif info['method'] == "upnp":
                if info['status'] == "ready":
                    sock.sendto(f'ready {gen_prime()} {generator()}'.encode(), address)
                share = (info['ip_pub'],info['dport'],info['username'])
                client.append(share)
                upnp_conn(client,sock)
            else:
                print("[-] Invalide connexion method")

if __name__ == "__main__":

    sock = init_sock()
    get_conn(sock)
