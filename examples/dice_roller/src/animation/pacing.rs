use std::future::poll_fn;
use std::task::Poll;

pub async fn advance() {
    let mut done = false;
    poll_fn(|cx| {
        if done {
            Poll::Ready(())
        } else {
            done = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    })
    .await
}

pub async fn drain() {
    for _ in 0..2 {
        let mut done = false;
        poll_fn(|cx| {
            if done {
                Poll::Ready(())
            } else {
                done = true;
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        })
        .await;
    }
}

pub async fn let_ui_settle() {
    #[cfg(target_arch = "wasm32")]
    {
        use futures::channel::oneshot;
        let (tx, rx) = oneshot::channel::<()>();
        wasm_bindgen_futures::spawn_local(async move {
            let _ = tx.send(());
        });
        let _ = rx.await;
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        advance().await;
    }
}
