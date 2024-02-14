#import "@preview/fletcher:0.3.0" as fletcher: node, edge

#set page(numbering: "1")
#set math.mat(delim: "[")

#align(center, text(17pt)[NarrowWay specification])

#align(center, [31core \
#link("31core@tutanota.com")])

#align(center, [*Caution*: This algorithm is not verified and not formed as the final standard yet.

Use at your own risk.])

#outline()

#set heading(numbering: "1.")

= Introduction
NarrowWay is a symmetric cipher based on Substitution-Permutation Network, with fixed 128, 192 and 256 bits block size and 128, 192 and 256 bits key size.

The number of rounds for each key size is:
- 16 rounds for 128 bits
- 18 rounds for 192 bits
- 20 rounds for 256 bits

It is designed with the following goals:
- High performance
- Strong security

= Structure
NarrowWay-128 puts 16 bytes data and key into a 4x4 matrix and perform calculations on it. For 192 and 256 bit blocks, the matrix shapes are 6x4 and 8x4.

#figure(
$ M = mat(b_0, b_1, b_2, b_3;
b_4, b_5, b_6, b_7;
b_8, b_9, b_10, b_11;
b_12, b_13, b_14, b_15) $, caption: [4x4 matrix for NarrowWay-128])

#figure(
$ M = mat(b_0, b_1, b_2, b_3;
b_4, b_5, b_6, b_7;
b_8, b_9, b_10, b_11;
b_12, b_13, b_14, b_15;
b_16, b_17, b_18, b_19;
b_20, b_21, b_22, b_23) $, caption: [6x4 matrix for NarrowWay-192])

#figure(
$ M = mat(b_0, b_1, b_2, b_3;
b_4, b_5, b_6, b_7;
b_8, b_9, b_10, b_11;
b_12, b_13, b_14, b_15;
b_16, b_17, b_18, b_19;
b_20, b_21, b_22, b_23;
b_24, b_25, b_26, b_27;
b_28, b_29, b_30, b_31) $, caption: [8x4 matrix for NarrowWay-256])

== Function F
Function $F$ is the core encryption function in NarrowWay, it takes 4 bytes plaintext ($P_0$, $P_1$, $P_2$ and $P_3$) and 4 bytes key ($K_0$, $K_1$, $K_2$ and $K_3$) as input, and then outputs 4 bytes encrypted data ($C_0$, $C_1$, $C_2$ and $C_3$).

#figure(
[#fletcher.diagram(
  node((0, 0), $P_0$),
  node((1, 0), $P_1$),
  node((2, 0), $P_2$),
  node((3, 0), $P_3$),

  /* line 1 */
  edge((0, 0), (0, -2), "-|>"),
  edge((2, 0), (2, -2), "-|>"),
  edge((0, -1), (1, -1), "-|>"),
  edge((1, 0), (1, -1), "-|>"),
  edge((2, -1), (1, -1), "-|>"),
  edge((3, 0), (3, -1), "-|>"),
  node((1, -1), $xor$),
  node((3, -1), $>>> 2$),

  /* line 2 */
  edge((1, -1), (1, -3), "-|>"),
  edge((3, -1), (3, -3), "-|>"),
  edge((1, -2), (1.5, -2), "-|>"),
  edge((1.5, -2), (2, -2), "-|>"),
  edge((3, -2), (2, -2), "-|>"),
  node((0, -2), $<<< 2$),
  node((1.5, -2), $>>> 4$),
  node((2, -2), $xor$),

  /* line 3 */
  node((1, -3), $>>> 2$),
  node((2.5, -3), $<<<1$),
  node((3, -3), $xor$),
  edge((2, -3), (2.5, -3), "-|>"),
  edge((2.5, -3), (3, -3), "-|>"),

  /* line 4 */
  edge((0, -2), (0, -4), "-|>"),
  edge((2, -2), (2, -4), "-|>"),
  edge((1, -4), (0, -4), "-|>"),
  node((0, -4), $xor$),
  node((2, -4), $>>> 3$),

  /* Apply key */
  edge((0, -4), (0, -5), "-|>"),
  edge((1, -3), (1, -5), "-|>"),
  edge((2, -4), (2, -5), "-|>"),
  edge((3, -3), (3, -5), "-|>"),
  node((0, -5), $xor$),
  node((1, -5), $xor$),
  node((2, -5), $xor$),
  node((3, -5), $xor$),

  node((0.5, -5), $K_0$),
  edge((0.5, -5), (0, -5), "-|>"),
  node((1.5, -5), $K_1$),
  edge((1.5, -5), (1, -5), "-|>"),
  node((2.5, -5), $K_2$),
  edge((2.5, -5), (2, -5), "-|>"),
  node((3.5, -5), $K_3$),
  edge((3.5, -5), (3, -5), "-|>"),

  /* Bit shift */
  edge((0, -5), (0, -6)),
  edge((1, -5), (1, -6)),
  edge((2, -5), (2, -6)),
  edge((3, -5), (3, -6)),

  /* Returns */
  edge((0, -6), (2, -7), "-|>"),
  node((0, -7), $C_0$),
  edge((1, -6), (3, -7), "-|>"),
  node((1, -7), $C_1$),
  edge((2, -6), (0, -7), "-|>"),
  node((2, -7), $C_2$),
  edge((3, -6), (1, -7), "-|>"),
  node((3, -7), $C_3$),
)]
,caption: [Function $F$])

