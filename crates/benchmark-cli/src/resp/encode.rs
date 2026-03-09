use bytes::BytesMut;
use protocol::encoder::Encoder;
use protocol::types::{BulkData, RespFrame};

#[derive(Clone, Debug)]
pub enum ExpectedResponse {
    Simple(&'static str),
    Bulk(Option<Vec<u8>>),
    Array(Vec<ExpectedResponse>),
}

pub fn encode_resp_parts(parts: &[&[u8]]) -> Vec<u8> {
    let mut encoder = Encoder::default();
    let mut buf =
        BytesMut::with_capacity(parts.iter().map(|part| part.len() + 16).sum::<usize>() + 16);
    let frame = RespFrame::Array(Some(
        parts
            .iter()
            .map(|part| RespFrame::Bulk(Some(BulkData::from_vec(part.to_vec()))))
            .collect(),
    ));
    encoder.encode(&frame, &mut buf);
    buf.to_vec()
}

pub fn encode_expected_response(expected: &ExpectedResponse) -> Option<Vec<u8>> {
    let mut encoder = Encoder::default();
    let mut buf = BytesMut::new();
    let frame = expected_to_frame(expected)?;
    encoder.encode(&frame, &mut buf);
    Some(buf.to_vec())
}

fn expected_to_frame(expected: &ExpectedResponse) -> Option<RespFrame> {
    Some(match expected {
        ExpectedResponse::Simple(value) => RespFrame::SimpleStatic(value),
        ExpectedResponse::Bulk(None) => RespFrame::Bulk(None),
        ExpectedResponse::Bulk(Some(value)) => {
            RespFrame::Bulk(Some(BulkData::from_vec(value.clone())))
        }
        ExpectedResponse::Array(items) => RespFrame::Array(Some(
            items
                .iter()
                .map(expected_to_frame)
                .collect::<Option<Vec<_>>>()?,
        )),
    })
}
