import socket

Connecteur = socket.socket(socket.AF_INET,socket.SOCK_STREAM)
 
Hote = '127.0.0.1' 
Port = 80          
Connecteur.bind((Hote,Port)) 
print("Le programme est a l'ecoute d'une eventuelle discussion, vous en serez averti.") 
Connecteur.listen(1)                  
client, adresse = Connecteur.accept()
print(f"L'ordinateur {adresse} veut discuter ! J'attends son message.") 
 
# Creation du connecteur de reponse
Reponse = socket.socket(socket.AF_INET,socket.SOCK_STREAM)
Portreponse = 234
Reponse.connect((Hote,Portreponse ))
print(f"Note : je me suis connecte a {adresse} pour lui repondre")
 
while 1:
        Message = str(client.recv(255),'mac_roman')
        if not Message: 
                break  
        print(f"\nMessage : {Message}\a" + "\n\nVotre reponse :")
        msgR = bytes(input('>> '),'mac_roman')
        Reponse.send(msgR)
 
client.close()