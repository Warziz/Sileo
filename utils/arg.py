import argparse


def arguments():
    parser = argparse.ArgumentParser(prog="agent.py")
    parser.add_argument(
        "--method",
        "-m",
        type=str,
        default="both",
        choices=["both", "hole", "upnp"],
        help="Choose your connection method. By default the program will test both method if one fail. First method (hole) is with the hole punching (pure p2p). The second method is for upnp configuration.",
    )
    parser.add_argument(
        "--anonymous",
        "-a",
        type=bool,
        default="True",
        help="Connect to someone with false pseudonyme",
    )
    parser.add_argument(
        "--search", "-s", type=str, help="Search for people you want t odiscuss with"
    )
    parser.add_argument(
        "--username",
        "-u",
        type=str,
        help="Enter your pseudonyme or one will be generate",
    )
    parser.add_argument(
        "--server_ip",
        "-srv",
        type=str,
        help="Choose your own RendezVous Server (be sure your contact use the same)",
    )
    parser.add_argument(
        "--server_port",
        "-srv_p",
        type=int,
        help="Bind your the port of RendezVous Server",
    )
    parser.add_argument(
        "--source_port",
        "-sp",
        type=int,
        help="Custom your source port for upnp or hole punching (50001 by default)",
    )
    parser.add_argument(
        "--destination_port",
        "-dp",
        type=int,
        help="Custome your destination port for upnp or hole punching (50002 by default)",
    )
    # rajouter les ports et ip server
    return parser.parse_args()
