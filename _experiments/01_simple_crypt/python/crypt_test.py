from Cryptodome.PublicKey import RSA
from Cryptodome.Cipher import PKCS1_OAEP

f = open('private.pem', 'r')
private_key = RSA.importKey(f.read(), passphrase='12345')
f.close()

f = open('public.pem', 'r')
public_key = RSA.importKey(f.read())
f.close()

secret_message = "Daj się poznać!".encode('utf-8')

cipher = PKCS1_OAEP.new(public_key)
crypted_text = cipher.encrypt(secret_message)

print("Encrypted:")
print(crypted_text)
print("\n")

cipher = PKCS1_OAEP.new(private_key)
decrypted_text = cipher.decrypt(crypted_text).decode('utf-8')

print("Decrypted:")
print(decrypted_text)
