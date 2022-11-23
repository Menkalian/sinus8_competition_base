extern crate rayon;

// use rayon::iter::IntoParallelRefIterator;
// use rayon::iter::ParallelIterator;

const SINUS_GRADE: usize = 8;
// const MULTITHREAD_THRESHOLD: usize = 64;
// Include Lookuptable
include!(concat!(env!("OUT_DIR"), "/lut_sin.rs"));

pub fn sinus8(data: &str) -> u8 {
    let bytes = data.as_bytes();
    // if false && bytes.len() > MULTITHREAD_THRESHOLD {
    //     let mut grouped_bytes: [Vec<u8>; SINUS_GRADE] = [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    //     for i in 0..bytes.len() {
    //         grouped_bytes[i % SINUS_GRADE].push(bytes[i]);
    //     }
    //
    //     let res: Vec<f64> = grouped_bytes
    //         .par_iter()
    //         .map(|v| {
    //             let mut cumul = 0.0f64;
    //
    //             for b in v {
    //                 cumul += SIN_LUT[*b as usize];
    //                 if cumul.abs() > 1.0 {
    //                     if cumul > 0.0 {
    //                         cumul = cumul % 1.0 - 1.0;
    //                     } else {
    //                         cumul = cumul % 1.0 + 1.0;
    //                     }
    //                 }
    //             }
    //
    //             cumul
    //         })
    //         .collect();
    //
    //     let mut hash: u8 = 0;
    //     for i in 0..SINUS_GRADE {
    //         hash = hash << 1;
    //         if res[i] > 0.0 {
    //             hash += 1;
    //         }
    //     }
    //
    //     hash
    // } else {
        // avoid threading overhead
        let mut sin_cumul = [0; SINUS_GRADE];

        for i in 0..bytes.len() {
            sin_cumul[i % SINUS_GRADE] += SIN_LUT[bytes[i] as usize];
        }

        let mut hash: u8 = 0;
        for i in 0..SINUS_GRADE {
            hash = hash << 1;
            if sin_cumul[i] > 0 {
                hash += 1;
            }
        }

        hash
    // }
}

#[cfg(test)]
mod tests {
    use crate::sinus8;

    #[test]
    fn testcase1() {
        let result = sinus8("12345678");
        assert_eq!(result, 0x38)
    }

    #[test]
    fn testcase2() {
        let result = sinus8("123456781234567812345678");
        assert_eq!(result, 0x35)
    }

    #[test]
    fn testcase3() {
        let result = sinus8("12345678ABCDEFGH12345678");
        assert_eq!(result, 0xAA)
    }

    #[test]
    fn testcase4() {
        let result = sinus8("");
        assert_eq!(result, 0x00)
    }

    #[test]
    fn testcase5() {
        let data = (33..127).map(|c| c as u8 as char).collect::<String>();
        let result = sinus8(&data);
        assert_eq!(result, 0x57)
    }
}
