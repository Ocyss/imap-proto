use nom::{bytes::streaming::tag_no_case, sequence::tuple, IResult};

use crate::{
    parser::core::{number_64, paren_delimited},
    types::*,
};

// Extends status-att/status-att-list defined in rfc8438
// [RFC8438](https://tools.ietf.org/html/rfc8438)
#[cfg(feature = "rfc-8438")]
pub(crate) fn status_att_val_size(i: &[u8]) -> IResult<&[u8], StatusAttribute> {
    let (i, (_, num)) = tuple((tag_no_case("SIZE "), number_64))(i)?;
    Ok((i, StatusAttribute::Size(num)))
}
