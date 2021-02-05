use tokio::io::{stdin as tokio_stdin, AsyncReadExt};
use tokio::sync::mpsc::UnboundedSender;

pub async fn readstdin(tx: UnboundedSender<Vec<u8>>) {
    let mut stdin = tokio_stdin();

    loop {
        let mut buff = vec![0; 1024];
        let n = match stdin.read(&mut buff).await {
            Err(_) | Ok(0) => break,
            Ok(n) => n,
        };

        buff.truncate(n);
        tx.send(buff).unwrap();
    }
}
