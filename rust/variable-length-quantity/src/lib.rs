/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut res = vec![];

    for value in values {
        res.append(&mut to_bytes_single(*value));
    }
    res
}

fn to_bytes_single(mut value: u32) -> Vec<u8> {
    // over allocates, but avoids growth
    let mut res = Vec::with_capacity(4);

    // 0 must be handeled specially, because we need to push one byte
    if value == 0 {
        return vec![0];
    }

    while value > 0 {
        // take the lower 7 bits
        let mut tmp = (value & 0x7f) as u8;
        // remove them from the original value
        value >>= 7;

        // set continuation bit
        if !res.is_empty() {
            tmp |= 0x80;
        }

        res.push(tmp);
    }

    // order is wrong due to the way we pushed the data onto it
    res.reverse();
    res
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, &'static str> {
    let mut res = vec![];
    let mut tmp = 0;
    let mut completed = false;
    for b in bytes {
        // test if first 7 bit are set, to check for overflow
        if (tmp & 0xfe_00_00_00) > 0 {
            return Err("Would overflow");
        }

        // append bytes of b to tmp
        tmp = (tmp << 7) | (b & 0x7f) as u32;

        if 0x80 & b == 0 {
            // continuation bit not set, number is complete
            res.push(tmp);
            tmp = 0;
            completed = true;
        }
    }

    // check for incomplete bytes
    if !completed {
        return Err("Incomplete byte sequence");
    }

    Ok(res)
}
