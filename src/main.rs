mod channel;
mod handler;
mod model;

use channel::Channel;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut _channel = Channel::connect().await?;

    // get peer id [x]
    // register status as listener [x]
    // get all producers [x]
    // send start session for all of the consumers too
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

    _channel.join_handle.await?;
    // loop {}

    // channel.close().await?;

    Ok(())
}
