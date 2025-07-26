use crate::settings::ProtoTypes;

pub(crate) fn parse_body(source: &String) -> Vec<(ProtoTypes, String)> {
    let mut index = 0;
    let mut result: Vec<(ProtoTypes, String)> = Vec::new();

    loop {
        let opt_tuple = parse_string_find_next(source, index);
        if opt_tuple.is_some() {
            let (proto, url, i) = opt_tuple.unwrap();
            result.push((proto, url));
            index = i;
        } else {
            return result;
        }
    }
}

/* Start searching in source string from index
 * Returns found server data (proto + url) and index to the lasting part of string
 * Returns None if no data found or index is invalid */
fn parse_string_find_next(source: &str, index: usize) -> Option<(ProtoTypes, String, usize)> {
    let opening = "<a href=\"";
    let closing = "</a>";

    let opt_start = source.get(index..);
    if opt_start.is_none() {
        return None;
    }
    let sub_source = opt_start.unwrap();

    let oi = sub_source.find(opening);
    if oi.is_none() {
        return None;
    }
    let i1 = oi.unwrap() + opening.len();

    let oi = sub_source.get(i1..).unwrap().find("\"");
    if oi.is_none() {
        return None;
    }
    let i2 = i1 + oi.unwrap();

    let url = &sub_source[i1..i2];

    let oi = sub_source.get(i2..).unwrap().find(">");
    if oi.is_none() {
        return None;
    }
    let i3 = i2 + oi.unwrap() + 1;

    let oi = sub_source.get(i3..).unwrap().find(closing);
    if oi.is_none() {
        return None;
    }
    let i4: usize = i3 + oi.unwrap();

    let tag = &sub_source[i3..i4];
    let end_index = i4 + closing.len() + index;

    if tag.to_ascii_uppercase().contains("UDP") {
        return Some((ProtoTypes::UDP, url.to_string(), end_index));
    }

    if tag.to_ascii_uppercase().contains("TCP") {
        return Some((ProtoTypes::TCP, url.to_string(), end_index));
    }

    Some((ProtoTypes::Unknown, url.to_string(), end_index))
}