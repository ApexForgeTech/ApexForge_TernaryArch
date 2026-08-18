 Burada:

* Binary → Decimal → Ternary çevirmə
* Ternary → Decimal çevirmə
* Ternary → Binary
* Binary → Ternary
* Hexadecimal → Decimal/Binary/Ternary
* Hesablama qaydaları
* `0, 1, 2` məsələsi
* Ternary conversion cədvəli
* Binary / Ternary / Decimal / Hex müqayisəsi

hamısı var.

Aşağıdakı məzmunu məsələn `NUMBER_SYSTEMS.md` kimi saxlaya bilərsən:

````md
# Number Systems Reference

This document describes the number systems used by the ApexForge
Ternary CPU / VM project.

The VM uses **ternary values internally**, while the host machine
uses the normal binary representation.

---

# 1. Number Systems

The main number systems are:

| System | Base | Digits |
|---|---:|---|
| Binary | 2 | 0, 1 |
| Ternary | 3 | 0, 1, 2 |
| Decimal | 10 | 0-9 |
| Hexadecimal | 16 | 0-9, A-F |

The base determines how many different digits are available.

---

# 2. Binary

Binary is base 2.

It has only two digits:

```text
0
1
````

Examples:

```text
0
1
10
11
100
101
110
111
1000
1001
1010
```

Binary positional notation uses powers of 2.

For example:

```text
1011₂
```

means:

```text
1 × 2³
+
0 × 2²
+
1 × 2¹
+
1 × 2⁰
```

Therefore:

```text
1 × 8
+ 0 × 4
+ 1 × 2
+ 1 × 1
= 11
```

So:

```text
1011₂ = 11₁₀
```

---

# 3. Ternary

Ternary is base 3.

It has three digits:

```text
0
1
2
```

There is no digit `3`.

When the value reaches 3, a new position is created.

For example:

```text
0
1
2
10
11
12
20
21
22
100
101
102
110
...
```

This is similar to binary:

Binary:

```text
0
1
10
11
100
```

Ternary:

```text
0
1
2
10
11
12
20
```

The difference is the base.

---

# 4. Decimal 2 in Ternary

Decimal `2` does NOT need a special conversion.

Both systems contain the digit `2`.

Therefore:

```text
2₁₀ = 2₃
```

Example:

```text
2 % 3 = 2
2 / 3 = 0
```

The remainder is `2`, so:

```text
2₁₀ = 2₃
```

---

# 5. Decimal to Ternary

The general algorithm is:

1. Divide the decimal number by 3.
2. Save the remainder.
3. Divide the quotient by 3 again.
4. Continue until the quotient becomes 0.
5. Read the remainders from bottom to top.

---

## Example: Decimal 5

```text
5 / 3 = 1 remainder 2
1 / 3 = 0 remainder 1
```

Read remainders from bottom to top:

```text
12
```

Therefore:

```text
5₁₀ = 12₃
```

---

## Example: Decimal 10

```text
10 / 3 = 3 remainder 1
3  / 3 = 1 remainder 0
1  / 3 = 0 remainder 1
```

Read upwards:

```text
101
```

Therefore:

```text
10₁₀ = 101₃
```

---

## Example: Decimal 42

```text
42 / 3 = 14 remainder 0
14 / 3 = 4  remainder 2
4  / 3 = 1  remainder 1
1  / 3 = 0  remainder 1
```

Read from bottom to top:

```text
1120
```

Therefore:

```text
42₁₀ = 1120₃
```

This is exactly the algorithm implemented by:

```rust
pub fn from_u64(mut value: u64) -> Self {
    let mut trits = [Trit::Zero; REGISTER_WIDTH];

    for i in 0..REGISTER_WIDTH {
        let digit = (value % 3) as u8;

        trits[i] = Trit::from_u8(digit).unwrap();

        value /= 3;

        if value == 0 {
            break;
        }
    }

    Self { trits }
}
```

---

# 6. Ternary to Decimal

To convert ternary to decimal, multiply every digit by the
corresponding power of 3.

The positions are:

```text
... 3⁴  3³  3²  3¹  3⁰
... 81  27   9    3    1
```

---

## Example: 1120₃

```text
1120₃
```

Calculate:

```text
1 × 3³
+
1 × 3²
+
2 × 3¹
+
0 × 3⁰
```

Therefore:

```text
1 × 27
+
1 × 9
+
2 × 3
+
0 × 1

