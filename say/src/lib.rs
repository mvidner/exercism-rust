use std::collections::LinkedList;

static SMALL: [&'static str; 20] =
    ["zero", "one", "two", "three", "four",
     "five", "six", "seven", "eight", "nine",
     "ten", "eleven", "twelve", "thirteen", "fourteen",
     "fifteen", "sixteen", "seventeen", "eighteen", "nineteen"];

static TENS: [&'static str; 10] =
    ["ZERTY", "ONETY", "twenty", "thirty", "forty",
     "fifty", "sixty", "seventy", "eighty", "ninety"];

static ORDERS: [&'static str; 7] =
    ["ONES", "thousand", "million", "billion", "trillion",
     "quadrillion", "quintillion"]; // a sextillion is larger than u64

pub fn encode(n: u64) -> String {
    let mut parts = LinkedList::new();

    let mut order = ORDERS.into_iter();
    order.next();               // throw away the "ONES"
    let mut small: u16 = (n % 1000) as u16;
    let mut big = n / 1000;
    if small > 0 || big == 0 {
        parts.push_front(encode_1000(small));
    }

    while big > 0 {
        let order_s = order.next().unwrap();
        small = (big % 1000) as u16;
        if small > 0 {
            parts.push_front(order_s.to_string());
            parts.push_front(encode_1000(small));
        }
        big /= 1000;
    }
    parts.into_iter().collect::<Vec<_>>().join(" ")
}

fn encode_1000(n: u16) -> String {
    match n {
        0 ... 19 => SMALL[n as usize].to_string(),
        20 ... 99 => connected(encode_tens(n), "-", n % 10),
        100 ... 999 => connected(encode_hundreds(n), " ", n % 100), // not " and "
        _ => "too many".to_string(),
    }
}

fn connected(big: String, connector: &str, small: u16) -> String {
    match small {
        0 => big,
        _ => big + connector + &encode_1000(small)
    }
}

fn encode_tens(n: u16) -> String {
    TENS[(n / 10) as usize].to_string()
}

fn encode_hundreds(n: u16) -> String {
    encode_1000(n / 100) + " hundred"
}
