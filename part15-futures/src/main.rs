fn main() {
    const NUM_MASSAGES: usize = 1000;

    let receiver = futures::executor::block_on(async {
        async_std::net::UdpSocket::bind("127.0.0.1:0")
            .await
            .unwrap()
    });

    let sender = futures::executor::block_on(async {
        async_std::net::UdpSocket::bind("127.0.0.1:0")
            .await
            .unwrap()
    });

    let sender_future = async {
        for _ in 0..NUM_MASSAGES {
            let _ = sender
                .send_to(b" Hello, World", receiver.local_addr().unwrap())
                .await
                .unwrap();
            futures_timer::Delay::new(std::time::Duration::from_millis(1)).await;
        }
    };

    let receiver_future = async {
        let mut buf = [0; 256];
        let mut count = 0;

        for _ in 0..NUM_MASSAGES {
            let _ = receiver.recv_from(&mut buf).await.unwrap();
            count += 1;
            println!("msg no {} = {}", count, String::from_utf8_lossy(&buf));
        }
    };

    futures::executor::block_on(async {
        futures::join!(sender_future, receiver_future);
    });
}