= 27 + 9 + 6
= 42
```

So:

```text
1120₃ = 42₁₀
```

This is implemented by:

```rust
pub fn to_u64(&self) -> u64 {
    let mut result = 0u64;
    let mut power = 1u64;

    for trit in &self.trits {
        result += trit.value() as u64 * power;
        power *= 3;
    }

    result
}
```

---

# 7. Binary to Decimal

Use powers of 2.

Example:

```text
1101₂
```

Calculate:

```text
1 × 2³
+
1 × 2²
+
0 × 2¹
+
1 × 2⁰
```

Therefore:

```text
8 + 4 + 0 + 1 = 13
```

So:

```text
1101₂ = 13₁₀
```

---

# 8. Decimal to Binary

Repeatedly divide by 2 and save the remainders.

Example:

```text
13 / 2 = 6 remainder 1
6  / 2 = 3 remainder 0
3  / 2 = 1 remainder 1
1  / 2 = 0 remainder 1
```

Read upwards:

```text
1101
```

Therefore:

```text
13₁₀ = 1101₂
```

---

# 9. Binary to Ternary

There is no direct "bit substitution".

Do this:

```text
Binary
  ↓
Decimal
  ↓
Ternary
```

Example:

```text
1010₂
```

First convert binary to decimal:

```text
1010₂
=
1×8 + 0×4 + 1×2 + 0×1
=
10₁₀
```

Then decimal to ternary:

```text
10 / 3 = 3 remainder 1
3  / 3 = 1 remainder 0
1  / 3 = 0 remainder 1
```

Therefore:

```text
1010₂ = 101₃
```

---

# 10. Ternary to Binary

Use the reverse process:

```text
Ternary
  ↓
Decimal
  ↓
Binary
```

Example:

```text
101₃
```

Convert to decimal:

```text
1×3² + 0×3¹ + 1×3⁰

= 9 + 0 + 1

= 10
```

Then:

```text
10₁₀ = 1010₂
```

Therefore:

```text
101₃ = 1010₂
```

---

# 11. Binary, Ternary and Decimal Table

| Decimal | Binary | Ternary |
| ------: | -----: | ------: |
|       0 |      0 |       0 |
|       1 |      1 |       1 |
|       2 |     10 |       2 |
|       3 |     11 |      10 |
|       4 |    100 |      11 |
|       5 |    101 |      12 |
|       6 |    110 |      20 |
|       7 |    111 |      21 |
|       8 |   1000 |      22 |
|       9 |   1001 |     100 |
|      10 |   1010 |     101 |
|      11 |   1011 |     102 |
|      12 |   1100 |     110 |
|      13 |   1101 |     111 |
|      14 |   1110 |     112 |
|      15 |   1111 |     120 |
|      16 |  10000 |     121 |
|      17 |  10001 |     122 |
|      18 |  10010 |     200 |
|      19 |  10011 |     201 |
|      20 |  10100 |     202 |
|      21 |  10101 |     210 |
|      22 |  10110 |     211 |
|      23 |  10111 |     212 |
|      24 |  11000 |     220 |
|      25 |  11001 |     221 |
|      26 |  11010 |     222 |
|      27 |  11011 |    1000 |
|      28 |  11100 |    1001 |
|      29 |  11101 |    1002 |
|      30 |  11110 |    1010 |
|      31 |  11111 |    1011 |
|      32 | 100000 |    1012 |

---

# 12. First Ternary Counting Sequence

The beginning of the ternary number system is:

```text
Decimal    Ternary
------------------
0          0
1          1
2          2
3          10
4          11
5          12
6          20
7          21
8          22
9          100
10         101
11         102
12         110
13         111
14         112
15         120
16         121
17         122
18         200
19         201
20         202
21         210
22         211
23         212
24         220
25         221
26         222
27         1000
```

Notice the important pattern:

```text
0
1
2
10
11
12
20
21
22
100
```

There is never:

```text
3
4
5
...
```

inside a ternary number.

---

# 13. Hexadecimal

Hexadecimal is base 16.

Digits:

```text
0 1 2 3 4 5 6 7 8 9 A B C D E F
```

The letters represent:

```text
A = 10
B = 11
C = 12
D = 13
E = 14
F = 15
```

---

# 14. Hexadecimal to Decimal

Example:

```text
2A₁₆
```

Calculate:

```text
2 × 16¹
+
A × 16⁰
```

Since:

```text
A = 10
```

we get:

```text
2 × 16 + 10
=
32 + 10
=
42
```

Therefore:

```text
2A₁₆ = 42₁₀
```

---

# 15. Decimal to Hexadecimal

Repeatedly divide by 16.

Example:

```text
42 / 16 = 2 remainder 10
2  / 16 = 0 remainder 2
```

Remainder `10` is hexadecimal `A`.

Read upwards:

```text
2A
```

Therefore:

```text
42₁₀ = 2A₁₆
```

---

# 16. Hexadecimal and Binary

Hexadecimal is particularly convenient for binary because:

```text
1 hexadecimal digit = 4 binary bits
```

Mapping:

| Hex | Binary |
| --- | ------ |
| 0   | 0000   |
| 1   | 0001   |
| 2   | 0010   |
| 3   | 0011   |
| 4   | 0100   |
| 5   | 0101   |
| 6   | 0110   |
| 7   | 0111   |
| 8   | 1000   |
| 9   | 1001   |
| A   | 1010   |
| B   | 1011   |
| C   | 1100   |
| D   | 1101   |
| E   | 1110   |
| F   | 1111   |

Example:

```text
2A₁₆
```

becomes:

```text
2 = 0010
A = 1010
```

Therefore:

```text
2A₁₆ = 00101010₂
```

---

# 17. Hexadecimal to Ternary

There is no simple one-digit mapping between hexadecimal
and ternary because their bases are different:

```text
Hexadecimal → base 16
Ternary     → base 3
```

Use:

```text
Hexadecimal
     ↓
