pub(super) fn decode_html_entities(value: &str) -> String {
    let mut output = String::with_capacity(value.len());
    let mut remaining = value;
    while let Some(start) = remaining.find('&') {
        output.push_str(&remaining[..start]);
        let entity = &remaining[start + 1..];
        let Some(end) = entity.find(';').filter(|end| *end <= 10) else {
            output.push('&');
            remaining = entity;
            continue;
        };
        let Some(character) = decode_entity(&entity[..end]) else {
            output.push('&');
            remaining = entity;
            continue;
        };
        output.push(character);
        remaining = &entity[end + 1..];
    }
    output.push_str(remaining);
    output
}

fn decode_entity(entity: &str) -> Option<char> {
    match entity {
        "amp" => Some('&'),
        "apos" => Some('\''),
        "gt" => Some('>'),
        "lt" => Some('<'),
        "nbsp" => Some(' '),
        "quot" => Some('"'),
        value => decode_numeric(value),
    }
}

fn decode_numeric(value: &str) -> Option<char> {
    let number = value.strip_prefix('#')?;
    let codepoint = match number.strip_prefix(['x', 'X']) {
        Some(hexadecimal) => u32::from_str_radix(hexadecimal, 16).ok()?,
        None => number.parse().ok()?,
    };
    char::from_u32(codepoint)
}
