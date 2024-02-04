#include <stdint.h>

typedef void * Cipher128;
typedef void * Cipher192;
typedef void * Cipher256;

Cipher128 new_cipher_128(uint8_t*);
void encrypt_128(Cipher128, uint8_t*, uint8_t*);
Cipher192 new_cipher_128(uint8_t*);
void encrypt_128(Cipher192, uint8_t*, uint8_t*);
Cipher256 new_cipher_128(uint8_t*);
void encrypt_128(Cipher256, uint8_t*, uint8_t*);