*Confuse*

$ P_1 := P_0 xor P_1 xor P_2 $
$ P_0 := P_0 <<< 2 $
$ P_2 := (P_1>>> 4) xor P_2 xor P_3 $
$ P_1 := P_1 >>> 2 $
$ P_3 := P_3 >>> 2 $
$ P_3 := P_3 xor (P_2 <<< 1) $
$ P_0 := P_0 xor P_1 $
$ P_2 := P_2 >>> 3 $
$ P_3 := P_3 <<< 1 $

*Apply key*

$ P_0 := P_0 xor K_0 $
$ P_1 := P_1 xor K_1 $
$ P_2 := P_2 xor K_2 $
$ P_3 := P_3 xor K_3 $

*Output*

$ C_0 := P_2 $
$ C_1 := P_3 $
$ C_2 := P_0 $
$ C_3 := P_1 $

== $op("GF")(2^8)$
Addition and multiplication in NarrowWay are performed over $op("GF")(2^8)$.

The primitive polynomial is:

$ m(x) = x^8 + x^6 + x^5 + x^4 + 1 $

== S-Box
In NarrowWay, each round has its own key-dependent S-Box, which is generated over $op("GF")(2^8)$ by the round key dynamically.

Calculate every byte's multiple inverse of the S-Box:

$ f(x) dot f^(-1)(x) eq.triple 1 (mod m(x)) $

$ S_0 = mat(0, 1, 2, ..., 255;
0^(-1), 1^(-1), 2^(-1), ..., 255^(-1); delim: "(") $

*Bits substitute*

Permutate bits in $S_0(i)$ using the following formula:

$ S(i) = mat(1, 0, 1, 0, 1, 0, 1, 1;
1, 1, 0, 1, 0, 1, 0, 1;
1, 1, 1, 0, 1, 0, 1, 0;
0, 1, 1, 1, 0, 1, 0, 1;
1, 0, 1, 1, 1, 0, 1, 0;
0, 1, 0, 1, 1, 1, 0, 1;
1, 0, 1, 0, 1, 1, 1, 0;
0, 1, 0, 1, 0, 1, 1, 1;)
times mat(b_0; b_1; b_2; b_3; b_4; b_5; b_6; b_7)
xor mat(c_0; c_1; c_2; c_3; c_4; c_5; c_6; c_7) $

In other words, the matrix operation can be expressed as:

$ B_i = (b_7, b_6, b_5, b_4, b_3, b_2, b_1, b_0) $
$ C = (c_7, c_6, c_5, c_4, c_3, c_2, c_1, c_0) $
$ b_i^' = b_i xor b_(2 + i mod 8) xor b_(4 + i mod 8) xor b_(6 + i mod 8) xor b_(7 + i mod 8) xor c_i $

Where $B_i$ is any byte in the S-Box ($S_0(i)$), and $C$ is a key based byte in order to generate different S-Boxes for each round.

For generating $C$, we can digest a special byte in a round key ($R$) to use in generating S-Box like this:

$ C = max(R_0 xor R_1 xor R_2 xor ... R_15, 1) $

== Round key
Before generate round keys, we define a round constant ($"RC"$) changed by round count over $op("GF")(2^8)$.

$ op("RC")[i] = 2^(i + 2) $

Generate round keys:

$ R_0 = op("RC")[r] xor (K_0 <<< 4)^(-1) (r = 0, 1, 2, ...) $

$ R_i = R_(i - 1) xor (K_i <<< 4)^(-1) (i = 1, 2, ...) $

Where $K$ is the previous round key, and $R$ is the current round key. And when $R$ is the first round key, the $K$ is the primary key.

== Round
An integral round for encryption conatains these following 3 steps:

- Mix columns
- Sub bytes
- Apply round key

*Mix columns*

Shift column 1 down one row, shift column 2 down two rows, and shift column 3 down three rows.

$ 
mat(b_(1,1), b_(1,2), b_(1,3), b_(1,4);
b_(2,1), b_(2,2), b_(2,3), b_(2,4);
b_(3,1), b_(3,2), b_(3,3), b_(3,4);
b_(4,1), b_(4,2), b_(4,3), b_(4,4);)
->
mat(b_(1,1), b_(4,2), b_(3,3), b_(2,4);
b_(2,1), b_(1,2), b_(4,3), b_(3,4);
b_(3,1), b_(2,2), b_(1,3), b_(4,4);
b_(4,1), b_(3,2), b_(2,3), b_(1,4);)
 $

*Sub bytes*

Substitute bytes in the matrix using the round S-Box.

$ M_(i, j)^' = S(M_(i, j)) $

*Apply round key*

Encrypt each row with function $F$.

$ b_(i,0)^', b_(i,1)^', b_(i,2)^', b_(i,3)^' = F(b_(i,0), b_(i,1), b_(i,2), b_(i,3), K_(i,0), K_(i,1), K_(i,2), K_(i,3))
(i = 0, 1, 2, 3) $
