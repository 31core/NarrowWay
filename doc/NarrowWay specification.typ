#import "Function F.typ": func_img

#set page(numbering: "1")
#set math.mat(delim: "[")
#set text(size: 10pt)

#align(center, text(17pt)[NarrowWay specification])

#align(center, [31core \
#link("31core@tutanota.com")])

#align(center, [*Caution*: This algorithm is not verified and not formed as the final standard yet.

Unsuitable for production use.])

#outline()

#set heading(numbering: "1.")

= Introduction
NarrowWay is a symmetric cipher based on Substitution-Permutation Network, with fixed 256, 384 and 512 bits block size and 256, 384 and 512 bits key size.

The number of rounds for each key size is:
- 16 rounds for 256 bits
- 18 rounds for 384 bits
- 20 rounds for 512 bits

It is designed with the following goals:
- High performance
- Strong security

= Structure
Take NarrowWay-256 for for example, put 32 bytes (from $b_0$ to $b_31$) plaintext and key into a 4x8 matrix. Encryption is to be performed on this matrix. As for 384 and 512 bit blocks, the matrix shapes are 6x8 and 8x8.

#figure(
$ M = mat(b_0, b_1, ...,  b_7;
b_8, b_9, ...,  b_15;
dots.v, dots.v, dots.down, dots.v;
b_24, b_25, ..., b_31) $, caption: [4x8 matrix for NarrowWay-256])

#figure(
$ M = mat(b_0, b_1, ...,  b_7;
b_8, b_9, ...,  b_15;
dots.v, dots.v, dots.down, dots.v;
b_40, b_41, ..., b_47) $, caption: [6x8 matrix for NarrowWay-384])

#figure(
$ M = mat(b_0, b_1, ...,  b_7;
b_8, b_9, ...,  b_15;
dots.v, dots.v, dots.down, dots.v;
b_56, b_57, ..., b_63) $, caption: [8x8 matrix for NarrowWay-512])

== Function F
Function $F$ is the core encryption function in NarrowWay, it takes 8 bytes plaintext ($P_0$, $P_1$, $P_2$, $P_3$, $P_4$, $P_5$, $P_6$ and $P_7$) and 8 bytes key ($K_0$, $K_1$, $K_2$, $K_3$, $K_4$, $K_5$, $K_6$ and $K_7$) as input, and then outputs 8 bytes encrypted data ($C_0$, $C_1$, $C_2$, $C_3$, $C_4$, $C_5$, $C_6$ and $C_7$).

#figure(
[#func_img]
,caption: [Function $F$])

== $op("GF")(2^8)$
Addition and multiplication in NarrowWay are performed over $op("GF")(2^8)$ with the specific primitive polynomial $m(x)$.

The primitive polynomial is:

$ m(x) = x^8 + x^6 + x^5 + x^4 + 1 $

== S-Box
In NarrowWay, each round has its own key-dependent S-Boxes set (defined as $op("Sg")$) containing several S-Boxes and the number of S-Boxes in the set depends on the block size (4 for 256-bit, 6 for 384-bit, and 8 for 512-bit), for example $op("Sg")$ containing four S-Boxes ($S_1$ ,$S_2$, $S_3$ and $S_4$). We call $S_i$ as any S-Box in $op("Sg")$ ($i$ is row number), wich will be mentioned below.

#figure($ op("Sg") = mat(
  S_1;
  S_2;
  S_3;
  S_4;
) $, caption: [S-Boxes Matrix for certain round of NarrowWay-256])

=== Generate pre-S-Box
*Calculate multiple inverse*

Calculate every byte's multiple inverse of the range over $op("GF")(2^8)$ from 0 to 255, then do a *bits substitute* step for each byte to obtain the pre-S-Box ($S_0$):

$ S^'_0 = mat(0, 1, 2, ..., 255;
0^(-1), 1^(-1), 2^(-1), ..., 255^(-1); delim: "(") $

*Bits substitute*

Permutate bits in $S^'_0(i)$ ($b_0$, $b_1$, $b_2$, $b_3$, $b_4$, $b_5$, $b_6$ and $b_7$) using the following formula:

$ mat(1, 0, 1, 0, 1, 0, 1, 1;
1, 1, 0, 1, 0, 1, 0, 1;
1, 1, 1, 0, 1, 0, 1, 0;
0, 1, 1, 1, 0, 1, 0, 1;
1, 0, 1, 1, 1, 0, 1, 0;
0, 1, 0, 1, 1, 1, 0, 1;
1, 0, 1, 0, 1, 1, 1, 0;
0, 1, 0, 1, 0, 1, 1, 1;)
mat(b_0; b_1; b_2; b_3; b_4; b_5; b_6; b_7)
=
mat(b^'_0; b^'_1; b^'_2; b^'_3; b^'_4; b^'_5; b^'_6; b^'_7) $

In other words, the matrix operation can be expressed as:

$ b_i^' = b_i xor b_(2 + i mod 8) xor b_(4 + i mod 8) xor b_(6 + i mod 8) xor b_(7 + i mod 8) $

=== Generate key dependent S-Boxes
Each round has several S-Boxes, here we mark S-Box for row $i$ as $S_i$.

For generating $S_i$, digest a specific byte ($C_i$) in a round key ($R$) at row $i$ ($R_0$, $R_1$, $R_2$, $R_3$, $R_4$, $R_5$, $R_6$ and $R_7$):

$ C_i = max(R_0, 1) dot max(R_1, 1) dot max(R_2, 1) dot ... max(R_7, 1) $

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
An integral round for encryption consists of these following three steps, $M^'$ is defined as operated state, $b_(i,j)$ is defined as byte at column i and row j in the original state below:

+ Mix columns
+ Sub bytes
+ Apply round key
 
*Mix columns*

The *Mix columns* step operates on the columns of the state, it cyclically shifts column $C$ down $C - 1 mod 4$ rows.

The example of 256-bit state:

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

*Sub bytes*

The *Sub bytes* step substitutes each elements in the state using the round S-Box.

$ M^' = mat(S_1(b_0), S_1(b_1), .., S_1(b_7);
  S_2(b_8), S_2(b_9), .., S_2(b_15);
  dots.v, dots.v, dots.down, dots.v;
  S_4(b_24), S_4(b_25), .., S_4(b_31);
  dots.v, dots.v, dots.down, dots.v;
  S_6(b_40), S_6(b_41), .., S_6(b_47);
  dots.v, dots.v, dots.down, dots.v;
  S_8(b_56), S_8(b_57), .., S_8(b_63)) $

*Apply round key*

The *Apply round key* step applies the round key on each row of the state with function $F$.

$ M^' = mat(F(b_0, b_1, .., b_7, K_0, K_1, ..., K_7);
  F(b_8, b_9, .., b_15, K_8, K_9, ..., K_15);
  dots.v;
  F(b_24, b_25, .., b_31, K_24, K_25, ..., K_31);
  dots.v;
  F(b_40, b_41, .., b_47, K_40, K_41, ..., K_47);
  dots.v;
  F(b_56, b_57, .., b_63, K_56, K_57, ..., K_63)) $
