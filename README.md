## RSA

Basic implementation of RSA

```input:``` user provides string input as command line argument, and creates a ```.env``` file which contains two prime numbers. 


```output``` 
- Encrypted message
- public key
- private key
- decrypted message
- length of encrypted and decrypted message

Usage

```bash
cargo run [--release] Basic RSA program

p: 46877
q: 80681
n: 3782083237
phi: 3781955680
e: 3
d: 2521303787

Public key: (3, 3782083237)
Private key: (2521303787, 3782083237)
Encrypted message: 񆌈󞴡󬸻耀򆧈򋦋񃃁耀􊱷󞴡
Decrypted message: Basic RSA program
Length of message: 17
Length of encrypted message: 45
```

Random number generation is delegated to the user