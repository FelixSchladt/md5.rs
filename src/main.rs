use std::env;
use std::fs;

// This has been written as an exercise, for the original specification please refer to:
// https://datatracker.ietf.org/doc/html/rfc1321


fn as_u32_le(array: &[u8]) -> u32 {
    ((array[0] as u32) << 0)
        + ((array[1] as u32) << 8)
        + ((array[2] as u32) << 16)
        + ((array[3] as u32) << 24)
}

fn md5(inital_msg: &str, initial_len: usize) {
    //initialize standard values
    let r: [u32; 64] = [
        7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5,
        9, 14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10,
        15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
    ];

    let k: [u32; 64] = [
        0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613,
        0xfd469501, 0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193,
        0xa679438e, 0x49b40821, 0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d,
        0x02441453, 0xd8a1e681, 0xe7d3fbc8, 0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
        0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a, 0xfffa3942, 0x8771f681, 0x6d9d6122,
        0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70, 0x289b7ec6, 0xeaa127fa,
        0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665, 0xf4292244,
        0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
        0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb,
        0xeb86d391,
    ];

    let mut h0: u32 = 0x67452301;
    let mut h1: u32 = 0xefcdab89;
    let mut h2: u32 = 0x98badcfe;
    let mut h3: u32 = 0x10325476;

    //read msg into vec
    let mut msg: Vec<u8> = Vec::new();
    for character in inital_msg.bytes() {
        msg.push(character);
    }

    //Add padding
    msg.push(128);
    let mut lc = msg.len();
    while ((lc + 8) % 64) != 0 {
        //add fill upd with zeros until 512bit block is complete
        msg.push(0);
        lc += 1;
    }
    let bitlen = initial_len * 8;
    let msg_len_le = bitlen.to_le_bytes();
    //add length to the end of the last block
    for byte in msg_len_le {
        msg.push(byte);
    }

    //iterate through 512 Bit blocks
    let mut offset = 0;
    let total_len = msg.len();
    while offset < total_len - 1 {
        let mut chunk: [u32; 16] = [0; 16];
        for i in 0..16 {
            //represent each block as 16 u32
            chunk[i] = as_u32_le(&msg[(offset + i * 4)..((offset + i * 4) + 4)]);
        }

        let mut a: u32 = h0;
        let mut b: u32 = h1;
        let mut c: u32 = h2;
        let mut d: u32 = h3;
        
        //compression
        for i in 0..64 {
            let mut f = 0;
            let mut g = 0;
            if i < 16 {
                f = (b & c) | ((!b) & d);
                g = i;
            } else if i < 32 {
                f = (d & b) | ((!d) & c);
                g = (5 * i + 1) % 16;
            } else if i < 48 {
                f = b ^ c ^ d;
                g = (3 * i + 5) % 16;
            } else {
                f = c ^ (b | (!d));
                g = (7 * i) % 16;
            }

            //Iterate through variables
            let temp: u32 = d;
            d = c;
            c = b;
            b = b.wrapping_add(
                a.wrapping_add(f.wrapping_add(k[i].wrapping_add(chunk[g])))
                    .rotate_left(r[i]),
            );
            a = temp;
        }

        //add with overflowing
        h0 = h0.wrapping_add(a);
        h1 = h1.wrapping_add(b);
        h2 = h2.wrapping_add(c);
        h3 = h3.wrapping_add(d);

        offset += 64;
    }

    //print by converting to big endian
    println!(
        "{:x}{:x}{:x}{:x}",
        h0.to_be(),
        h1.to_be(),
        h2.to_be(),
        h3.to_be()
    );
}

fn main() {
    //collect args
    let mut args: Vec<String> = env::args().collect();
    let len = args.len();
    args.push(String::from(""));

    //read second parameter as string
    if &args[1] == "-s" && len == 3 {
        md5(&args[2], args[2].len());
    }
    //check for help message
    else if &args[1] == "-h" || &args[1] == "--help" {
        println!("Usage: {} [OPTION]... [FILE]...\nPrint or check MD5 (128-bit) checksums.\n    -h, --help      show this menu\n    -s              take string parameter as hash source
", args[0])
    }
    //read text from file
    else if len == 2 {
        let contents = fs::read_to_string(&args[1]).expect("Error reading the given file");
        md5(&contents[..], contents.len());
    }
    //show message if invalid options are entered
    else {
        println!(
            "Error: invalid options set.\nTry '{} --help' for more information.",
            args[0]
        );
    }
}
