use std::{
    fs::File,
    io::{Read, Write},
    vec,
};

use prost::Message;
use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};

const HOP_FILE: &str = "../randomHopMsgs.bin";
const STOP_FILE: &str = "../randomStopMsgs.bin";

fn verify_hop_msgs() -> std::io::Result<()> {
    // Hop messages
    let mut f = File::open(HOP_FILE)?;

    // read the message
    let mut data: Vec<u8> = vec![];
    f.read_to_end(&mut data)?;

    let mut slice = data.as_slice();

    while slice.len() > 0 {
        // Using &slice[0..] to not advance the slice
        let result_from_3 = p3::HopMessage::decode_length_delimited(&slice[0..])?;
        let result_from_2 = p2::HopMessage::decode_length_delimited(&mut slice)?;
        let mismatch_err = Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!(
                "Type mismatch in Hop message. 2: {:?}, 3: {:?}",
                result_from_2, result_from_3
            ),
        ));

        if Some(result_from_2.r#type) != result_from_3.r#type {
            return mismatch_err;
        }

        if result_from_2.peer.is_some() && result_from_3.peer.is_some() {
            let peer2 = result_from_2.peer.unwrap();
            let peer3 = result_from_3.peer.unwrap();

            if peer2.id != peer3.id.unwrap() {
                return mismatch_err;
            }

            if peer2.addrs != peer3.addrs {
                return mismatch_err;
            }
        } else if result_from_2.peer.is_some() || result_from_3.peer.is_some() {
            return mismatch_err;
        }

        if result_from_2.reservation.is_some() && result_from_3.reservation.is_some() {
            let reservation2 = result_from_2.reservation.unwrap();
            let reservation3 = result_from_3.reservation.unwrap();

            if reservation2.expire != reservation3.expire.unwrap() {
                return mismatch_err;
            }

            if reservation2.addrs != reservation3.addrs {
                return mismatch_err;
            }

            if reservation2.voucher != reservation3.voucher {
                return mismatch_err;
            }
        } else if result_from_2.reservation.is_some() || result_from_3.reservation.is_some() {
            return mismatch_err;
        }

        if result_from_2.limit.is_some() && result_from_3.limit.is_some() {
            let limit2 = result_from_2.limit.unwrap();
            let limit3 = result_from_3.limit.unwrap();

            if limit2.duration != limit3.duration {
                return mismatch_err;
            }

            if limit2.data != limit3.data {
                return mismatch_err;
            }
        } else if result_from_2.limit.is_some() || result_from_3.limit.is_some() {
            return mismatch_err;
        }

        if result_from_2.status != result_from_3.status {
            return mismatch_err;
        }
    }
    Ok(())
}

fn verify_stop_msgs() -> std::io::Result<()> {
    let mut f = File::open(STOP_FILE)?;

    // read the message
    let mut data: Vec<u8> = vec![];
    f.read_to_end(&mut data)?;

    let mut slice = data.as_slice();
    while slice.len() > 0 {
        // Using &slice[0..] to not advance the slice
        let result_from_3 = p3::StopMessage::decode_length_delimited(&slice[0..])?;
        let result_from_2 = p2::StopMessage::decode_length_delimited(&mut slice)?;
        let mismatch_err = Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!(
                "Type mismatch in stop message. 2: {:?}, 3: {:?}",
                result_from_2, result_from_3
            ),
        ));

        if Some(result_from_2.r#type) != result_from_3.r#type {
            return mismatch_err;
        }

        if result_from_2.peer.is_some() && result_from_3.peer.is_some() {
            let peer2 = result_from_2.peer.unwrap();
            let peer3 = result_from_3.peer.unwrap();

            if peer2.id != peer3.id.unwrap() {
                return mismatch_err;
            }

            if peer2.addrs != peer3.addrs {
                return mismatch_err;
            }
        } else if result_from_2.peer.is_some() || result_from_3.peer.is_some() {
            return mismatch_err;
        }

        if result_from_2.limit.is_some() && result_from_3.limit.is_some() {
            let limit2 = result_from_2.limit.unwrap();
            let limit3 = result_from_3.limit.unwrap();

            if limit2.duration != limit3.duration {
                return mismatch_err;
            }

            if limit2.data != limit3.data {
                return mismatch_err;
            }
        } else if result_from_2.limit.is_some() || result_from_3.limit.is_some() {
            return mismatch_err;
        }

        if result_from_2.status != result_from_3.status {
            return mismatch_err;
        }
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let cmd = std::env::args().nth(1).unwrap_or("".to_string());
    if cmd == "verify" {
        verify_hop_msgs()?;
        verify_stop_msgs()?;
        return Ok(());
    }

    let mut file = File::create(HOP_FILE).unwrap();
    // Generate 10_000 random messages
    for seed in 0..10_000 {
        let mut rng = StdRng::seed_from_u64(seed);
        let b = gen_hop_msg(&mut rng).encode_length_delimited_to_vec();
        file.write_all(&b)?;
    }

    let mut file = File::create(STOP_FILE).unwrap();
    for seed in 0..10_000 {
        let mut rng = StdRng::seed_from_u64(seed);
        let b = gen_stop_msg(&mut rng).encode_length_delimited_to_vec();
        file.write_all(&b)?;
    }

    Ok(())
}

