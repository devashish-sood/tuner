use cpal::traits::StreamTrait;
mod stream;
mod detection;
mod fsvec;

fn main() {
    let stream: cpal::Stream = stream::build_stream();
    stream.play().expect("stream couldn't play?");
    loop { std::thread::park(); }
}
