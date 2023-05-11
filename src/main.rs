mod handler;
mod channel;

use channel::Channel;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut channel = Channel::connect().await?;

    // get peer id
    // register status as listener
    // get all producers
    // register consumers for all producers
    // register webrtc connection listener for each consumer?
    // start consuming using webrtcsrc
    // make a grid
    // record

    // let list_producers_message = json!({
    //     "type": "list"
    // });

    // socket.write_message(Message::Text(list_producers_message.to_string())).unwrap();
    // let msg = socket.read_message().expect("Error reading message");
    // println!("Received: {}", msg);

    // socket.close(None)?;
    // channel.close().await?;

    println!("here");
    channel.send();

    Ok(())
}
