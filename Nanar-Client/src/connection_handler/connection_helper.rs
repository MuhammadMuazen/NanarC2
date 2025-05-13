pub fn ip_to_u8_array(ip_str: &str) -> Option<[u8; 4]> {
    
    let (a, rest) = ip_str.split_once('.')?;
    let (b, rest) = rest.split_once('.')?;
    let (c, d) = rest.split_once('.')?;
    
    Some([
        a.parse().ok()?,
        b.parse().ok()?,
        c.parse().ok()?,
        d.parse().ok()?,
    ])
}