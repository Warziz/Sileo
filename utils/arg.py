
import argparse

def arguments():
    
    parser = argparse.ArgumentParser(prog='agent.py')
    parser.add_argument("--method", type=str, default="both", choices=["hole","upnp"], help="Choose your connection method. By default the program will test both method if one fail. First method (hole) is with the hole punching (pure p2p). The second method is for upnp configuration.")
    parser.add_argument("--anonymous", type=bool, default="True", help="Connect to someone with false pseudonyme")
    parser.add_argument("--search", type=str, help="Search for people you want t odiscuss with")
    #parser.add_argument("--port", type=int, help="Bind your")
    #parser.add_argument("--ip", type=str, help="")
    parser.add_argument
    # rajouter les ports et ip server
    return parser.parse_args()