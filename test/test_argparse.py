import pytest
from agent import arguments


def test_default_values(monkeypatch):
    monkeypatch.setattr("sys.argv", ["agent.py"])
    args = arguments()
    assert args.method == "both"
    assert args.anonymous is True
    assert args.search is None
    assert args.username is None
    assert args.server_ip is None
    assert args.server_port is None
    assert args.source_port is None
    assert args.destination_port is None


@pytest.mark.parametrize("method", ["both", "hole", "upnp"])
def test_method_choices(monkeypatch, method):
    monkeypatch.setattr("sys.argv", ["agent.py", "--method", method])
    args = arguments()
    assert args.method == method


def test_search_argument(monkeypatch):
    monkeypatch.setattr("sys.argv", ["agent.py", "--search", "alice"])
    args = arguments()
    assert args.search == "alice"


def test_username_argument(monkeypatch):
    monkeypatch.setattr("sys.argv", ["agent.py", "-u", "bob"])
    args = arguments()
    assert args.username == "bob"


def test_server_ip_and_ports(monkeypatch):
    monkeypatch.setattr(
        "sys.argv",
        [
            "agent.py",
            "--server_ip",
            "192.168.1.1",
            "--server_port",
            "9000",
            "--source_port",
            "50010",
            "--destination_port",
            "50011",
        ],
    )
    args = arguments()
    assert args.server_ip == "192.168.1.1"
    assert args.server_port == 9000
    assert args.source_port == 50010
    assert args.destination_port == 50011


"""
def test_anonymous_false(monkeypatch):
    # ATTENTION : Avec type=bool, argparse ne gère pas comme on croit
    monkeypatch.setattr("sys.argv", ["agent.py", "--anonymous", "False"])
    args = arguments()
    # Ici, "False" est converti en True car bool("False") == True
    # Le test vérifie juste le comportement actuel, pas l'intention
    assert args.anonymous is True
"""
