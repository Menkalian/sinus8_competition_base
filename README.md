# Sinus 8

Sinus 8 is a revolutionary hash algorithm. With it's modern algebra and precise handling of numbers, it should only be used in the most modern applications with the highest requirements for technology.

Handle with care!

## Testcases
The following testcases are used to verify the implementation:

````
"12345678"                 -> 0x38
"123456781234567812345678" -> 0x35
"12345678ABCDEFGH12345678" -> 0xAA
""                         -> 0x00
All AsciiChars in order    -> 0x57
````

## Original implementation
This is the original implementation created by the visionary Merten Warnecke, who envisioned, named and specified the Sinus8 Algorithm.

````java
package org.example;

import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.nio.charset.StandardCharsets;
import java.util.Random;

public class Main {

    private static final int mMax = 8;

    public static void main(String[] args) {
        byte[] bytes = new byte[10000000];
        new Random().nextBytes(bytes);
        bytes = "123456781234567812345678".getBytes(StandardCharsets.US_ASCII);
        long start = System.currentTimeMillis();
        byte b = sin8(bytes);
        long end = System.currentTimeMillis();
        System.out.printf("Der Algo braucht: %dms%n", end - start);
        System.out.printf("%02X %n", b);
        System.out.printf("%d %n", Byte.toUnsignedInt(b));
        System.out.printf("%s %n", Integer.toBinaryString(b));
    }

    private static byte sin8(byte[] text) {
        double[] hashValues = new double[mMax];
        int outerLoop = text.length / mMax;
        int innerLoop = mMax;
        int last = outerLoop - 1;
        for (int i = 0; i <= outerLoop; i++) {
            if (i > last) {
                innerLoop = text.length - (i * mMax);
            }
            for (int j = 0; j < innerLoop; j++) {
                int dez = text[(i * mMax) + j];
                double v = hashValues[j] + Math.sin(dez);
                if (v > 1) {
                    hashValues[j] = v % 1 - 1;
                } else if (v < -1) {
                    hashValues[j] = v % 1 + 1;
                } else {
                    hashValues[j] = v;
                }
            }
        }

        byte hash = 0x00;
        for (int i = 0; i < hashValues.length; i++) {
            if (hashValues[i] > 0) {
                hash = setBit(hash, i, true);
            } else if (hashValues[i] < 0) {
                hash = setBit(hash, i, false);
            }
            System.out.println(String.format("%8s", Integer.toBinaryString(hash & 0xFF)).replace(' ', '0'));
        }
        return hash;
    }

    private static byte setBit(byte hash, int pos, boolean posOrNeg) {
        if (posOrNeg) {
            hash |= 1 << (mMax - 1 - pos) % 8;
        } else {
            hash &= ~(1 << (mMax - 1 - pos) % 8);
        }
        return hash;
    }
}
````