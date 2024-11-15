#include <node_api.h>
#include "../include/NarrowWay.h"

napi_status insert_method(napi_env env, napi_value obj, const char *name, napi_callback cb) {
    napi_status status;
    napi_value func;
    status = napi_create_function(env, name, NAPI_AUTO_LENGTH, cb, nullptr, &func);
    if (status != napi_ok) {
        return status;
    }

    status = napi_set_named_property(env, obj, name, func);
    if (status != napi_ok) {
        return status;
    }

    return napi_ok;
}

napi_value Cipher256_encrypt(napi_env env, napi_callback_info info) {
    size_t argc = 1;
    napi_value args;
    napi_value obj_cipher_256;
    if (napi_get_cb_info(env, info, &argc, &args, &obj_cipher_256, nullptr) != napi_ok) {
        return nullptr;
    }

    napi_value ptr;
    if (napi_get_named_property(env, obj_cipher_256, "ptr", &ptr) != napi_ok) {
        return nullptr;
    }

    Cipher256 cipher;
    if (napi_get_value_external(env, ptr, (void **) &cipher) != napi_ok) {
        return nullptr;
    }

    /* convert js Array into C array */
    uint8_t block[32] = { 0 };
    for (int i = 0; i < 32; i++) {
        napi_value byte;
        napi_get_element(env, args, i, &byte);
        uint32_t b;
        napi_get_value_uint32(env, byte, &b);
        block[i] = b;
    }

    uint8_t output[32] = { 0 };
    nw_encrypt_256(cipher, block, output);

    napi_value result;
    if (napi_create_array_with_length(env, 32, &result) != napi_ok) {
        return nullptr;
    }

    /* convert into C array into js Array */
    for (int i = 0; i < 32; i++) {
        napi_value byte;
        if (napi_create_uint32(env, output[i], &byte) != napi_ok) {
            return nullptr;
        }
        napi_set_element(env, result, i, byte);
    }

    return result;
}

napi_value Cipher256_decrypt(napi_env env, napi_callback_info info) {
    size_t argc = 1;
    napi_value args;
    napi_value obj_cipher_256;
    if (napi_get_cb_info(env, info, &argc, &args, &obj_cipher_256, nullptr) != napi_ok) {
        return nullptr;
    }

    napi_value ptr;
    if (napi_get_named_property(env, obj_cipher_256, "ptr", &ptr) != napi_ok) {
        return nullptr;
    }

    Cipher256 cipher;
    if (napi_get_value_external(env, ptr, (void **) &cipher) != napi_ok) {
        return nullptr;
    }

    /* convert js Array into C array */
    uint8_t block[32] = { 0 };
    for (int i = 0; i < 32; i++) {
        napi_value byte;
        napi_get_element(env, args, i, &byte);
        uint32_t b;
        napi_get_value_uint32(env, byte, &b);
        block[i] = b;
    }

    uint8_t output[32] = { 0 };
    nw_decrypt_256(cipher, block, output);

    napi_value result;
    if (napi_create_array_with_length(env, 32, &result) != napi_ok) {
        return nullptr;
    }

    /* convert into C array into js Array */
    for (int i = 0; i < 32; i++) {
        napi_value byte;
        if (napi_create_uint32(env, output[i], &byte) != napi_ok) {
            return nullptr;
        }
        napi_set_element(env, result, i, byte);
    }

    return result;
}

napi_value newCipher256(napi_env env, napi_callback_info info) {
    size_t argc = 1;
    napi_value args;
    if (napi_get_cb_info(env, info, &argc, &args, nullptr, nullptr) != napi_ok) {
        return nullptr;
    }

    /* convert js Array into C array */
    uint8_t key[32] = { 0 };
    for (int i = 0; i < 32; i++) {
        napi_value byte;
        napi_get_element(env, args, i, &byte);
        uint32_t b;
        napi_get_value_uint32(env, byte, &b);
        key[i] = b;
    }

    napi_value obj_cipher_256;
    if (napi_create_object(env, &obj_cipher_256) != napi_ok) {
        return nullptr;
    }

    napi_value ptr;
    if (napi_create_external(env, new_cipher_256(key),
        nullptr, nullptr, &ptr) != napi_ok) {
        return nullptr;
    }

    if (napi_set_named_property(env, obj_cipher_256, "ptr", ptr) != napi_ok) {
        return nullptr;
    }

    if (insert_method(env, obj_cipher_256, "encrypt", Cipher256_encrypt) != napi_ok) {
        return nullptr;
    }

    if (insert_method(env, obj_cipher_256, "decrypt", Cipher256_decrypt) != napi_ok) {
        return nullptr;
    }

    return obj_cipher_256;
}