Decimal
     ↓
Ternary
```

Example:

```text
2A₁₆
```

First:

```text
2A₁₆ = 42₁₀
```

Then:

```text
42₁₀ = 1120₃
```

Therefore:

```text
2A₁₆ = 1120₃
```

---

# 18. Complete Conversion Example

Convert:

```text
42₁₀
```

into every system.

### Decimal

```text
42₁₀
```

### Binary

```text
42₁₀ = 101010₂
```

### Ternary

```text
42₁₀ = 1120₃
```

### Hexadecimal

```text
42₁₀ = 2A₁₆
```

Therefore:

```text
Decimal      42
Binary       101010
Ternary      1120
Hexadecimal  2A
```

---

# 19. Same Value in Four Systems

| Decimal | Binary | Ternary | Hexadecimal |
| ------: | -----: | ------: | ----------: |
|       0 |      0 |       0 |           0 |
|       1 |      1 |       1 |           1 |
|       2 |     10 |       2 |           2 |
|       3 |     11 |      10 |           3 |
|       4 |    100 |      11 |           4 |
|       5 |    101 |      12 |           5 |
|       6 |    110 |      20 |           6 |
|       7 |    111 |      21 |           7 |
|       8 |   1000 |      22 |           8 |
|       9 |   1001 |     100 |           9 |
|      10 |   1010 |     101 |           A |
|      11 |   1011 |     102 |           B |
|      12 |   1100 |     110 |           C |
|      13 |   1101 |     111 |           D |
|      14 |   1110 |     112 |           E |
|      15 |   1111 |     120 |           F |
|      16 |  10000 |     121 |          10 |
|      17 |  10001 |     122 |          11 |
|      18 |  10010 |     200 |          12 |
|      19 |  10011 |     201 |          13 |
|      20 |  10100 |     202 |          14 |
|      21 |  10101 |     210 |          15 |
|      22 |  10110 |     211 |          16 |
|      23 |  10111 |     212 |          17 |
|      24 |  11000 |     220 |          18 |
|      25 |  11001 |     221 |          19 |
|      26 |  11010 |     222 |          1A |
|      27 |  11011 |    1000 |          1B |
|      28 |  11100 |    1001 |          1C |
|      29 |  11101 |    1002 |          1D |
|      30 |  11110 |    1010 |          1E |
|      31 |  11111 |    1011 |          1F |
|      32 | 100000 |    1012 |          20 |
|      33 | 100001 |    1020 |          21 |
|      34 | 100010 |    1021 |          22 |
|      35 | 100011 |    1022 |          23 |
|      36 | 100100 |    1100 |          24 |
|      37 | 100101 |    1101 |          25 |
|      38 | 100110 |    1102 |          26 |
|      39 | 100111 |    1110 |          27 |
|      40 | 101000 |    1111 |          28 |
|      41 | 101001 |    1112 |          29 |
|      42 | 101010 |    1120 |          2A |
|      43 | 101011 |    1121 |          2B |
|      44 | 101100 |    1122 |          2C |
|      45 | 101101 |    1200 |          2D |
|      46 | 101110 |    1201 |          2E |
|      47 | 101111 |    1202 |          2F |
|      48 | 110000 |    1210 |          30 |
|      49 | 110001 |    1211 |          31 |
|      50 | 110010 |    1212 |          32 |

---

# 20. Ternary Arithmetic

Ternary addition works similarly to binary addition, but the base is 3.

Basic rules:

```text
0 + 0 = 0
0 + 1 = 1
0 + 2 = 2

