pub fn extractxml(message: &str) -> &str {
    let startpos = message.find('<').unwrap_or(0);
    let endpos = message.rfind('>').unwrap_or(0);
    &message[startpos..=endpos]
}
