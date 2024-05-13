#include <stdint.h>

typedef void * Cipher256;
typedef void * Cipher384;
typedef void * Cipher512;

Cipher256 new_cipher_256(uint8_t*);
void nw_encrypt_256(Cipher256, uint8_t*, uint8_t*);
void nw_decrypt_256(Cipher256, uint8_t*, uint8_t*);
Cipher384 new_cipher_384(uint8_t*);
void nw_encrypt_384(Cipher384, uint8_t*, uint8_t*);
void nw_decrypt_384(Cipher256, uint8_t*, uint8_t*);
Cipher512 new_cipher_512(uint8_t*);
void nw_encrypt_512(Cipher512, uint8_t*, uint8_t*);
void nw_decrypt_512(Cipher256, uint8_t*, uint8_t*);
