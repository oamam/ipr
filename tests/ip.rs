#[cfg(test)]
mod tests {
    use ipr::ip::*;

    #[test]
    fn test_normal_get_bit() {
        let data: Vec<Vec<&str>> = vec![
            vec!["123.123.123.123", "01111011011110110111101101111011"],
            vec!["172.22.24.186", "10101100000101100001100010111010"],
            vec!["1.1.1.1", "00000001000000010000000100000001"],
            vec!["0.0.0.0", "00000000000000000000000000000000"],
            vec!["255.255.255.255", "11111111111111111111111111111111"],
        ];
        for d in &data {
            let result = get_bit(d[0]);
            match result {
                Ok(recived) => assert_eq!(recived, d[1]),
                Err(e) => panic!(e),
            };
        }
    }

    #[test]
    fn test_abnormal_get_bit() {
        let data: Vec<Vec<&str>> = vec![
            vec!["123.123.123.123.123", "fileds length is larger than 4"],
            vec!["123.123.123.256", "failed parse"],
        ];
        for d in &data {
            let result = get_bit(d[0]);
            match result {
                Ok(_) => panic!("this response must be error"),
                Err(e) => assert_eq!(e, d[1]),
            };
        }
    }

    #[test]
    fn test_normal_get_subnet_mask() {
        let ips = [
            "0.0.0.0",
            "128.0.0.0",
            "192.0.0.0",
            "224.0.0.0",
            "240.0.0.0",
            "248.0.0.0",
            "252.0.0.0",
            "254.0.0.0",
            "255.0.0.0",
            "255.128.0.0",
            "255.192.0.0",
            "255.224.0.0",
            "255.240.0.0",
            "255.248.0.0",
            "255.252.0.0",
            "255.254.0.0",
            "255.255.0.0",
            "255.255.128.0",
            "255.255.192.0",
            "255.255.224.0",
            "255.255.240.0",
            "255.255.248.0",
            "255.255.252.0",
            "255.255.254.0",
            "255.255.255.0",
            "255.255.255.128",
            "255.255.255.192",
            "255.255.255.224",
            "255.255.255.240",
            "255.255.255.248",
            "255.255.255.252",
            "255.255.255.254",
            "255.255.255.255",
        ];
        for (i, expected) in ips.iter().enumerate() {
            let result = get_subnet_mask(i as u8);
            match result {
                Ok(recived) => assert_eq!(recived, expected.to_string()),
                Err(e) => panic!(e),
            }
        }
    }

    #[test]
    fn test_abnormal_get_subnet_mask() {
        let result = get_subnet_mask(33);
        match result {
            Ok(_) => panic!("this response must be error"),
            Err(e) => assert_eq!(e, "mask_bit must be less than or equal to 32"),
        }
    }

