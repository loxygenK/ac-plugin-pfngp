use crate::notify::notify;
use gpgme::{Context, Key, Protocol};

fn find_key(ctx: &mut Context, query: &str) -> Result<Key, String> {
    Ok(ctx
        .find_keys(vec![query])
        .map_err(|x| format!("Could not search keys ({:?})", x))?
        .next()
        .ok_or("No key found for this query".to_string())?
        .map_err(|x| format!("Could not get keys ({:?})", x))?)
}

pub fn encrypt(raw_data: &str) -> Option<String> {
    // Get footer
    let footer = &raw_data[raw_data.rfind('/').expect("Footer separator ('/')")..];
    let recipients = footer.split(' ').skip(1);

    // Get communicated with OpenGPG
    let mut ctx = Context::from_protocol(Protocol::OpenPgp)
        .map_err(|x| {
            notify(
                "Error",
                &format!("Failed to communicate with Opengpg:\n{:?}", x),
            )
        })
        .ok()?;

    // We need armored data
    ctx.set_armor(true);

    // Search appropriate keys the user are going to use
    let target_keys: Vec<Key> = recipients
        .map(|r| find_key(&mut ctx, r)) // Try to find key...
        .filter(|x| x.is_ok()) // Get only succeeded ones...
        .map(|x| x.unwrap()) // Then unwrap it...
        .collect(); // And make them vector.

    // Getting texte of the user id of keys
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

    // Finally encrypting
    let mut output = Vec::new();
    match ctx.encrypt(&target_keys, raw_data, &mut output) {
        Ok(_) => {
            // If succeeded, try to decrypt data (data maybe not plain-text)
            // TODO: Save non-plain text data to somewhere
            match String::from_utf8(output) {
                Ok(d) => {
                    notify(
                        "Encrypted",
                        &format!(
                            "The data you copied just now has been encrypted for:\n{}",
                            target_uids
                        ),
                    );
                    Some(d)
                }
                Err(_) => {
                    notify(
                        "Encrypted (not text)",
                        "The data you copied just now was not text.",
                    );
                    None
                }
            }
        }
        Err(e) => {
            // If failed, tell user to the error and let autoclip NOT to modify clipboard
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
