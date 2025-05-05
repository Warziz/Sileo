import socket

know_port = 50002

sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
sock.bind(('0.0.0.0',55555))

while True:
    client=[]
    
    while True:
        data,address = sock.recvfrom(128)
        
        print(f'connection from: {address}')
        
        client.append(address)
        
        sock.sendto(b'ready',address)
        if len(client) == 2:
            print('Got 2 clients, sending details to each')
            break
    c1 = client.pop()
    c1_addr, c1_port = c1
    c2 = client.pop()
    c2_addr, c2_port = c2
    
    sock.sendto(f"{c1_addr} {c1_port} {know_port}".encode(), c2)
    sock.sendto(f"{c2_addr} {c2_port} {know_port}".encode(), c1)

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