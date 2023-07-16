use num_bigint::BigInt;

#[derive(Debug, Copy, Clone)]
enum FloatKind {
    Regular { exp: i32 },
    Zero,
    Infinity,
    NaN,
}

#[derive(Debug, Clone)]
struct ArbFloat {
    kind: FloatKind,
    num: BigInt,
}

impl ArbFloat {
    fn new(mut kind: FloatKind, mut num: BigInt) -> Self {
        if let FloatKind::Regular { exp } = &mut kind {
            let adjustment = num.trailing_zeros().unwrap() as i32;
            *exp += adjustment;
            num >>= adjustment;
        }
        Self { kind, num }
    }
}

type IntStorage = u64;

#[derive(Debug, Copy, Clone)]
struct FormatDesc {
    frac_bits: u8,
    exp_bits: u8,
}

impl FormatDesc {
    const BINARY32: Self = Self {
        frac_bits: 23,
        exp_bits: 8,
    };
    const BINARY64: Self = Self {
        frac_bits: 52,
        exp_bits: 11,
    };

    fn precision(&self) -> i32 {
        self.frac_bits as i32 + 1
    }

    fn mask(bits: u32) -> IntStorage {
        IntStorage::MAX >> (IntStorage::BITS - bits)
    }

    fn frac_mask(&self) -> IntStorage {
        Self::mask(self.frac_bits as u32)
    }

    fn frac_shift(&self) -> IntStorage {
        0
    }

    fn biased_exp_mask(&self) -> IntStorage {
        Self::mask(self.exp_bits as u32)
    }

    fn biased_exp_shift(&self) -> IntStorage {
        self.frac_shift() + self.frac_bits as IntStorage
    }

    fn exp_bias(&self) -> i32 {
        (1 << (self.exp_bits - 1)) - 1
    }

    fn sign_mask(&self) -> IntStorage {
        1
    }

    fn sign_shift(&self) -> IntStorage {
        self.biased_exp_shift() + self.exp_bits as IntStorage
    }

    fn integer_bit(&self) -> IntStorage {
        1 << self.frac_bits
    }
}

fn parse(desc: FormatDesc, storage: IntStorage) -> ArbFloat {
    let frac = (storage >> desc.frac_shift()) & desc.frac_mask();
    let biased_exp = (storage >> desc.biased_exp_shift()) & desc.biased_exp_mask();
    let sign = ((storage >> desc.sign_shift()) & desc.sign_mask()) != 0;

    // We add the precision to the bias to be able to interpret the fraction
    // field as an integer rather than as a fixed-point number in [1, 2).
    // We've essentially moved the radix point to the right by multiplying
    // the fraction by `2^(precision - 1)`, so we compensate by subtracting
    // `precision - 1` from the exponent.
    let exp = biased_exp as i32 - (desc.exp_bias() + desc.precision() - 1);
    let mut num = BigInt::from(if sign { -1 } else { 1 });
    let kind = if biased_exp == desc.biased_exp_mask() {
        if frac == 0 {
            FloatKind::Infinity
        } else {
            FloatKind::NaN
        }
    } else if biased_exp == 0 {
        if frac == 0 {
            FloatKind::Zero
        } else {
            // Subnormals. Multiply by `frac` here only to preserve the sign of zeros.
            num *= frac;
            FloatKind::Regular { exp: exp + 1 }
        }
    } else {
        num *= frac | desc.integer_bit();
        FloatKind::Regular { exp }
    };
    ArbFloat::new(kind, num)
}

fn print_examples() {
    println!("{:?}", parse(FormatDesc::BINARY32, 0x8000_0000)); // -0f32
    println!("{:?}", parse(FormatDesc::BINARY32, 0x7F80_0000)); // f32::INFINITY
    println!("{:?}", parse(FormatDesc::BINARY32, 0x7FC0_0000)); // f32::NAN
    println!("{:?}", parse(FormatDesc::BINARY32, 0x3F80_0000)); // 1f32
    println!("{:?}", parse(FormatDesc::BINARY64, 0x3FF0_0000_0000_0000)); // 1f64
}

fn print_binary3() {
    const BINARY3: FormatDesc = FormatDesc {
        frac_bits: 1,
        exp_bits: 1,
    };
    for x in 0..8 {
        println!("{:?}", parse(BINARY3, x));
    }
}

fn main() {
    print_examples();
    println!("");
    print_binary3();
}
