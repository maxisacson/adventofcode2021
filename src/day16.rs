use crate::utils::{day, close, part, read_lines_to_vec, answer};

pub fn run() {
    day(16);
    part1();
    part2();
    close();
}

fn part1() {
    part(1);

    let input = hex2binary(&read_lines_to_vec("data/day16.txt")[0]);

    let (_, packet) = decode_packet(&input);
    let result = sum_version(&packet.unwrap());

    assert_eq!(result, 1038);
    answer(result);
}

fn part2() {
    part(2);

    let input = hex2binary(&read_lines_to_vec("data/day16.txt")[0]);

    let (_, packet) = decode_packet(&input);
    let result = packet.unwrap().eval();

    assert_eq!(result, 246761930504);
    answer(result);
}

fn decode_packet(data: &str) -> (usize, Option<Packet>) {
    if data.len() == 0 { return (0, None); }

    let version = bin2int(&data[..3]);
    let id = bin2int(&data[3..6]);
    let mut bit = 6;

    let mut packet = Packet{version, id, value: 0, sub_packets: Vec::new()};
    match id {
        4 => {
            // decode literal
            let (value, count) = decode_literal(&data[6..]);
            packet.value = value as i64;
            bit += count;
        }
        _ => {
            // decode operator package
            let length_type_id = bin2int(&data[bit..bit+1]);
            bit += 1;
            match length_type_id {
                0 => {
                    let bit_count = bin2int(&data[bit..bit+15]);
                    bit += 15;
                    let subdata = &data[bit..];
                    let mut bits_read = 0;
                    while bits_read < bit_count {
                        let (count, tmp) = decode_packet(&subdata[bits_read..]);
                        bits_read += count;
                        if let Some(pack) = tmp {
                            packet.sub_packets.push(pack);
                        }
                    }
                    bit += bits_read;
                },
                1 => {
                    let packet_count = bin2int(&data[bit..bit+11]);
                    bit += 11;
                    let subdata = &data[bit..];
                    let mut bits_read = 0;
                    for _ in 0..packet_count {
                        let (count, tmp) = decode_packet(&subdata[bits_read..]);
                        bits_read += count;
                        if let Some(pack) = tmp {
                            packet.sub_packets.push(pack);
                        }
                    }
                    bit += bits_read;
                },
                _ => { }
            }
        }
    }

    (bit, Some(packet))
}

struct Packet {
    version: usize,
    id: usize,
    value: i64,
    sub_packets: Vec<Packet>,
}

impl Packet {
    fn eval(&mut self) -> i64 {
        match self.id {
            0 => {
                self.value = self.sub_packets.iter_mut().fold(0, |a, p| a + p.eval());
            },
            1 => {
                self.value = self.sub_packets.iter_mut().fold(1, |a, p| a * p.eval());
            },
            2 => {
                self.value = self.sub_packets.iter_mut().fold(i64::MAX, |min, p| {
                    let val = p.eval();
                    if val < min { val } else { min }
                });
            },
            3 => {
                self.value = self.sub_packets.iter_mut().fold(i64::MIN, |max, p| {
                    let val = p.eval();
                    if val > max { val } else { max }
                });
            },
            5 => {
                assert_eq!(self.sub_packets.len(), 2);
                let val1 = self.sub_packets[0].eval();
                let val2 = self.sub_packets[1].eval();
                self.value = (val1 > val2) as i64;
            },
            6 => {
                assert_eq!(self.sub_packets.len(), 2);
                let val1 = self.sub_packets[0].eval();
                let val2 = self.sub_packets[1].eval();
                self.value = (val1 < val2) as i64;
            },
            7 => {
                assert_eq!(self.sub_packets.len(), 2);
                let val1 = self.sub_packets[0].eval();
                let val2 = self.sub_packets[1].eval();
                self.value = (val1 == val2) as i64;
            }
            _ => {}
        }
        self.value
    }
}

fn decode_literal(packet: &str) -> (usize, usize) {
    let mut i = 0;
    let mut value = String::new();
    loop {
        let prefix = bin2int(&packet[i..i+1]);
        let val = &packet[i+1..i+5];
        value.push_str(&val);
        i += 5;
        if prefix == 0 { break }
    }

    (bin2int(&value), i)
}

fn hex2binary(hex: &str) -> String {
    String::from_iter(hex.chars().map(|c| format!("{:04b}", c.to_digit(16).unwrap())))
}

fn bin2int(bin: &str) -> usize {
    usize::from_str_radix(bin, 2).unwrap()
}

fn sum_version(packet: &Packet) -> usize {
    let mut sum = packet.version;
    for pack in &packet.sub_packets {
        sum += sum_version(&pack);
    }
    return sum;
}

