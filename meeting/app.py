from flask import Flask, request, jsonify
from flask_cors import CORS

app = Flask(__name__)
CORS(app)

peers = {}

@app.route('/register', methods=['POST'])
def register():
    content = request.json
    peer_id = content.get("id")
    ip = request.remote_addr #public adresse
    port = content.get("port")

    if not peer_id or not port:
        return jsonify({"error": "id and port required"}),400
    
    peers[peer_id] = {"id":ip, "port": port}
    return jsonify({"status": "registered", "ip":ip, "port":port})

@app.route('/peer/<peer_id>', methods=['GET'])
def get_peer(peer_id):
    peer = peers.get(peer_id)
    if not peer:
        return jsonify({"error": "peer not found"}),404
    return jsonify(peer)

if __name__ == "__main__":
    app.run(host='0.0.0.0', port=5000)