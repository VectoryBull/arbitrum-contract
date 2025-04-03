def generate_keypair(p: int, q: int):
    """
    Generate public and private keys using two prime numbers.
    """
    n = p * q
    # Euler's totient function
    phi = (p - 1) * (q - 1)
    
    # Choose public key e
    e = 65537  # Commonly used value for e
    
    # Calculate private key d
    def multiplicative_inverse(e, phi):
        def extended_gcd(a, b):
            if a == 0:
                return b, 0, 1
            gcd, x1, y1 = extended_gcd(b % a, a)
            x = y1 - (b // a) * x1
            y = x1
            return gcd, x, y
        
        _, d, _ = extended_gcd(e, phi)
        return d % phi
    
    d = multiplicative_inverse(e, phi)
    
    return ((e, n), (d, n))  # (public_key, private_key)

def encrypt(message: int, public_key: tuple) -> int:
    """
    Encrypt a message using the public key.
    """
    e, n = public_key
    cipher = pow(message, e, n)  # Efficient modular exponentiation
    return cipher

def decrypt(cipher: int, private_key: tuple) -> int:
    """
    Decrypt a cipher using the private key.
    """
    d, n = private_key
    message = pow(cipher, d, n)  # Efficient modular exponentiation
    return message

# Example usage
if __name__ == "__main__":
    # Choose two prime numbers (should be much larger in practice)
    p = 61
    q = 53
    
    # Generate keypair
    public_key, private_key = generate_keypair(p, q)
    print(public_key, private_key)
    
    # Original message (must be smaller than n = p*q)
    for message in [56]:
        
        # Encryption
        cipher = encrypt(message, public_key)
        
        # Decryption
        decrypted = decrypt(cipher, private_key)
        
        print(f"Original message: {message}")
        print(f"Encrypted message: {cipher}")
        print(f"Decrypted message: {decrypted}")
        assert message == decrypted, "Decryption failed!"