import random
import string
import requests
import time

def generate_username() -> str:
    length = 10
    username = ''.join(random.choices(string.ascii_letters + string.digits, k=length))
    return username


def rendezvous_sync(my_id:str, my_port:int, target_id:str, rendezvous_url:str, timeout=30):
    # Register
    requests.post(f"{rendezvous_url}/register", json={"id": my_id, "port": my_port})
    
    # Wait for peer
    start = time.time()
    while time.time() - start < timeout:
        print(f"[{my_id}] Waiting for {target_id}...")
        try:
            resp = requests.post(f"{rendezvous_url}/wait_for_peer", json={"id": my_id, "target_id": target_id})
            if resp.status_code == 200:
                peer_info = resp.json()["peer"]
                print(f"[{my_id}] Got peer info: {peer_info}")
                return peer_info
        except requests.RequestException:
            pass
        print(f"En attente que le peer '{target_id}' soit enregistré...")
        time.sleep(1)
    raise TimeoutError("Timeout : l'autre peer ne s'est pas enregistré à temps.")
