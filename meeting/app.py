import socket
import json

know_port = 50002

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
        'username': decoded_data.get('username'),
        'status': decoded_data.get('status'),
        'method': decoded_data.get('method')
    }



def hole_punching_conn(client:list, sock:socket.socket):

    if len(client) == 2:
        print('[+] Got 2 clients, sending details to each')
                    
        c1 = client.pop(0,1)
        c1_addr, c1_port = c1
        c2 = client.pop(0,1)
        c2_addr, c2_port = c2
            
        sock.sendto(f"{c1_addr} {c1_port} {know_port} {client[2]}".encode(), c2)
        sock.sendto(f"{c2_addr} {c2_port} {know_port} {client[2]}".encode(), c1)    

def upnp_conn(client: list, address:str, dst_port:int, sock:socket.socket):
    while True:
        sock.sendto(b'ready',address)
        if len(client) == 2:
            print('[+] Got 2 clients, sending details to each')
            break
    
        c1 = client.pop()
        c1_addr, c1_port = c1
        c2 = client.pop()
        c2_addr, c2_port = c2
            
        sock.sendto(f"{c1_addr} {c1_port} {know_port}".encode(), c2)
        sock.sendto(f"{c2_addr} {c2_port} {know_port}".encode(), c1)

def get_conn(sock: socket.socket):
    
    while True:
        client=[]
        
        while True:
            data,address = sock.recvfrom(128)
            
            info = parser(data, address)
            
            if info['method'] == "hole":
                if info['status'] == "ready":
                    sock.sendto(b'ready',address)
                share = (info['ip_pub'], info['sport'], info['username'])
                client.append(share)
                hole_punching_conn(client,sock)

            elif info['method'] == "upnp":
                client.append(address,info['dport'])
                upnp_conn(address, info['dport'])
            else:
                print("[-] Invalide connexion method")

if __name__ == "__main__":

    sock = init_sock()
    get_conn(sock)
