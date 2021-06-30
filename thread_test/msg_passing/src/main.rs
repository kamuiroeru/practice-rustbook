use std::sync::mpsc;
use std::thread;
use rand::prelude::*;

fn main() {

    let mut handles = Vec::new();
    let mut data = vec![1; 10];
    let mut snd_channels = Vec::new();
    let mut rcv_channels = Vec::new();

    for _ in 0..10 {
        // main -> 各スレッド のチャネル
        let (snd_tx, snd_rx) = mpsc::channel();
        // 各スレッド -> main のチャネル
        let (rcv_tx, rcv_rx) = mpsc::channel();

        snd_channels.push(snd_tx);
        rcv_channels.push(rcv_rx);

        handles.push(thread::spawn(move || {
            let mut data = snd_rx.recv().unwrap();
            data += 1;
            let _ = rcv_tx.send(data);
        }));
    }

    for x in 0..10 {
        let _ = snd_channels[x].send(data[x]);
    }

    for x in 0..10 {
        data[x] = rcv_channels[x].recv().unwrap();
    }

    for handle in handles {
        let _ = handle.join();
    }

    dbg!(data);

    let mut rng = thread_rng();
}