#import "Function F.typ": func_img

#set page(numbering: "1")
#set par(justify: true)
#set math.mat(delim: "[")

#align(center, text(17pt)[*NarrowWay specification*])

#align(center)[
31core #link("mailto:31core@tutanota.com")

*Caution*: This algorithm is not verified and not formed as the final standard yet. \
Unsuitable for production use.]

#outline()

#set heading(numbering: "1.")

= Introduction
NarrowWay is a symmetric block cipher based on Substitution-Permutation Network, with fixed 256, 384 and 512 bits block size and its corresponding 256, 384 and 512 bits key size.

It is designed with the following goals:
- High performance
- Strong security
- Universal application

= Structure
All encryption/decryption steps are operating on a certain matrix with unsigned 8-bits elements. Take NarrowWay-256 for example, it put 32 bytes (from $b_0$ to $b_31$) plaintext/ciphertext and key into a $4 times 8$ matrix. As for 384 and 512 bits blocks, the matrix shapes are $6 times 8$ and $8 times 8$.

#figure(
$ M = mat(b_0, b_1, ...,  b_7;
b_8, b_9, ...,  b_15;
dots.v, dots.v, dots.down, dots.v;
b_24, b_25, ..., b_31) $, caption: [$4 times 8$ matrix for NarrowWay-256])

#figure(
$ M = mat(b_0, b_1, ...,  b_7;
b_8, b_9, ...,  b_15;
dots.v, dots.v, dots.down, dots.v;
b_40, b_41, ..., b_47) $, caption: [$6 times 8$  matrix for NarrowWay-384])

#figure(
$ M = mat(b_0, b_1, ...,  b_7;
b_8, b_9, ...,  b_15;
dots.v, dots.v, dots.down, dots.v;
b_56, b_57, ..., b_63) $, caption: [$8 times 8$  matrix for NarrowWay-512])

== Function F
Function $F$ is the core encryption function in NarrowWay, it takes 8 bytes of plaintext ($P_0$, $P_1$, $P_2$, $P_3$, $P_4$, $P_5$, $P_6$ and $P_7$) and 8 bytes of round key ($K_0$, $K_1$, $K_2$, $K_3$, $K_4$, $K_5$, $K_6$ and $K_7$) as input, and then outputs 8 bytes of encrypted data ($C_0$, $C_1$, $C_2$, $C_3$, $C_4$, $C_5$, $C_6$ and $C_7$).

The flow chart of Function $F$ is as follow:

#figure(
  [#func_img],
  caption: [Function $F$]
)

== $op("GF")(2^8)$
Addition and multiplication in NarrowWay are performed over $op("GF")(2^8)$ with the specific primitive polynomial $m(x)$.

The primitive polynomial is:

$ m(x) = x^8 + x^6 + x^5 + x^4 + 1 $

== S-Box
In NarrowWay, each round has its own round-key-dependent S-Boxes matrix (defined as $M_s$) containing several S-Boxes and the shape of S-Boxes matrix the same as the block size ($4 times 8$ for 256-bit, $6 times 8$ for 384-bit, and $8 times 8$ for 512-bit), for example $M_s$ containing four S-Boxes ($S_1$ ,$S_2$, $S_3$ and $S_4$). We call $S_i$ as any S-Box in $M_s$ ($i$ is row number), wich will be mentioned below.

#figure($ M_s = mat(
  S_1, S_1, ..., S_1;
  S_2, S_2, ..., S_2;
  S_3, S_3, ..., S_3;
  S_4, S_4, ..., S_4;
  dots.v, dots.v, dots.down, dots.v;
  S_6, S_6, ..., S_6;
  dots.v, dots.v, dots.down, dots.v;
  S_8, S_8, ..., S_8;
) $, caption: [S-Boxes Matrix for certain round])

=== Generate pre-S-Box
*Calculate multiple inverse*

Calculate every byte's multiple inverse of the range over $op("GF")(2^8)$ from 0 to 255, then do a *bits substitute* step for each byte to obtain the pre-S-Box (defined as $S_0$):

$ S^'_0 = mat(0, 1, 2, ..., 255;
0^(-1), 1^(-1), 2^(-1), ..., 255^(-1); delim: "(") $

*Bits substitute*

Permutate bits in $S^'_0(i)$ (bits in $S^'_0(i)$ from low to high are defined as $b_0$, $b_1$, $b_2$, $b_3$, $b_4$, $b_5$, $b_6$ and $b_7$) using the following formula:

