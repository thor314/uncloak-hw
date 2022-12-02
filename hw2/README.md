# 2022-11-25 Week 2 Solutions
- back:: [[course-2022-11-125 Session 2 Notes]]

# Exercises For Week 2
> From the Text: https://drive.google.com/file/d/15csrrLC72dgFdCTxQ4FQBhHtRFDbLzkU/view?usp=sharing

### Chapter 3 (p. 61)
1; How much space would be required to store a table for an entire idealized block cipher that operates on 64-bit blocks and has 80-bit keys?
- How many possible keys are there? $2^{80}$. 
- How many possible blocks are there? $2^{60}$
- There will be $2^{144}=2^{64}*2^{80}$ entries, with 64 bits per block, so $2^{150}=2^{144}*2^6$ bits total, or $2^{147}$ bytes.

5; Suppose you have a processor that can perform a single $DES$ encryption or decryption operation in $2^{-26}$  seconds. Suppose you also have a large number of plaintext-ciphertext pairs for $DES$ under a single unknown key. How many hours would it take, on average, to find that $DES$ key, using an exhaustive search approach and a single processor? How many hours would it take, with a collection of $2^{14}$ processors? 
- note: "a large number" is weird, they should have just given a number. But w/e, *ahem* I mean, I assigned this for an opportunity to practice symbolic reasoning
- Let:
    -  the number of owned plaintext-ciphertext pairs be $n$
    -  the time per (en|de)cryption be $t$ seconds
    -  the total number of possible DES keys be $N$
    -  the number of parallel processers be $d$
    -  the number of attempts to be $a$
- For any independent attempt[^1], the probability of key-discovery can be expressed: $P[\text{key discovery}]=n/N$. We make the assumption attempts approximately independent, and solve $na/N=.5$:
$$0.5 = na/N = \frac{na}{2^{56}}$$
$$a=2^{55}/n$$
and the time to obtain a collision can be expressed:
$$T := \frac{at}{d} = \frac{2^{55}2^{-26}}{n*2^{14}}$$
$$=2^{15}/n \text{ seconds}$$



6; Consider a new block cipher, *DES2*, that consists only of two rounds of the *DES* block cipher. *DES2* has the same block and key size as *DES*. For this question you should consider the *DES* $F$ function as a black box that takes two inputs, a 32-bit data segment and a 48-bit round key, and that produces a 32-bit output. Suppose you have a large number of plaintext-ciphertext pairs for *DES2* under a single, unknown key. Give an algorithm for recovering the 48-bit round key for round 1 and the 48-bit round key for round 2. Your algorithm should require fewer operations than an exhaustive search for an entire 56-bit *DES* key. Can your algorithm be converted into a distinguishable attack against *DES2*?
- Let $L$ and $R$ be indexed by round: $L_0$ is for instance, the initial value for $L$, and $L_2$ the output. Then:
    - $R_1=F(R_0,k_0) \oplus L_0$
    - $L_1=R_0$
    - $R_2=F(R_1,k_1) \oplus L_1$
    - $L_2=R_1=F(R_0,k_0) \oplus L_0$

For known ciphertexts-plaintext pairs, we can peel off XOR'd values to compute:
- $R_1\oplus L_0=F(R_0,k_0)$
- $R_2\oplus L_1=F(R_1,k_1)$
- $L_2\oplus L_0=F(R_0,k_0)=R_1\oplus L_0$

That is, for each cipher-text plaintext pair, we obtain three 

___
Observations
1; given $F: 2^{32} \times 2^{48} \rightarrow 2^{32}$ ; we would need $\mathcal{O}(2^{16})$* input-outputs of $F$ before obtaining a collision for $F$.

Denote: 
$F_i(x) = F(x,k_i)$
$i \in \{0,1\}$

2; Given {$L_0, L_2, R_0, R_1$}, we can obtain $L_1, R_1:$ $L_1 = R_0$ & $R_1 = L_2$

3; What can we do with collisions?

- Suppose $f_1(a) = f_0(b)$,

    - Then:
$R_2 \oplus L_1 = R'_1 \oplus L_0$
$L'_0 = R_2 \oplus L_1 \oplus R'_1$

- or $f_1(a) = f_1(b)$ $a \neq b$,

    - Then:
$R_2 \oplus L_1 = R'_2 \oplus L_1'$
$L'_1 = R_2 \oplus L_1 \oplus R'_2$
$L'_1 = R'_0$: we can obtaiin half of the plaintext.