pub mod p2 {
    include!(concat!(env!("OUT_DIR"), "/p2.rs"));
}

pub mod p3 {
    include!(concat!(env!("OUT_DIR"), "/p3.rs"));
}

fn gen_addrs(rng: &mut StdRng) -> Vec<Vec<u8>> {
    let mut addrs = vec![];
    for _ in 0..rng.gen_range(0..5) {
        let mut addr = vec![0u8; 32];
        rng.fill(addr.as_mut_slice());
        addrs.push(addr)
    }
    addrs
}

fn gen_peer(rng: &mut StdRng) -> p3::Peer {
    let mut id = vec![0u8; 32];
    let addrs = gen_addrs(rng);
    rng.fill(id.as_mut_slice());

    p3::Peer {
        id: Some(id),
        addrs: addrs,
    }
}

fn gen_reservation(rng: &mut StdRng) -> p3::Reservation {
    let mut voucher: Option<Vec<u8>> = None;
    if rng.gen::<bool>() {
        let mut b = vec![0u8; 32];
        rng.fill(b.as_mut_slice());
        voucher = Some(b);
    }
    p3::Reservation {
        expire: Some(if rng.gen() { rng.gen() } else { 0 }),
        addrs: gen_addrs(rng),
        voucher: voucher,
    }
}

fn gen_limit(rng: &mut StdRng) -> p3::Limit {
    p3::Limit {
        duration: rng.gen(),
        data: rng.gen(),
    }
}

fn gen_status(rng: &mut StdRng) -> p3::Status {
    let statuses = [
        p3::Status::Ok,
        p3::Status::ReservationRefused,
        p3::Status::ResourceLimitExceeded,
        p3::Status::PermissionDenied,
        p3::Status::ConnectionFailed,
        p3::Status::NoReservation,
        p3::Status::MalformedMessage,
        p3::Status::UnexpectedMessage,
    ];
    *statuses.choose(rng).unwrap()
}

fn gen_hop_msg(rng: &mut StdRng) -> p3::HopMessage {
    let typ = p3::hop_message::Type::from_i32(rng.gen_range(0..3)).unwrap();
    p3::HopMessage {
        r#type: Some(typ as i32),
        peer: rng.gen::<bool>().then(|| gen_peer(rng)),
        reservation: rng.gen::<bool>().then(|| gen_reservation(rng)),
        limit: rng.gen::<bool>().then(|| gen_limit(rng)),
        status: rng.gen::<bool>().then(|| gen_status(rng) as i32),
    }
}

