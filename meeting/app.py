import socket

know_port = 50002

def init_sock():

    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    sock.bind(('0.0.0.0',55555))

    return sock

def parser(data:bytes, address: str):

    print(f'[+] Connection from: {address}')

    param = data.decode()
    ready = param['ready']
    if ready == 1:
        method = param['method']
        if method == 'upnp':
            dst_port = int(param['dst_port'])
        username = param['username']
    
    return username,method,dst_port


def hole_punching_conn(client:list,address:bytes, sock:socket.socket):

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
            
            username, method, dst_port = parser(data, address)
            
            if method == "hole":
                client.append(address)
                hole_punching_conn(client,address)
            elif method == "upnp":
                client.append(address,dst_port)
                upnp_conn(address, dst_port)
            else:
                print("[-] Invalide connexion method")

if __name__ == "__main__":

    sock = init_sock()
    get_conn(sock)

"""
from flask import Flask, request, jsonify
from flask_cors import CORS
import time

app = Flask(__name__)
CORS(app)

peers = {}

@app.route('/register', methods=['POST'])
def register():
    content = request.json
    peer_id = content.get("id")
    ip = request.remote_addr
    port = content.get("port")

    if not peer_id or not port:
        return jsonify({"error": "id and port required"}), 400

    peers[peer_id] = {"ip": ip, "port": port}
    return jsonify({"status": "registered", "ip": ip, "port": port})

@app.route('/wait_for_peer', methods=['POST'])
def wait_for_peer():
    
    #Un client appelle ce endpoint pour attendre un autre pair.
    #Il doit envoyer son propre ID, et le nom du pair qu’il veut attendre.
    
    content = request.json
    my_id = content.get("id")
    target_id = content.get("target_id")

    start_time = time.time()
    timeout = 20  # secondes

    while time.time() - start_time < timeout:
        if target_id in peers and my_id in peers:
            return jsonify({
                "peer": {
                    "ip": peers[target_id]["ip"],
                    "port": peers[target_id]["port"]
                }
            })
        time.sleep(1)

    return jsonify({"error": "timeout waiting for peer"}), 408

if __name__ == "__main__":
    app.run(host='0.0.0.0', port=5000)
"""