1 + 0 = 1
1 + 1 = 2
1 + 2 = 10

2 + 0 = 2
2 + 1 = 10
2 + 2 = 11
```

Important:

```text
1 + 2 = 10₃
```

because:

```text
1 + 2 = 3₁₀
3₁₀ = 10₃
```

Similarly:

```text
2 + 2 = 4₁₀
4₁₀ = 11₃
```

---

# 21. Ternary Carry

Example:

```text
  12
+ 21
----
```

Right side:

```text
2 + 1 = 3
```

which is:

```text
10₃
```

Write `0` and carry `1`.

Next:

```text
1 + 2 + 1 = 4
```

and:

```text
4₁₀ = 11₃
```

Therefore:

```text
  12₃
+ 21₃
-----
 110₃
```

Check:

```text
12₃ = 5₁₀
21₃ = 7₁₀

5 + 7 = 12

12₁₀ = 110₃
```

Correct.

---

# 22. Ternary Multiplication

Basic multiplication:

```text
0 × anything = 0

1 × anything = anything

2 × 0 = 0
2 × 1 = 2
2 × 2 = 11₃
```

Why?

```text
2 × 2 = 4₁₀
4₁₀ = 11₃
```

---

# 23. Ternary Subtraction

Example:

```text
  10₃
-  1₃
-----
```

Borrow from the `1`:

```text
10₃ = 3₁₀
3 - 1 = 2
```

Therefore:

```text
10₃ - 1₃ = 2₃
```

---

# 24. Ternary Register Representation

The VM uses:

```rust
pub const REGISTER_WIDTH: usize = 16;
```

Therefore:

```text
1 TernaryWord
=
16 trits
```

Each trit can contain:

```text
0
1
2
```

Example:

```text
TernaryWord:

[0, 2, 1, 1, 0, 0, ...]
```

Human-readable representation:

```text
1120
```

---

# 25. Ternary Word Capacity

A 16-trit word has:

```text
3¹⁶
```

possible combinations.

That is:

```text
43,046,721
```

different values.

Therefore an unsigned 16-trit word can represent:

```text
0
...
43,046,720
```

---

# 26. Host Memory Encoding

The virtual CPU uses:

```text
Trit = 0, 1, 2
```

but the host RAM stores bytes.

The current VM uses:

```text
2 bits per trit
```

Encoding:

```text
Trit    Encoded bits
--------------------
0       00
1       01
2       10
```

The combination:

```text
11
```

is currently unused.

---

# 27. TernaryWord → Host Bytes

A 16-trit word contains:

```text
16 × 2 = 32 bits
```

Therefore:

```text
32 bits = 4 bytes
```

So:

```text
TernaryWord
     ↓
16 trits
     ↓
32 encoded bits
     ↓
4 host bytes
```

This is implemented by:

```rust
encode_word()
```

---

# 28. Host Bytes → TernaryWord

The reverse operation is:

```text
4 host bytes
     ↓
32 encoded bits
     ↓
16 × 2-bit fields
     ↓
16 trits
     ↓
TernaryWord
```

This is implemented by:

```rust
decode_word()
```

---

# 29. Important Architecture Rule

The VM has twoƏlbəttə. Sənin **ternary CPU/VM layihən** üçün bir Markdown sənədi hazırladım. Burada:

* Binary → Decimal → Ternary çevirmə
* Ternary → Decimal çevirmə
* Ternary → Binary
* Binary → Ternary
* Hexadecimal → Decimal/Binary/Ternary
* Hesablama qaydaları
* `0, 1, 2` məsələsi
* Ternary conversion cədvəli
* Binary / Ternary / Decimal / Hex müqayisəsi

hamısı var.

Aşağıdakı məzmunu məsələn `NUMBER_SYSTEMS.md` kimi saxlaya bilərsən:

````md
# Number Systems Reference

This document describes the number systems used by the ApexForge
Ternary CPU / VM project.

The VM uses **ternary values internally**, while the host machine
uses the normal binary representation.

---

# 1. Number Systems

The main number systems are:

| System | Base | Digits |
|---|---:|---|
| Binary | 2 | 0, 1 |
| Ternary | 3 | 0, 1, 2 |
| Decimal | 10 | 0-9 |
| Hexadecimal | 16 | 0-9, A-F |

The base determines how many different digits are available.

---

# 2. Binary

Binary is base 2.

It has only two digits:

```text
0
1
````

