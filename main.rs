```rust
use rand::Rng;
use std::convert::From;

struct NostrKey {
    pub public_key: [u8; 32],
    pub private_key: [u8; 64],
}

impl From<Rng> for NostrKey {
    fn from(rng: Rng) -> Self {
        let mut public_key = [0; 32];
        let mut private_key = [0; 64];

        for i in 0..32 {
            public_key[i] = rng.gen::<u8>();
        }

        for i in 0..64 {
            private_key[i] = rng.gen::<u8>();
        }

        NostrKey { public_key, private_key }
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let nostr_key = NostrKey::from(rng);

    println!("Public Key: {:?}", nostr_key.public_key);
    println!("Private Key: {:?}", nostr_key.private_key);
}
```