fn gen_stop_msg(rng: &mut StdRng) -> p3::StopMessage {
    let typ = p3::stop_message::Type::from_i32(rng.gen_range(0..2)).unwrap();
    p3::StopMessage {
        r#type: Some(typ as i32),
        peer: rng.gen::<bool>().then(|| gen_peer(rng)),
        limit: rng.gen::<bool>().then(|| gen_limit(rng)),
        status: rng.gen::<bool>().then(|| gen_status(rng) as i32),
    }
}

#[cfg(test)]
mod test {

    use crate::{verify_hop_msgs, verify_stop_msgs};

    use super::{p2, p3};
    use prost::Message;

    #[test]
    fn test_hop_reserve() {
        let r2 = p2::HopMessage {
            r#type: p2::hop_message::Type::Reserve as i32,
            peer: None,
            status: None,
            limit: None,
            reservation: None,
        };
        let r3 = p3::HopMessage {
            r#type: Some(p3::hop_message::Type::Reserve as i32),
            peer: None,
            status: None,
            limit: None,
            reservation: None,
        };

        let b2 = r2.encode_to_vec();
        let b3 = r3.encode_to_vec();

        assert_eq!(b2, b3);

        let b2to3 = p3::HopMessage::decode(bytes::Bytes::from(b2)).expect("should be decodable");
        assert_eq!(b2to3, r3);
        let b3to2 = p2::HopMessage::decode(bytes::Bytes::from(b3)).expect("should be decodable");
        assert_eq!(b3to2, r2);
    }

    #[test]
    fn test_hop_no_status() {
        let r2 = p2::HopMessage {
            status: None,
            r#type: p2::hop_message::Type::Reserve as i32,
            ..Default::default()
        };
        let r3 = p3::HopMessage {
            status: None,
            r#type: Some(p3::hop_message::Type::Reserve as i32),
            ..Default::default()
        };

        let b2 = r2.encode_to_vec();
        let b3 = r3.encode_to_vec();
        assert_eq!(b2, b3);
        // decode proto3 message
        let b3to2 = p2::HopMessage::decode(bytes::Bytes::from(b3)).expect("should be decodable");
        assert!(b3to2.status.is_none());
        let b2to3 = p3::HopMessage::decode(bytes::Bytes::from(b2)).expect("should be decodable");
        assert!(b2to3.status.is_none());
    }

    #[test]
    fn test_explicit_values_wire() {
        let r2 = p2::HopMessage {
            r#type: p2::hop_message::Type::Connect as i32,
            // proto2 will serialize the explicitly set enum
            status: Some(p2::Status::Ok as i32),
            ..Default::default()
        };

        let r3 = p3::HopMessage {
            r#type: Some(p3::hop_message::Type::Connect as i32),
            status: Some(p3::Status::Ok as i32),
            ..Default::default()
        };

        let b2 = r2.encode_to_vec();
        let b3 = r3.encode_to_vec();
        assert_eq!(b2, b3);

        let r2 = p2::HopMessage {
            r#type: p2::hop_message::Type::Connect as i32,
            ..Default::default()
        };

        let r3 = p3::HopMessage {
            r#type: Some(p3::hop_message::Type::Connect as i32),
            ..Default::default()
        };

        let b2 = r2.encode_to_vec();
        let b3 = r3.encode_to_vec();
        assert_eq!(b2, b3);
    }

    #[test]
    fn test_proto3_default_status_serialized_can_decode() {
        let r3 = p3::HopMessage {
            r#type: Some(p3::hop_message::Type::Connect as i32),
            status: Some(p3::Status::Unused as i32),
            ..Default::default()
        };

        let b3 = r3.encode_to_vec();
        let b3to2 = p2::HopMessage::decode(bytes::Bytes::from(b3)).expect("should be decodable");
        assert_eq!(b3to2.status, Some(0))
    }

    #[test]
    fn test_decode_empty_buffer() {
        let message = p2::HopMessage::decode(bytes::Bytes::new()).expect("should decode");
        assert_eq!(message.r#type, 0)
    }

    #[test]
    fn test_random_msgs() -> std::io::Result<()> {
        verify_hop_msgs()?;
        verify_stop_msgs()?;
        Ok(())
    }
}
