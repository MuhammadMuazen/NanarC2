#### THIS IS JUST A REFERENCE FILE #####
#### Use this file to learn how to hash the password and how to create it ####
import base64
import hashlib

# This is just a test for decoding and de-hashing the password that will init the connection
######

base64_bytes = base64_string.encode("ascii")
sample_string_bytes = base64.b64decode(base64_bytes)
sample_string = sample_string_bytes.decode("ascii")

print(f"Decoded string: {sample_string}")

secret = "thisismypassword"

sample_string = list(map(int, sample_string.split(",")))

sha256_hash = hashlib.sha256(secret.encode()).digest()

hash_list = list(sha256_hash)

print(sample_string)
print(hash_list)

if sample_string == hash_list:
    print("Password is correct")
else:
    print("Password is wrong")
####