- or $f_0(a) = f_0(b)$ $a \neq b$,
    - Then:
    $R_1 \oplus L_0 = R'_1 \oplus L_0'$
    $L'_0 = R_1 \oplus L_0 \oplus R'_1$

4; Can we detect collisions from *any* ciphertext?

Given: 
$L'_2 = R'_1 = f_0(R'_0) \oplus L'_0$
$R'_2 = f_1(R'_1) \oplus L'_1$

If we can build a table for $F_1$, including $f_1(R'_1)$, we can obtain:
$L'_1 = R'_2 \oplus f_1(R'_1) = R'_0$

Further, if we can build a table for $f_0$ including $f+0(R'_0)$, we can obtain:
$L'_0 = R'_1 \oplus f+0 (R'_0)$

So we want to build tables for $f_0, f_1$.
$2^{32}$ values each, total runtime $\mathcal{O}(2^{32})$ operations.

The Algorithm
1; Vary $R_0 \in [0, 2^{32}-1]$ to build a table for $f_0$. Set $L_0 = 0$
2; Find collisions of $f_0(\{R_0\})$
3; Choose values for $L_0$, such that we eliminate collisions and may compute a complete table for $f_1(R_1)$.
4; Done. For all ciphertexts $(L_2, R_2)$:
we will be able to compute $F_1(R_1)$ and $f_0(R_0)$, therefore we can compute:
$R_0 = L_1 = R_2 \oplus f_1(R_1)$ and $L_0 = R_1 \oplus f_0(R_0)$

>$L_1 = R_0$
>$L_2 = R_1$
>$R_1 = F(R_0, k_0) \oplus L_0$
    = $F_0(R_0)\oplus L_0$ 
>$R_2 = F(R_1, k_1) \oplus L_1$
= $F_1(R_1)\oplus L_1$
___


8; Familiarize yourself with a cryptographic software development package for your computer. A popular open source package is [*OpenSSL*](https://docs.rs/openssl/latest/openssl/aes/index.html).
	- Using an existing cryptographic library, decrypt the following ciphertext (in hex)
```hex 
	53 9B 33 3B 39 70 6D 14 90 28 CF E1 D9 D4 A4 07
```

with the following 256-bit key (also in hex)

```hex
	80 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
	00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 01
```

using *AES*.
- answer:
```bash
$ echo "539B333B39706D149028CFE1D9D4A407" | xxd -r -p | openssl enc -aes256 -d -K "8000000000000000000000000000000000000000000000000000000000000001" -iv 0 -nopad | xxd
```

9; Using an existing cryptography library, encrypt the following plaintext (in hex)

```hex
	29 6C 93 FD F4 99 AA EB 41 94 BA BC 2E 63 56 1D
```

with the following 256-bit key (also in hex)

```hex
	80 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
	00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 01
```
using *AES*.
- answer:
```bash
$ echo "296C93FDF499AAEB4194BABC2E63561D" | xxd -r -p | openssl enc -aes256 -e -K "8000000000000000000000000000000000000000000000000000000000000001" -iv 0 -nopad | xxd
```
10; Write a program that experimentally demonstrates the complementation property for *DES*. This program should take as input a key $K$ and a plaintext $P$ and demonstrate that the $DES$ complementation property holds for this key and plaintext. You may use an existing cryptography library for this exercise.  

### Chapter 4 (p. 107)
1; Let $P$ be a plaintext and let $\ell(P)$ be the length of $P$ in bytes. Let $b$ be the block size of the block cipher in bytes. Explain why the following is not a good padding scheme: 
- Determine the minimum number of padding bytes necessary in order to pad the plaintext to a block boundary. This is a number n which satisfies $0 ≤ n ≤ b − 1$ and $n + (P)$ is a multiple of $b$. Pad the plaintext by appending $n$ bytes, each with value $n$.
- The above is not a good padding scheme because it allows for the padding to be 0 bytes long ($n = 0$). If $n=0$, and the plaintext $P$ is exactly the same length as the block ($b$), the block cipher will not know how much padding to remove and might remove some of the message. Generally $n$ should be $>0$ so that the block cipher knows how to deal with $P$. If $P$ is exactly the same length as the $b$, then an additional block of padding can be added ($\ell(P) = \ell(b)$).

3; Suppose you, as an attacker, observe the following 32-byte ciphertext $C$ (in hex)
```hex
46 64 DC 06 97 BB FE 69 33 07 15 07 9B A6 C2 3D
2B 84 DE 4F 90 8D 7D 34 AA CE 96 8B 64 F3 DF 75
```
and the following 32-byte ciphertext $C'$ (also in hex)
```hex
51 7E CC 05 C3 BD EA 3B 33 57 0E 1B D8 97 D5 30
7B D0 91 6B 8D 82 6B 35 B7 8B BB 8D 74 E2 C7 3B.
```
Suppose you know these ciphertexts were generated using CTR mode with the same nonce. The nonce is implicit, so it is not included in the ciphertext. You also know that the plaintext $P$ corresponding to $C$ is
```hex
43 72 79 70 74 6F 67 72 61 70 68 79 20 43 72 79
70 74 6F 67 72 61 70 68 79 20 43 72 79 70 74 6F.
```
What information, if any, can you infer about the plaintext $P'$ corresponding
to $C'$?
- nonce is implicit, so it is not needed when calculating $P'$ (it isn't included in $C'$)
- Since the attacker knows $P$, $C$, & $C'$ they can find $P'$:
- $P' = P \oplus C \oplus C'$ 
- (if $C$ & $C'$ used a different nonce, this equality would not hold)

4; The ciphertext (in hex)

```hex
87 F3 48 FF 79 B8 11 AF 38 57 D6 71 8E 5F 0F 91
7C 3D 26 F7 73 77 63 5A 5E 43 E9 B5 CC 5D 05 92
6E 26 FF C5 22 0D C7 D4 05 F1 70 86 70 E6 E0 17
```
was generated with the 256-bit AES key (also in hex)
```hex
80 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 01
```
using CBC mode with a random IV. The IV is included at the beginning of the ciphertext. Decrypt this ciphertext. You may use an existing cryptography library for this exercise.
- asnwer:
```bash
$ echo "7C3D26F77377635A5E43E9B5CC5D05926E26FFC5220DC7D405F1708670E6E017" | xxd -r -p | openssl enc -aes-256-cbc -d -K "8000000000000000000000000000000000000000000000000000000000000001" -iv 87F348FF79B811AF3857D6718E5F0F91 -nopad | xxd
```
- Since The IV is included at the beginning of the ciphertext we know that "87F348FF79B811AF3857D6718E5F0F91" is the IV and can decrypt the ciphertext using the 256-bit AES key and this IV.

5; Encrypt the plaintext
```hex
62 6C 6F 63 6B 20 63 69 70 68 65 72 73 20 20 20
68 61 73 68 20 66 75 6E 63 74 69 6F 6E 73 20 78
62 6C 6F 63 6B 20 63 69 70 68 65 72 73 20 20 20
```
using AES in ECB mode and the key
```hex
80 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 01.
```
You may use an existing cryptography library for this exercise.
- answer:
```bash
echo "626C6F636B2063697068657273202020686173682066756E6374696F6E732078626C6F636B2063697068657273202020" | xxd -r -p | openssl enc -aes-256-ecb -e -K "8000000000000000000000000000000000000000000000000000000000000001" -nopad | xxd
```

6; Let $P_1$, $P_2$ be a message that is two blocks long, and let $P'_1$ be a message that is one block long. Let $C_0, C_1, C_2$ be the encryption of $P_1, P_2$ using CBC mode with a random IV and a random key, and let $C'_0, C'_1$ be the encryption of $P'_1$ using CBC mode with a random IV and the same key. Suppose an attacker knows $P_1, P_2$ and suppose the attacker intercepted and thus know $C0, C1, C2$ and $C0, C1$. Further suppose that, by random chance, $C_1 = C_2$. Show that the attacker can compute $P'_1$.
- find $C_1$


----

[^1]: these attempts aren't precisely independent, but we can model them as such for simplicity, though in reality the attacker would be slightly more powerful. The actual probability for attempt $m$ to obtain the key would be $P[E_m] = \frac{n}{N-m}$, and the probability that the key is discovered on or before attempt $m$ would be: 
$$P[\bigcup_i E_i=1]= P[E_1=1 \lor (E_1=0 \land \bigcup_i=E_i=1)] |$$
We can recursively repeat the above move, and separate the probabilities by mutual independence:
$$=P[E_1=1]+ P[E_1=0 \land E_2=1]+...$$
$$= \frac{n}{N} + \frac{N-n}{N}\frac{n}{N-1}+...+(\prod_{i=0}^{m-1} \frac{N-i-n}{N-i})\frac n {N-m}$$
Which is a start. There's some neat binomial tricks here, I may come back to demonstrate them later if there's interest.