Examples:

```text
0
1
10
11
100
101
110
111
1000
1001
1010
```

Binary positional notation uses powers of 2.

For example:

```text
1011₂
```

means:

```text
1 × 2³
+
0 × 2²
+
1 × 2¹
+
1 × 2⁰
```

Therefore:

```text
1 × 8
+ 0 × 4
+ 1 × 2
+ 1 × 1
= 11
```

So:

```text
1011₂ = 11₁₀
```

---

# 3. Ternary

Ternary is base 3.

It has three digits:

```text
0
1
2
```

There is no digit `3`.

When the value reaches 3, a new position is created.

For example:

```text
0
1
2
10
11
12
20
21
22
100
101
102
110
...
```

This is similar to binary:

Binary:

```text
0
1
10
11
100
```

Ternary:

```text
0
1
2
10
11
12
20
```

The difference is the base.

---

# 4. Decimal 2 in Ternary

Decimal `2` does NOT need a special conversion.

Both systems contain the digit `2`.

Therefore:

```text
2₁₀ = 2₃
```

Example:

```text
2 % 3 = 2
2 / 3 = 0
```

The remainder is `2`, so:

```text
2₁₀ = 2₃
```

---

# 5. Decimal to Ternary

The general algorithm is:

1. Divide the decimal number by 3.
2. Save the remainder.
3. Divide the quotient by 3 again.
4. Continue until the quotient becomes 0.
5. Read the remainders from bottom to top.

---

## Example: Decimal 5

```text
5 / 3 = 1 remainder 2
1 / 3 = 0 remainder 1
```

Read remainders from bottom to top:

```text
12
```

Therefore:

```text
5₁₀ = 12₃
```

---

## Example: Decimal 10

```text
10 / 3 = 3 remainder 1
3  / 3 = 1 remainder 0
1  / 3 = 0 remainder 1
```

Read upwards:

```text
101
```

Therefore:

```text
10₁₀ = 101₃
```

---

## Example: Decimal 42

```text
42 / 3 = 14 remainder 0
14 / 3 = 4  remainder 2
4  / 3 = 1  remainder 1
1  / 3 = 0  remainder 1
```

Read from bottom to top:

```text
1120
```

Therefore:

```text
42₁₀ = 1120₃
```

This is exactly the algorithm implemented by:

```rust
pub fn from_u64(mut value: u64) -> Self {
    let mut trits = [Trit::Zero; REGISTER_WIDTH];

    for i in 0..REGISTER_WIDTH {
        let digit = (value % 3) as u8;

        trits[i] = Trit::from_u8(digit).unwrap();

        value /= 3;

        if value == 0 {
            break;
        }
    }

    Self { trits }
}
```

---

# 6. Ternary to Decimal

To convert ternary to decimal, multiply every digit by the
corresponding power of 3.

The positions are:

```text
... 3⁴  3³  3²  3¹  3⁰
... 81  27   9    3    1
```

---

## Example: 1120₃

```text
1120₃
```

Calculate:

```text
1 × 3³
+
1 × 3²
+
2 × 3¹
+
0 × 3⁰
```

Therefore:

```text
1 × 27
+
1 × 9
+
2 × 3
+
0 × 1

= 27 + 9 + 6
= 42
```

So:

```text
1120₃ = 42₁₀
```

This is implemented by:

```rust
pub fn to_u64(&self) -> u64 {
    let mut result = 0u64;
    let mut power = 1u64;

    for trit in &self.trits {
        result += trit.value() as u64 * power;
        power *= 3;
    }

    result
}
```

---

# 7. Binary to Decimal

Use powers of 2.

Example:

```text
1101₂
```

Calculate:

```text
1 × 2³
+
1 × 2²
+
0 × 2¹
+
1 × 2⁰
```

Therefore:

```text
8 + 4 + 0 + 1 = 13
```

So:

```text
1101₂ = 13₁₀
```

---

# 8. Decimal to Binary

Repeatedly divide by 2 and save the remainders.

Example:

```text
13 / 2 = 6 remainder 1
6  / 2 = 3 remainder 0
3  / 2 = 1 remainder 1
1  / 2 = 0 remainder 1
```

Read upwards:

```text
1101
```

Therefore:

```text
13₁₀ = 1101₂
```

---

# 9. Binary to Ternary

