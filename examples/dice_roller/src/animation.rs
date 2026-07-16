use std::future::poll_fn;
use std::task::Poll;

pub async fn frame_yield() {
    let mut yielded = false;
    poll_fn(|cx| {
        if yielded {
            Poll::Ready(())
        } else {
            yielded = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    })
    .await
}

pub fn mix_entropy(bytes: &mut [u8]) {
    let seed: u32 = bytes.iter().fold(0x9E3779B9u32, |acc, &b| {
        acc.wrapping_mul(31).wrapping_add(b as u32)
    });
    for (i, b) in bytes.iter_mut().enumerate() {
        let salt = seed.wrapping_add(i as u32).wrapping_mul(0x85EBCA6B);
        *b = b.wrapping_add((salt >> 16) as u8);
    }
}
