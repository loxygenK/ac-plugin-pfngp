use crate::notify::notify;
use gpgme::{Context, Key, Protocol};
use notify_rust::Notification;

pub fn encrypt(raw_data: &str) -> Option<String> {
    let footer = &raw_data[raw_data.rfind('/').expect("Footer separator ('/')")..];
    let recipients = footer.split(' ').skip(1);

    let mut ctx = Context::from_protocol(Protocol::OpenPgp)
        .map_err(|x| {
            notify(
                "Error",
                &format!("Failed to communicate with Opengpg:\n{:?}", x),
            )
        })
        .ok()?;

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

    let target_uids: String = target_keys
        .iter()
        .map(|x| {
            x.user_ids()
                .next()
                .map_or("This key has no user id".to_string(), |x| {
                    format!(
                        "{}[{}] ({})",
                        x.name().unwrap_or("Unreadable Name"),
                        x.email().unwrap_or("Unreadable Email"),
                        x.comment().unwrap_or("Unreadable comment")
                    )
                })
        })
        .collect::<Vec<String>>()
        .join("\n");

    let mut output = Vec::new();
    match ctx.encrypt(&target_keys, raw_data, &mut output) {
        Ok(_) => {
            notify(
                "Encrypted",
                &format!(
                    "The data you copied just now has been encrypted for:\n{}",
                    target_uids
                ),
            );
            Some(String::from_utf8(output).unwrap())
        }
        Err(e) => {
            notify(
                "Failed to encrypt",
                &format!(
                    "The data you copied now could not be encrypted. Here's the reason:\n{:?}",
                    e
                ),
            );
            None
        }
    }
}

pub fn decrypt(raw_data: &str) -> Option<String> {
    let mut ctx = Context::from_protocol(Protocol::OpenPgp).ok()?;
    let mut output = Vec::new();
    notify(
        "Decryption in progress",
        &format!(
            "{}\n{}",
            "You'll be asked for your GPG Private Key's passphrase. (Maybe not)",
            "It will not be stored and used for only decryption, so no worry!"
        ),
    );
    match ctx.decrypt(raw_data.as_bytes(), &mut output) {
        Ok(_) => {
            notify("Decrypted", "The data you copied has been decrypted.");
            Some(String::from_utf8(output).unwrap())
        }
        Err(e) => {
            notify(
                "Failed to decrypt",
                &format!("The data you copied could not be decrypted.\n{:?}", e),
            );
            None
        }
    }
}
