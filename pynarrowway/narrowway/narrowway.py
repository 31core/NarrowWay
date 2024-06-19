import ctypes
import platform
import os
import sys

dir = os.path.dirname(sys.modules["narrowway"].__file__)

if platform.system() == "Darwin":
    path = os.path.join(dir, "libnarrowway.dylib")
elif platform.system() == "Linux":
    path = os.path.join(dir, "libnarrowway.so")
elif platform.system() == "Windows":
    path = os.path.join(dir, "narrowway.dll")

clib = ctypes.cdll.LoadLibrary(path)

class Cipher256:
    def __init__(self, key: bytes) -> None:
        key_raw = (ctypes.c_uint8 * 32)()
        for i in range(32):
            key_raw[i] = key[i]
        self.cipher = new_cipher_256(key_raw)
    def encrypt(self, bytes: bytes) -> bytes:
        data_in = (ctypes.c_uint8 * 32)()
        data_out = (ctypes.c_uint8 * 32)()
        for i in range(32):
            data_in[i] = bytes[i]

        nw_encrypt_256(self.cipher, data_in, data_out)

        bytes = b""
        for i in range(32):
            bytes += data_out[i].to_bytes()
        return bytes
    def decrypt(self, bytes: bytes) -> bytes:
        data_in = (ctypes.c_uint8 * 32)()
        data_out = (ctypes.c_uint8 * 32)()
        for i in range(32):
            data_in[i] = bytes[i]

        nw_decrypt_256(self.cipher, data_in, data_out)

        bytes = b""
        for i in range(32):
            bytes += data_out[i].to_bytes()
        return bytes

class Cipher384:
    def __init__(self, key: bytes) -> None:
        key_raw = (ctypes.c_uint8 * 48)()
        for i in range(48):
            key_raw[i] = key[i]
        self.cipher = new_cipher_384(key_raw)
    def encrypt(self, bytes: bytes) -> bytes:
        data_in = (ctypes.c_uint8 * 48)()
        data_out = (ctypes.c_uint8 * 48)()
        for i in range(48):
            data_in[i] = bytes[i]

        nw_encrypt_384(self.cipher, data_in, data_out)

        bytes = b""
        for i in range(48):
            bytes += data_out[i].to_bytes()
        return bytes
    def decrypt(self, bytes: bytes) -> bytes:
        data_in = (ctypes.c_uint8 * 48)()
        data_out = (ctypes.c_uint8 * 48)()
        for i in range(48):
            data_in[i] = bytes[i]

        nw_decrypt_384(self.cipher, data_in, data_out)

        bytes = b""
        for i in range(48):
            bytes += data_out[i].to_bytes()
        return bytes

class Cipher512:
    def __init__(self, key) -> None:
        key_raw = (ctypes.c_uint8 * 64)()
        for i in range(64):
            key_raw[i] = key[i]
        self.cipher = new_cipher_512(key_raw)
    def encrypt(self, bytes: bytes) -> bytes:
        data_in = (ctypes.c_uint8 * 64)()
        data_out = (ctypes.c_uint8 * 64)()
        for i in range(64):
            data_in[i] = bytes[i]

        nw_encrypt_512(self.cipher, data_in, data_out)

        bytes = b""
        for i in range(64):
            bytes += data_out[i].to_bytes()
        return bytes
    def decrypt(self, bytes: bytes) -> bytes:
        data_in = (ctypes.c_uint8 * 64)()
        data_out = (ctypes.c_uint8 * 64)()
        for i in range(64):
            data_in[i] = bytes[i]

        nw_decrypt_512(self.cipher, data_in, data_out)

        bytes = b""
        for i in range(64):
            bytes += data_out[i].to_bytes()
        return bytes

new_cipher_256 = clib.new_cipher_256
new_cipher_256.restype = ctypes.POINTER(ctypes.c_void_p)

nw_encrypt_256 = clib.nw_encrypt_256
nw_decrypt_256 = clib.nw_decrypt_256

new_cipher_384 = clib.new_cipher_384
new_cipher_384.restype = ctypes.POINTER(ctypes.c_void_p)

nw_encrypt_384 = clib.nw_encrypt_384
nw_decrypt_384 = clib.nw_decrypt_384 

new_cipher_512 = clib.new_cipher_512
new_cipher_512.restype = ctypes.POINTER(ctypes.c_void_p)

nw_encrypt_512 = clib.nw_encrypt_512
nw_decrypt_512 = clib.nw_decrypt_512
