fn main() {
    let byte_sizes: [u8; 8] = [128,64,32,16,8,4,2,1]; // i love everything being immutable by default
    let mut saved_indices: [u8; 8] = [0; 8];

    let mut cur_total: u32 = 0;
    let target: u32 = 67;

    let target_as_u8= u8::try_from(target) // finds if ts cant be converted to u8
        .expect("this is out of size limit try again stupid why you trying to find a binary value with a decimal THAT IS NOT 8 FUCKING BYTES");

    for _i in 0..8 {
        if cur_total + byte_sizes[_i] as u32 <= target {
            saved_indices[_i] = 1;
            cur_total += byte_sizes[_i] as u32;
        }
    }

    println!("Target: {target}");
    for &bit in &saved_indices {
        print!("{bit}");
    }
    println!("\n{}", target_as_u8 as char); // as as as as as as i as as i as as as

}
