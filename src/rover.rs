use async_std::io::prelude::*;
use async_std::net::TcpStream;

pub struct Rover {
    stream: TcpStream,
}

impl Rover {
    pub async fn connect(
        addr: impl async_std::net::ToSocketAddrs,
    ) -> Result<Self, async_std::io::Error> {
        Ok(Self {
            stream: TcpStream::connect(addr).await?,
        })
    }

    pub async fn recv(&mut self) -> anyhow::Result<crate::net::Event> {
        let mut len = [0; 4];
        self.stream.peek(&mut len).await?;
        let len = u32::from_be_bytes(len) as usize;
        let mut data = vec![0; 4 + len];
        self.stream.read_exact(&mut data).await?;
        Ok(bincode::deserialize(&data[4..])?)
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        loop {
            let event = self.recv().await?;

            println!("Received event '{:?}'", event);
        }
    }
}