There is no direct "bit substitution".

Do this:

```text
Binary
  ↓
Decimal
  ↓
Ternary
```

Example:

```text
1010₂
```

First convert binary to decimal:

```text
1010₂
=
1×8 + 0×4 + 1×2 + 0×1
=
10₁₀
```

Then decimal to ternary:

```text
10 / 3 = 3 remainder 1
3  / 3 = 1 remainder 0
1  / 3 = 0 remainder 1
```

Therefore:

```text
1010₂ = 101₃
```

---

# 10. Ternary to Binary

Use the reverse process:

```text
Ternary
  ↓
Decimal
  ↓
Binary
```

Example:

```text
101₃
```

Convert to decimal:

```text
1×3² + 0×3¹ + 1×3⁰

= 9 + 0 + 1

= 10
```

Then:

```text
10₁₀ = 1010₂
```

Therefore:

```text
101₃ = 1010₂
```

---

# 11. Binary, Ternary and Decimal Table

| Decimal | Binary | Ternary |
| ------: | -----: | ------: |
|       0 |      0 |       0 |
|       1 |      1 |       1 |
|       2 |     10 |       2 |
|       3 |     11 |      10 |
|       4 |    100 |      11 |
|       5 |    101 |      12 |
|       6 |    110 |      20 |
|       7 |    111 |      21 |
|       8 |   1000 |      22 |
|       9 |   1001 |     100 |
|      10 |   1010 |     101 |
|      11 |   1011 |     102 |
|      12 |   1100 |     110 |
|      13 |   1101 |     111 |
|      14 |   1110 |     112 |
|      15 |   1111 |     120 |
|      16 |  10000 |     121 |
|      17 |  10001 |     122 |
|      18 |  10010 |     200 |
|      19 |  10011 |     201 |
|      20 |  10100 |     202 |
|      21 |  10101 |     210 |
|      22 |  10110 |     211 |
|      23 |  10111 |     212 |
|      24 |  11000 |     220 |
|      25 |  11001 |     221 |
|      26 |  11010 |     222 |
|      27 |  11011 |    1000 |
|      28 |  11100 |    1001 |
|      29 |  11101 |    1002 |
|      30 |  11110 |    1010 |
|      31 |  11111 |    1011 |
|      32 | 100000 |    1012 |

---

# 12. First Ternary Counting Sequence

The beginning of the ternary number system is:

```text
Decimal    Ternary
------------------
0          0
1          1
2          2
3          10
4          11
5          12
6          20
7          21
8          22
9          100
10         101
11         102
12         110
13         111
14         112
15         120
16         121
17         122
18         200
19         201
20         202
21         210
22         211
23         212
24         220
25         221
26         222
27         1000
```

Notice the important pattern:

```text
0
1
2
10
11
12
20
21
22
100
```

There is never:

```text
3
4
5
...
```

inside a ternary number.

---

# 13. Hexadecimal

Hexadecimal is base 16.

Digits:

```text
0 1 2 3 4 5 6 7 8 9 A B C D E F
```

The letters represent:

```text
A = 10
B = 11
C = 12
D = 13
E = 14
F = 15
```

---

# 14. Hexadecimal to Decimal

Example:

```text
2A₁₆
```

Calculate:

```text
2 × 16¹
+
A × 16⁰
```

Since:

```text
A = 10
```

we get:

```text
2 × 16 + 10
=
32 + 10
=
42
```

Therefore:

```text
2A₁₆ = 42₁₀
```

---

# 15. Decimal to Hexadecimal

Repeatedly divide by 16.

Example:

```text
42 / 16 = 2 remainder 10
2  / 16 = 0 remainder 2
```

Remainder `10` is hexadecimal `A`.

Read upwards:

```text
2A
```

Therefore:

```text
42₁₀ = 2A₁₆
```

---

# 16. Hexadecimal and Binary

Hexadecimal is particularly convenient for binary because:

```text
1 hexadecimal digit = 4 binary bits
```

Mapping:

| Hex | Binary |
| --- | ------ |
| 0   | 0000   |
| 1   | 0001   |
| 2   | 0010   |
| 3   | 0011   |
| 4   | 0100   |
| 5   | 0101   |
| 6   | 0110   |
| 7   | 0111   |
| 8   | 1000   |
| 9   | 1001   |
| A   | 1010   |
| B   | 1011   |
| C   | 1100   |
| D   | 1101   |
| E   | 1110   |
| F   | 1111   |

Example:

```text
2A₁₆
```

becomes:

