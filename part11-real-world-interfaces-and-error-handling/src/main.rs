use std::io::{BufRead, Write};

fn main() {
    // let path: String = std::env::var("PATH").unwrap_or("".to_string());
    let path = std::env::var("PATH").unwrap();
    println!("PATH is {:?}", path);
    let folders = std::env::split_paths(&path);
    for (i, folder) in folders.enumerate().into_iter() {
        println!("folder {} : {:?}", i + 1, folder);
    }

    // ----------------------------------------

    let args = std::env::args();
    for (i, arg) in args.enumerate().into_iter() {
        std::io::stdout()
            .write_all(format!("arg {} : {:?}\n", i, arg).as_bytes())
            .unwrap();
        println!("arg {} : {:?}", i, arg);
    }

    // ----------------------------------------
    // let mut buf = String::new();
    // std::io::stdin().read_line(&mut buf).unwrap();
    // println!("buf is {:?}", buf);
    // ----------------------------------------

    let file = std::fs::File::open("out.txt").unwrap();
    let mut new_file = std::fs::File::create("new.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        new_file.write_all(line.as_bytes()).unwrap();
        //writeln!(new_file, "{}", line).unwrap();
        println!("line is {:?}", line);
    }
    // ----------------------------------------

    let receiver = match std::net::UdpSocket::bind("127.0.0.1:0") {
        Ok(receiver) => receiver,
        Err(error) => {
            println!("couldn't bind socket: {}", error);
            return;
        }
    };
    println!(
        "the bound port {} for listening",
        receiver.local_addr().unwrap().port()
    );

    let sender = std::net::UdpSocket::bind("127.0.0.1:0").unwrap();
    sender
        .send_to(b"hello", receiver.local_addr().unwrap()) //b turn string of caracter to byte to syring of bytes
        .unwrap();

    let mut buf = [0; 256];
    let (n, sender_address) = receiver.recv_from(&mut buf).unwrap(); // 256 is the size of the buffer
    println!("received {:?} from {}", buf, sender_address);
    //convert the buffer to a string
    let received = std::str::from_utf8(&mut buf[0..n]).unwrap();
    println!("received {:?}", received);
    // ----------------------------------------
    // ERROR HANDLING
    // ----------------------------------------
    read_file().unwrap();
}

#[derive(Debug)]
enum Error {
    Open(std::io::Error),
    Read(std::io::Error),
}

fn read_file() -> Result<(), Error> {
    // let file = match std::fs::File::open("out.txt").map_err(|err| Error::Open(err)) {
    //     Ok(file) => file,
    //     Err(err) => return Err(err),
    // };

    let file = std::fs::File::open("out.txt").map_err(|err| Error::Open(err))?;

    let reader = std::io::BufReader::new(file);

    for line in reader.lines() {
        let line = match line.map_err(|err| Error::Read(err)) {
            Ok(file) => file,
            Err(err) => return Err(err),
        };
        println!("line is {:?}", line);
    }
    Ok(())
}
