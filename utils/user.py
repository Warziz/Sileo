import random
import string
import requests
import time

def generate_username() -> str:
    length = 10
    username = ''.join(random.choices(string.ascii_letters + string.digits, k=length))
    return username

def color_text(text, color="red"):
    colors = {
        "red": "\033[91m",
        "green": "\033[92m",
        "yellow": "\033[93m",
        "blue": "\033[94m",
        "magenta": "\033[95m",
        "cyan": "\033[96m",
        "white": "\033[97m",
        "reset": "\033[0m"
    }
    return f"{colors.get(color, colors['reset'])}{text}{colors['reset']}"

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
