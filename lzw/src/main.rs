use std::collections::HashMap;


// https://rosettacode.org/wiki/LZW_compression#Rust
fn compress(data: &[u8]) -> Vec<u16> {
    let mut dict: HashMap<Vec<u8>, u16> = (0u16..=255).map(|i| (vec![i as u8], i)).collect();

    let mut w = Vec::new();
    let mut compressed = Vec::new();

    for &b in data {
        println!("{:?}", w);
        let mut wc = w.clone();
        wc.push(b);

        if dict.contains_key(&wc) {
            w = wc;
        } else {
            compressed.push(dict[&w]);

            dict.insert(wc, dict.len() as u16);
            w.clear();
            w.push(b);
        }
    }

    if !w.is_empty() {
        compressed.push(dict[&w]);
    }

    compressed
}

fn main() {
    let to_compress = b"booba dooba bobba lobba hey may lay ney";
    let compressed = compress(to_compress);
    println!("{:?}", compressed);

    // 39 * 8 < 28 * 16
    println!("{} {}", to_compress.len(), compressed.len());

 // Real LZW implementations pack the codes into a bitstream, using only as many bits per code as needed.
 // In practice LZW codes start only one bit wider than the input symbols (for 8-bit input, start at 9 bits) and grow as the dictionary fills
}