```text
2 = 0010
A = 1010
```

Therefore:

```text
2A₁₆ = 00101010₂
```

---

# 17. Hexadecimal to Ternary

There is no simple one-digit mapping between hexadecimal
and ternary because their bases are different:

```text
Hexadecimal → base 16
Ternary     → base 3
```

Use:

```text
Hexadecimal
     ↓
Decimal
     ↓
Ternary
```

Example:

```text
2A₁₆
```

First:

```text
2A₁₆ = 42₁₀
```

Then:

```text
42₁₀ = 1120₃
```

Therefore:

```text
2A₁₆ = 1120₃
```

---

# 18. Complete Conversion Example

Convert:

```text
42₁₀
```

into every system.

### Decimal

```text
42₁₀
```

### Binary

```text
42₁₀ = 101010₂
```

### Ternary

```text
42₁₀ = 1120₃
```

### Hexadecimal

```text
42₁₀ = 2A₁₆
```

Therefore:

```text
Decimal      42
Binary       101010
Ternary      1120
Hexadecimal  2A
```

---

# 19. Same Value in Four Systems

| Decimal | Binary | Ternary | Hexadecimal |
| ------: | -----: | ------: | ----------: |
|       0 |      0 |       0 |           0 |
|       1 |      1 |       1 |           1 |
|       2 |     10 |       2 |           2 |
|       3 |     11 |      10 |           3 |
|       4 |    100 |      11 |           4 |
|       5 |    101 |      12 |           5 |
|       6 |    110 |      20 |           6 |
|       7 |    111 |      21 |           7 |
|       8 |   1000 |      22 |           8 |
|       9 |   1001 |     100 |           9 |
|      10 |   1010 |     101 |           A |
|      11 |   1011 |     102 |           B |
|      12 |   1100 |     110 |           C |
|      13 |   1101 |     111 |           D |
|      14 |   1110 |     112 |           E |
|      15 |   1111 |     120 |           F |
|      16 |  10000 |     121 |          10 |
|      17 |  10001 |     122 |          11 |
|      18 |  10010 |     200 |          12 |
|      19 |  10011 |     201 |          13 |
|      20 |  10100 |     202 |          14 |
|      21 |  10101 |     210 |          15 |
|      22 |  10110 |     211 |          16 |
|      23 |  10111 |     212 |          17 |
|      24 |  11000 |     220 |          18 |
|      25 |  11001 |     221 |          19 |
|      26 |  11010 |     222 |          1A |
|      27 |  11011 |    1000 |          1B |
|      28 |  11100 |    1001 |          1C |
|      29 |  11101 |    1002 |          1D |
|      30 |  11110 |    1010 |          1E |
|      31 |  11111 |    1011 |          1F |
|      32 | 100000 |    1012 |          20 |
|      33 | 100001 |    1020 |          21 |
|      34 | 100010 |    1021 |          22 |
|      35 | 100011 |    1022 |          23 |
|      36 | 100100 |    1100 |          24 |
|      37 | 100101 |    1101 |          25 |
|      38 | 100110 |    1102 |          26 |
|      39 | 100111 |    1110 |          27 |
|      40 | 101000 |    1111 |          28 |
|      41 | 101001 |    1112 |          29 |
|      42 | 101010 |    1120 |          2A |
|      43 | 101011 |    1121 |          2B |
|      44 | 101100 |    1122 |          2C |
|      45 | 101101 |    1200 |          2D |
|      46 | 101110 |    1201 |          2E |
|      47 | 101111 |    1202 |          2F |
|      48 | 110000 |    1210 |          30 |
|      49 | 110001 |    1211 |          31 |
|      50 | 110010 |    1212 |          32 |

---

# 20. Ternary Arithmetic

Ternary addition works similarly to binary addition, but the base is 3.

Basic rules:

```text
0 + 0 = 0
0 + 1 = 1
0 + 2 = 2

1 + 0 = 1
1 + 1 = 2
1 + 2 = 10

2 + 0 = 2
2 + 1 = 10
2 + 2 = 11
```

Important:

```text
1 + 2 = 10₃
```

because:

```text
1 + 2 = 3₁₀
3₁₀ = 10₃
```

Similarly:

```text
2 + 2 = 4₁₀
4₁₀ = 11₃
```

---

# 21. Ternary Carry

Example:

```text
  12
+ 21
----
```

Right side:

```text
2 + 1 = 3
```

which is:

```text
10₃
```

Write `0` and carry `1`.

Next:

```text
1 + 2 + 1 = 4
```