napi_value Cipher384_encrypt(napi_env env, napi_callback_info info) {
    size_t argc = 1;
    napi_value args;
    napi_value obj_cipher_384;
    if (napi_get_cb_info(env, info, &argc, &args, &obj_cipher_384, nullptr) != napi_ok) {
        return nullptr;
    }

    napi_value ptr;
    if (napi_get_named_property(env, obj_cipher_384, "ptr", &ptr) != napi_ok) {
        return nullptr;
    }

    Cipher384 cipher;
    if (napi_get_value_external(env, ptr, (void **) &cipher) != napi_ok) {
        return nullptr;
    }

    /* convert js Array into C array */
    uint8_t block[48] = { 0 };
    for (int i = 0; i < 48; i++) {
        napi_value byte;
        napi_get_element(env, args, i, &byte);
        uint32_t b;
        napi_get_value_uint32(env, byte, &b);
        block[i] = b;
    }

    uint8_t output[48] = { 0 };
    nw_encrypt_384(cipher, block, output);

    napi_value result;
    if (napi_create_array_with_length(env, 48, &result) != napi_ok) {
        return nullptr;
    }

    /* convert into C array into js Array */
    for (int i = 0; i < 48; i++) {
        napi_value byte;
        if (napi_create_uint32(env, output[i], &byte) != napi_ok) {
            return nullptr;
        }
        napi_set_element(env, result, i, byte);
    }

    return result;
}

napi_value Cipher384_decrypt(napi_env env, napi_callback_info info) {
    size_t argc = 1;
    napi_value args;
    napi_value obj_cipher_384;
    if (napi_get_cb_info(env, info, &argc, &args, &obj_cipher_384, nullptr) != napi_ok) {
        return nullptr;
    }

    napi_value ptr;
    if (napi_get_named_property(env, obj_cipher_384, "ptr", &ptr) != napi_ok) {
        return nullptr;
    }

    Cipher384 cipher;
    if (napi_get_value_external(env, ptr, (void **) &cipher) != napi_ok) {
        return nullptr;
    }

    /* convert js Array into C array */
    uint8_t block[48] = { 0 };
    for (int i = 0; i < 48; i++) {
        napi_value byte;
        napi_get_element(env, args, i, &byte);
        uint32_t b;
        napi_get_value_uint32(env, byte, &b);
        block[i] = b;
    }

    uint8_t output[48] = { 0 };
    nw_decrypt_384(cipher, block, output);

    napi_value result;
    if (napi_create_array_with_length(env, 48, &result) != napi_ok) {
        return nullptr;
    }

    /* convert into C array into js Array */
    for (int i = 0; i < 48; i++) {
        napi_value byte;
        if (napi_create_uint32(env, output[i], &byte) != napi_ok) {
            return nullptr;
        }
        napi_set_element(env, result, i, byte);
    }

    return result;
}

napi_value newCipher384(napi_env env, napi_callback_info info) {
    size_t argc = 1;
    napi_value args;
    if (napi_get_cb_info(env, info, &argc, &args, nullptr, nullptr) != napi_ok) {
        return nullptr;
    }

    /* convert js Array into C array */
    uint8_t key[48] = { 0 };
    for (int i = 0; i < 48; i++) {
        napi_value byte;
        napi_get_element(env, args, i, &byte);
        uint32_t b;
        napi_get_value_uint32(env, byte, &b);
        key[i] = b;
    }

    napi_value obj_cipher_384;
    if (napi_create_object(env, &obj_cipher_384) != napi_ok) {
        return nullptr;
    }

    napi_value ptr;
    if (napi_create_external(env, new_cipher_384(key),
        nullptr, nullptr, &ptr) != napi_ok) {
        return nullptr;
    }

    if (napi_set_named_property(env, obj_cipher_384, "ptr", ptr) != napi_ok) {
        return nullptr;
    }

    if (insert_method(env, obj_cipher_384, "encrypt", Cipher384_encrypt) != napi_ok) {
        return nullptr;
    }

    if (insert_method(env, obj_cipher_384, "decrypt", Cipher384_decrypt) != napi_ok) {
        return nullptr;
    }

    return obj_cipher_384;
}

