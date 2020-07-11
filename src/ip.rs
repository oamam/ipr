/// Returns a string of the ip address converted to 32 bits.
///
/// # Example
/// ```
/// match get_bit("123.123.123.123") {
///     Ok(bit) => println!("your ip address bit is [{}].", bit),
///     Err(_) => println!("your input is invalid.")
/// }
/// ```
pub fn get_bit(ip: impl Into<String>) -> Result<String, String> {
    let ip = ip.into();
    let fields: Vec<&str> = ip.split(".").collect();
    if fields.len() > 4 {
        return Err(String::from("fileds length of ip is larger than 4"));
    }
    let mut bit: u32 = 0;
    for field in fields {
        let field = match field.parse::<u8>() {
            Ok(field) => field,
            Err(_) => return Err(String::from("failed parse")),
        };
        bit = bit << 8 ^ field as u32;
    }
    Ok(format!("{:0>32b}", bit))
}

/// Returns the subnet mask of the prefix.
///
/// # Example
/// ```
/// match get_subnet_mask(16) {
///     Ok(subnet) => println!("your subnet mask is [{}].", subnet),
///     Err(_) => println!("your input is invalid.")
/// }
/// ```
pub fn get_subnet_mask(prefix: u8) -> Result<String, String> {
    if prefix > 32 {
        return Err(String::from("mask_bit must be less than or equal to 32"));
    }
    let mask_bit = format!("{:0<32}", "1".repeat(prefix as usize));
    make_address(mask_bit)
}

/// Returns the network address of the ip address and prefix.
///
/// # Example
/// ```
/// match get_network_address("123.123.123.123", 16) {
///     Ok(address) => println!("your network address is [{}].", address),
///     Err(_) => println!("your input is invalid.")
/// }
/// ```
pub fn get_network_address(ip: impl Into<String>, prefix: u8) -> Result<String, String> {
    if prefix > 32 {
        return Err(String::from("mask_bit must be less than or equal to 32"));
    }
    let ip_bit = match get_bit(ip) {
        Ok(bit) => bit,
        Err(e) => return Err(e),
    };
    let network_bit = &ip_bit[..prefix as usize];
    let mask_bit = format!("{:0<32}", network_bit);
    make_address(mask_bit)
}

/// Returns the broadcast address of the ip address and prefix.
///
/// # Example
/// ```
/// match get_broadcast_address("123.123.123.123", 16) {
///     Ok(address) => println!("your broadcast address is [{}].", address),
///     Err(_) => println!("your input is invalid.")
/// }
/// ```
pub fn get_broadcast_address(ip: impl Into<String>, prefix: u8) -> Result<String, String> {
    if prefix > 32 {
        return Err(String::from("mask_bit must be less than or equal to 32"));
    }
    let ip_bit = match get_bit(ip) {
        Ok(bit) => bit,
        Err(e) => return Err(e),
    };
    let network_bit = &ip_bit[..prefix as usize];
    let mask_bit = format!("{:1<32}", network_bit);
    make_address(mask_bit)
}

/// Returns whether the ip address is included in the address space.
///
/// # Example
/// ```
/// match check_subnet("123.123.123.123", "123.123.0.0/16") {
///     Ok(checked) => println!("Is your ip address included in the specified address space? [{}].", checked),
///     Err(_) => println!("your input is invalid.")
/// }
/// ```
pub fn check_subnet(ip: impl Into<String>, range: impl Into<String>) -> Result<bool, String> {
    let ip = ip.into();
    let range = range.into();
    let v: Vec<&str> = range.split("/").collect();
    if v.len() != 2 {
        return Err(String::from(""));
    };
    let ip_bit = match get_bit(&ip) {
        Ok(ip_bit) => ip_bit,
        Err(e) => return Err(e),
    };
    let cidr_ip = match get_bit(v[0]) {
        Ok(ip_bit) => ip_bit,
        Err(e) => return Err(e),
    };
    let cidr: u8 = match v[1].parse() {
        Ok(cidr) => cidr,
        Err(e) => return Err(e.to_string()),
    };
    if 0 >= cidr || cidr > 32 {
        return Err(String::from(
            "cidr must be more than 0 and less than or equal to 32",
        ));
    }
    let ip_bit_num = match u64::from_str_radix(&ip_bit, 2) {
        Ok(ip_bit_num) => ip_bit_num,
        Err(e) => return Err(e.to_string()),
    };
    let cidr_bit_num = match u64::from_str_radix(&cidr_ip, 2) {
        Ok(cidr_bit_num) => cidr_bit_num,
        Err(e) => return Err(e.to_string()),
    };
    Ok((ip_bit_num >> (32 - cidr)) ^ (cidr_bit_num >> (32 - cidr)) == 0)
}

fn make_address(bit: impl Into<String>) -> Result<String, String> {
    let bit = bit.into();
    let mut v: Vec<String> = Vec::new();
    for i in (0..25).step_by(8) {
        let field = match usize::from_str_radix(&bit[i..(i + 8)], 2) {
            Ok(field) => field,
            Err(e) => return Err(e.to_string()),
        };
        v.push(field.to_string());
    }
    Ok(v.join("."))
}
