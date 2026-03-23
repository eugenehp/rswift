use apple_platforms::platform::{ApplePlatform, Platform};

pub fn main() {
    let _ = ApplePlatform::into_iter()
        .map(|apple_platform| {
            let platform = Platform::try_from(apple_platform).unwrap();
            println!("{:?}", platform);
        })
        .collect::<Vec<_>>();
}
