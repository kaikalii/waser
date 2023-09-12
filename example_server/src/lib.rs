fn respond(input: &[u8]) -> Vec<u8> {
    String::from_utf8_lossy(input)
        .chars()
        .rev()
        .collect::<String>()
        .into_bytes()
}

waser::server!(respond);