$ mat(b^'_0; b^'_1; b^'_2; b^'_3; b^'_4; b^'_5; b^'_6; b^'_7)
=
mat(1, 0, 1, 0, 1, 0, 1, 1;
1, 1, 0, 1, 0, 1, 0, 1;
1, 1, 1, 0, 1, 0, 1, 0;
0, 1, 1, 1, 0, 1, 0, 1;
1, 0, 1, 1, 1, 0, 1, 0;
0, 1, 0, 1, 1, 1, 0, 1;
1, 0, 1, 0, 1, 1, 1, 0;
0, 1, 0, 1, 0, 1, 1, 1;)
mat(b_0; b_1; b_2; b_3; b_4; b_5; b_6; b_7) $

In other words, the matrix operation can be expressed as:

$ b_i^' = b_i xor b_(2 + i mod 8) xor b_(4 + i mod 8) xor b_(6 + i mod 8) xor b_(7 + i mod 8) $

=== Generate key dependent S-Boxes
Each round has several S-Boxes, here we mark S-Box for row $i$ as $S_i$.

For generating $S_i$, digest a specific byte (defined as $C_i$) in a round key ($R$) at row $i$ ($R_1$, $R_2$, $R_3$, $R_4$, $R_5$, $R_6$, $R_7$ and $R_8$):

$ C_i = max(R_1, 1) times max(R_2, 1) times max(R_3, 1) times ... max(R_8, 1) $

Finally apply $C_i$ into $S_0$ to obtain $S_i$:
$ S_i (n) = S_0(n) xor C_i (n = 0, 1, 2, ..., 254, 255) $

== Round key expansion
Each round of NarrowWay uses a unique round key derived from the primary key through a key expansion process.

Before generate round keys, we define a round constant ($"RC"$) changed by round count over $op("GF")(2^8)$.

$ op("RC")[i] = 2^(i + 2) $

Generate round keys:

$ cases( R_0 = op("RC")[r] xor (K_0 <<< 4)^(-1) (r = 0, 1, 2, ...),
  R_i = R_(i - 1) xor (K_i <<< 4)^(-1) (i = 1, 2, ...)
) $

Where $K$ is the previous round key ($K$ is the primary key when $r$ is equal to 0), and $R$ is the current round key

== Round
Each key size has different count of rounds, higher key size has higner count, and the number of rounds for each key size is:
- 16 rounds for 256 bits
- 18 rounds for 384 bits
- 20 rounds for 512 bits

An integral round for encryption consists of following three steps, $M^'$ is defined as the state after certain operate, $b_(i,j)$ is defined as byte at column $i$ and row $j$ in the original state below:

+ ShiftColumns
+ SubstituteBytes
+ ApplyRoundKey

*ShiftColumns*

The *ShiftColumns* step operates on the columns of the matrix, it cyclically shifts column $C$ ($C$ ranges from 1 to 8) down $C - 1 mod 4$ rows.

The example of *ShiftColumns* on a $4 times 8$ matrix:

$
mat(b_(1,1), b_(1,2), b_(1,3), ..., b_(1,8);
  b_(2,1), b_(2,2), b_(2,3), ..., b_(2,8);
  b_(3,1), b_(3,2), b_(3,3), ..., b_(3,8);
  b_(4,1), b_(4,2), b_(4,3), ..., b_(4,8);
)
->
mat(b_(1,1), b_(4,2), b_(3,3), ..., b_(2,8);
  b_(2,1), b_(1,2), b_(4,3), ..., b_(3,8);
  b_(3,1), b_(2,2), b_(1,3), ..., b_(4,8);
  b_(4,1), b_(3,2), b_(2,3), ..., b_(1,8);
)
$

*SubstituteBytes*

The *SubstituteBytes* step substitutes each elements in the matrix using the round S-Box.

$ M^' = mat(S_1(b_0), S_1(b_1), .., S_1(b_7);
  S_2(b_8), S_2(b_9), .., S_2(b_15);
  dots.v, dots.v, dots.down, dots.v;
  S_4(b_24), S_4(b_25), .., S_4(b_31);
  dots.v, dots.v, dots.down, dots.v;
  S_6(b_40), S_6(b_41), .., S_6(b_47);
  dots.v, dots.v, dots.down, dots.v;
  S_8(b_56), S_8(b_57), .., S_8(b_63)) $

*ApplyRoundKey*

The *ApplyRoundKey* step applies the round key on each row of the matrix with function $F$.

$ M^' = mat(F(b_0, b_1, .., b_7, K_0, K_1, ..., K_7);
  F(b_8, b_9, .., b_15, K_8, K_9, ..., K_15);
  dots.v;
  F(b_24, b_25, .., b_31, K_24, K_25, ..., K_31);
  dots.v;
  F(b_40, b_41, .., b_47, K_40, K_41, ..., K_47);
  dots.v;
  F(b_56, b_57, .., b_63, K_56, K_57, ..., K_63)) $
