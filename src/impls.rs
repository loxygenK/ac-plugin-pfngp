use gpgme::{Context, Key, Protocol};

pub fn encrypt(raw_data: &str) -> Option<String> {
    // Get footer
    let footer = &raw_data[raw_data.rfind('/').expect("Footer separator ('/')")..];
    println!("Footer: {}", footer);

    let recipients = footer.split(' ').skip(1);

    println!("Trying to communicate with OpenGPG.");
    let mut ctx = Context::from_protocol(Protocol::OpenPgp).expect("OpenPgp Protocolq");
    ctx.set_armor(true);

    let mut target_keys: Vec<Key> = vec![];
    for rec in recipients {
        let keys = ctx.find_keys(vec![rec]);
        if keys.is_err() {
            continue;
        }
        let key = keys.unwrap().next();
        if key.is_none() {
            continue;
        }
        target_keys.append(&mut vec![key.unwrap().unwrap()]);
    }

    let mut output = Vec::new();
    match ctx.encrypt(&target_keys, raw_data, &mut output) {
        Ok(_) => Some(String::from_utf8(output).unwrap()),
        Err(e) => {
            println!("{:?}", e);
            None
        }
    }
}

pub fn decrypt(raw_data: &str) -> Option<String> {
    let mut ctx = Context::from_protocol(Protocol::OpenPgp).ok()?;
    let mut output = Vec::new();
    match ctx.decrypt(raw_data.as_bytes(), &mut output) {
        Ok(_) => Some(String::from_utf8(output).unwrap()),
        Err(e) => {
            println!("{:?}", e);
            None
        }
    }
}