and:

```text
4₁₀ = 11₃
```

Therefore:

```text
  12₃
+ 21₃
-----
 110₃
```

Check:

```text
12₃ = 5₁₀
21₃ = 7₁₀

5 + 7 = 12

12₁₀ = 110₃
```

Correct.

---

# 22. Ternary Multiplication

Basic multiplication:

```text
0 × anything = 0

1 × anything = anything

2 × 0 = 0
2 × 1 = 2
2 × 2 = 11₃
```

Why?

```text
2 × 2 = 4₁₀
4₁₀ = 11₃
```

---

# 23. Ternary Subtraction

Example:

```text
  10₃
-  1₃
-----
```

Borrow from the `1`:

```text
10₃ = 3₁₀
3 - 1 = 2
```

Therefore:

```text
10₃ - 1₃ = 2₃
```

---

# 24. Ternary Register Representation

The VM uses:

```rust
pub const REGISTER_WIDTH: usize = 16;
```

Therefore:

```text
1 TernaryWord
=
16 trits
```

Each trit can contain:

```text
0
1
2
```

Example:

```text
TernaryWord:

[0, 2, 1, 1, 0, 0, ...]
```

Human-readable representation:

```text
1120
```

---

# 25. Ternary Word Capacity

A 16-trit word has:

```text
3¹⁶
```

possible combinations.

That is:

```text
43,046,721
```

different values.

Therefore an unsigned 16-trit word can represent:

```text
0
...
43,046,720
```

---

# 26. Host Memory Encoding

The virtual CPU uses:

```text
Trit = 0, 1, 2
```

but the host RAM stores bytes.

The current VM uses:

```text
2 bits per trit
```

Encoding:

```text
Trit    Encoded bits
--------------------
0       00
1       01
2       10
```

The combination:

```text
11
```

is currently unused.

---

# 27. TernaryWord → Host Bytes

A 16-trit word contains:

```text
16 × 2 = 32 bits
```

Therefore:

```text
32 bits = 4 bytes
```

So:

```text
TernaryWord
     ↓
16 trits
     ↓
32 encoded bits
     ↓
4 host bytes
```

This is implemented by:

```rust
encode_word()
```

---

# 28. Host Bytes → TernaryWord

The reverse operation is:

```text
4 host bytes
     ↓
32 encoded bits
     ↓
16 × 2-bit fields
     ↓
16 trits
     ↓
TernaryWord
```

This is implemented by:

```rust
decode_word()
```

---

# 29. Important Architecture Rule

The VM has two different representations.

## Virtual CPU representation

```text
Trit
TernaryWord
Ternary registers
Ternary arithmetic
Ternary addresses
```

## Host representation

```text
u8
u64
usize
bytes
pointers
mmap()
```

The conversion layer connects them:

```text
       TERNARY CPU
            │
            │
     TernaryWord
            │
            ↓
     conversion.rs
       ↙       ↘
   encode     decode
      ↓          ↑
   bytes       bytes
      ↓          ↑
   host_memory.rs
      ↓
     mmap()
      ↓
 Linux virtual memory
```

---

# 30. Important Note

A software ternary VM does not make the physical x86-64 CPU
a ternary CPU.

The host CPU is still binary.

The VM only defines a new virtual architecture:

```text
Virtual ISA
Virtual registers
Virtual ternary arithmetic
Virtual ternary memory model
Virtual instructions
```

The host machine executes the VM implementation using normal
binary machine instructions.

This is exactly what makes the project possible on ordinary hardware.

```
```
 different representations.

## Virtual CPU representation

```text
Trit
TernaryWord
Ternary registers
Ternary arithmetic
Ternary addresses
```

## Host representation

```text
u8
u64
usize
bytes
pointers
mmap()
```

The conversion layer connects them:

```text
       TERNARY CPU
            │
            │
     TernaryWord
            │
            ↓
     conversion.rs
       ↙       ↘
   encode     decode
      ↓          ↑
   bytes       bytes
      ↓          ↑
   host_memory.rs
      ↓
     mmap()
      ↓
 Linux virtual memory
```

---

# 30. Important Note

A software ternary VM does not make the physical x86-64 CPU
a ternary CPU.

The host CPU is still binary.

The VM only defines a new virtual architecture:

```text
Virtual ISA
Virtual registers
Virtual ternary arithmetic
Virtual ternary memory model
Virtual instructions
```

The host machine executes the VM implementation using normal
binary machine instructions.

This is exactly what makes the project possible on ordinary hardware.

```
```