napi_value Cipher512_encrypt(napi_env env, napi_callback_info info) {
    size_t argc = 1;
    napi_value args;
    napi_value obj_cipher_512;
    if (napi_get_cb_info(env, info, &argc, &args, &obj_cipher_512, nullptr) != napi_ok) {
        return nullptr;
    }

    napi_value ptr;
    if (napi_get_named_property(env, obj_cipher_512, "ptr", &ptr) != napi_ok) {
        return nullptr;
    }

    Cipher512 cipher;
    if (napi_get_value_external(env, ptr, (void **) &cipher) != napi_ok) {
        return nullptr;
    }

    /* convert js Array into C array */
    uint8_t block[64] = { 0 };
    for (int i = 0; i < 64; i++) {
        napi_value byte;
        napi_get_element(env, args, i, &byte);
        uint32_t b;
        napi_get_value_uint32(env, byte, &b);
        block[i] = b;
    }

    uint8_t output[64] = { 0 };
    nw_encrypt_512(cipher, block, output);

    napi_value result;
    if (napi_create_array_with_length(env, 64, &result) != napi_ok) {
        return nullptr;
    }

    /* convert into C array into js Array */
    for (int i = 0; i < 64; i++) {
        napi_value byte;
        if (napi_create_uint32(env, output[i], &byte) != napi_ok) {
            return nullptr;
        }
        napi_set_element(env, result, i, byte);
    }

    return result;
}

napi_value Cipher512_decrypt(napi_env env, napi_callback_info info) {
    size_t argc = 1;
    napi_value args;
    napi_value obj_cipher_512;
    if (napi_get_cb_info(env, info, &argc, &args, &obj_cipher_512, nullptr) != napi_ok) {
        return nullptr;
    }

    napi_value ptr;
    if (napi_get_named_property(env, obj_cipher_512, "ptr", &ptr) != napi_ok) {
        return nullptr;
    }

    Cipher512 cipher;
    if (napi_get_value_external(env, ptr, (void **) &cipher) != napi_ok) {
        return nullptr;
    }

    /* convert js Array into C array */
    uint8_t block[64] = { 0 };
    for (int i = 0; i < 64; i++) {
        napi_value byte;
        napi_get_element(env, args, i, &byte);
        uint32_t b;
        napi_get_value_uint32(env, byte, &b);
        block[i] = b;
    }

    uint8_t output[64] = { 0 };
    nw_decrypt_512(cipher, block, output);

    napi_value result;
    if (napi_create_array_with_length(env, 64, &result) != napi_ok) {
        return nullptr;
    }

    /* convert into C array into js Array */
    for (int i = 0; i < 64; i++) {
        napi_value byte;
        if (napi_create_uint32(env, output[i], &byte) != napi_ok) {
            return nullptr;
        }
        napi_set_element(env, result, i, byte);
    }

    return result;
}

napi_value newCipher512(napi_env env, napi_callback_info info) {
    size_t argc = 1;
    napi_value args;
    if (napi_get_cb_info(env, info, &argc, &args, nullptr, nullptr) != napi_ok) {
        return nullptr;
    }

    /* convert js Array into C array */
    uint8_t key[64] = { 0 };
    for (int i = 0; i < 64; i++) {
        napi_value byte;
        napi_get_element(env, args, i, &byte);
        uint32_t b;
        napi_get_value_uint32(env, byte, &b);
        key[i] = b;
    }

    napi_value obj_cipher_512;
    if (napi_create_object(env, &obj_cipher_512) != napi_ok) {
        return nullptr;
    }

    napi_value ptr;
    if (napi_create_external(env, new_cipher_512(key),
        nullptr, nullptr, &ptr) != napi_ok) {
        return nullptr;
    }

    if (napi_set_named_property(env, obj_cipher_512, "ptr", ptr) != napi_ok) {
        return nullptr;
    }

    if (insert_method(env, obj_cipher_512, "encrypt", Cipher512_encrypt) != napi_ok) {
        return nullptr;
    }

    if (insert_method(env, obj_cipher_512, "decrypt", Cipher512_decrypt) != napi_ok) {
        return nullptr;
    }

    return obj_cipher_512;
}

napi_value init(napi_env env, napi_value exports) {
    if (insert_method(env, exports, "newCipher256", newCipher256) != napi_ok) {
        return nullptr;
    }

    if (insert_method(env, exports, "newCipher384", newCipher384) != napi_ok) {
        return nullptr;
    }

    if (insert_method(env, exports, "newCipher512", newCipher512) != napi_ok) {
        return nullptr;
    }
    return exports;
}

NAPI_MODULE(NODE_GYP_MODULE_NAME, init)