    #[test]
    fn test_get_network_address() {
        struct TestStruct {
            ip: String,
            mask: u8,
            expected: String,
        };

        impl TestStruct {
            fn new(ip: impl Into<String>, mask: u8, expected: impl Into<String>) -> TestStruct {
                Self {
                    ip: ip.into(),
                    mask,
                    expected: expected.into(),
                }
            }
        }

        let test_structs: Vec<TestStruct> = vec![
            TestStruct::new("192.168.11.1", 0, "0.0.0.0"),
            TestStruct::new("192.168.11.1", 8, "192.0.0.0"),
            TestStruct::new("192.168.11.1", 10, "192.128.0.0"),
            TestStruct::new("192.168.11.1", 16, "192.168.0.0"),
            TestStruct::new("192.168.11.1", 20, "192.168.0.0"),
            TestStruct::new("192.168.11.1", 21, "192.168.8.0"),
            TestStruct::new("192.168.11.1", 22, "192.168.8.0"),
            TestStruct::new("192.168.11.1", 23, "192.168.10.0"),
            TestStruct::new("192.168.11.1", 24, "192.168.11.0"),
            TestStruct::new("192.168.11.1", 32, "192.168.11.1"),
        ];
        for ts in test_structs {
            let result = get_network_address(ts.ip, ts.mask);
            match result {
                Ok(recived) => assert_eq!(recived, ts.expected),
                Err(e) => panic!(e),
            };
        }
    }
    #[test]
    fn test_get_broadcast_address() {
        struct TestStruct {
            ip: String,
            mask: u8,
            expected: String,
        };

        impl TestStruct {
            fn new(ip: impl Into<String>, mask: u8, expected: impl Into<String>) -> TestStruct {
                Self {
                    ip: ip.into(),
                    mask,
                    expected: expected.into(),
                }
            }
        }

        let test_structs: Vec<TestStruct> = vec![
            TestStruct::new("192.168.11.1", 0, "255.255.255.255"),
            TestStruct::new("192.168.11.1", 8, "192.255.255.255"),
            TestStruct::new("192.168.11.1", 10, "192.191.255.255"),
            TestStruct::new("192.168.11.1", 16, "192.168.255.255"),
            TestStruct::new("192.168.11.1", 20, "192.168.15.255"),
            TestStruct::new("192.168.11.1", 21, "192.168.15.255"),
            TestStruct::new("192.168.11.1", 22, "192.168.11.255"),
            TestStruct::new("192.168.11.1", 23, "192.168.11.255"),
            TestStruct::new("192.168.11.1", 24, "192.168.11.255"),
            TestStruct::new("192.168.11.1", 32, "192.168.11.1"),
        ];
        for ts in test_structs {
            match get_broadcast_address(ts.ip, ts.mask) {
                Ok(recived) => assert_eq!(recived, ts.expected),
                Err(e) => panic!(e),
            };
        }
    }

    #[test]
    fn test_normal_check_subnet() {
        struct TestStruct {
            ip: String,
            range: String,
            expected: bool,
        }
        impl TestStruct {
            fn new(ip: impl Into<String>, range: impl Into<String>, expected: bool) -> TestStruct {
                Self {
                    ip: ip.into(),
                    range: range.into(),
                    expected,
                }
            }
        }
        let test_structs: Vec<TestStruct> = vec![
            TestStruct::new("192.168.11.1", "192.168.11.1/32", true),
            TestStruct::new("192.168.11.0", "192.168.11.1/31", true),
            TestStruct::new("192.168.11.1", "192.168.11.1/31", true),
            TestStruct::new("192.168.11.2", "192.168.11.1/31", false),
            TestStruct::new("192.168.11.1", "192.168.11.1/24", true),
            TestStruct::new("192.168.11.255", "192.168.11.1/24", true),
            TestStruct::new("192.168.12.0", "192.168.11.1/24", false),
            TestStruct::new("192.167.255.255", "192.168.11.1/16", false),
            TestStruct::new("192.168.0.0", "192.168.11.1/16", true),
            TestStruct::new("192.168.255.255", "192.168.11.1/16", true),
            TestStruct::new("192.169.0.0", "192.168.11.1/16", false),
            TestStruct::new("191.255.255.255", "192.168.11.1/8", false),
            TestStruct::new("192.0.0.0", "192.168.11.1/8", true),
            TestStruct::new("192.255.255.255", "192.168.11.1/8", true),
            TestStruct::new("193.0.0.0", "192.168.11.1/8", false),
            TestStruct::new("127.255.255.255", "192.168.11.1/1", false),
            TestStruct::new("128.0.0.0", "192.168.11.1/1", true),
            TestStruct::new("128.255.255.255", "192.168.11.1/1", true),
            TestStruct::new("255.255.255.255", "192.168.11.1/1", true),
        ];

        for ts in test_structs {
            match check_subnet(&ts.ip, &ts.range) {
                Ok(recived) => {
                    assert_eq!(recived, ts.expected, "ip: {}, range: {}", ts.ip, ts.range)
                }
                Err(e) => panic!(e),
            };
        }
    }

    #[test]
    fn test_abnormal_check_subnet() {
        match check_subnet("123.123.123.123", "123.123.123.123/33") {
            Ok(_) => panic!("the response must be error"),
            Err(e) => assert_eq!(e, "cidr must be more than 0 and less than or equal to 32"),
        };
    }
}
