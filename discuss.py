
import socket

Discuter = socket.socket(socket.AF_INET,socket.SOCK_STREAM)
Hote = '127.0.0.1'
Port = 80
Port_de_reponse = 234
Discuter.connect((Hote,Port))   
 
Reponse = socket.socket(socket.AF_INET,socket.SOCK_STREAM)
Reponse.bind((Hote,Port_de_reponse))
Reponse.listen(1)
client, adresse = Reponse.accept()  # Creation du connecteur pour la reponse de ecoute.py
print (f"L'adresse {adresse} vous a entendu et attend votre message.") 
while 1:
        msg = bytes(input('>> '),'mac_roman')  # votre message ? Python 3 : msg = bytes(input('>> '), 'mac_roman')
        Discuter.send(msg)      # envoi.
        print("Attente de la reponse...") 
        reponseaumessage = client.recv(255) # reception de la reponse, 255 caracteres max ; Python 3 : reponseaumessage = str(client.recv(255),'mac_roman')
        if not reponseaumessage:
                break  
        print(f"\n,{adresse}:{reponseaumessage},\a\n") 
 
client.close() 