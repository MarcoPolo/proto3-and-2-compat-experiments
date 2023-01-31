fn main() {
    println!("Hello, world!");
}

pub mod p2 {
    include!(concat!(env!("OUT_DIR"), "/p2.rs"));
}

pub mod p3 {
    include!(concat!(env!("OUT_DIR"), "/p3.rs"));
}

#[cfg(test)]
mod test {

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
            r#type: p3::hop_message::Type::Reserve as i32,
            peer: None,
            status: None,
            limit: None,
            reservation: None,
        };

        let b2 = r2.encode_to_vec();
        let b3 = r3.encode_to_vec();

        assert_ne!(b2, b3);
        assert_eq!(&b3[..], b"");

        let b2to3 = p3::HopMessage::decode(bytes::Bytes::from(b2)).expect("should be decodable");
        assert_eq!(b2to3, r3);
        let b3to2 = p2::HopMessage::decode(bytes::Bytes::from(b3)).expect("should be decodable");
        assert_eq!(b3to2, r2);
    }

    #[test]
    fn test_hop_no_status() {
        let r2 = p2::HopMessage {
            status: None,
            ..Default::default()
        };
        let r3 = p3::HopMessage {
            status: None,
            ..Default::default()
        };

        let b2 = r2.encode_to_vec();
        let b3 = r3.encode_to_vec();
        assert_ne!(b2, b3);
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
            r#type: p3::hop_message::Type::Connect as i32,
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
            r#type: p3::hop_message::Type::Connect as i32,
            ..Default::default()
        };

        let b2 = r2.encode_to_vec();
        let b3 = r3.encode_to_vec();
        assert_eq!(b2, b3);
    }

    #[test]
    fn test_proto3_default_status_serialized_can_decode() {
        let r3 = p3::HopMessage {
            r#type: p3::hop_message::Type::Connect as i32,
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
}
