# toy-rust-os
implement os by rust.  
https://os.phil-opp.com/ja/  

This OS build
```
$ cargo build --target thumbv7em-none-eabihf
```

rustup set nightly
```
$ rustup override set nightly
```

rustup add component
```
$ rustup component add rust-src --toolchain nightly-aarch64-apple-darwin
```

create bootimage
```
$ cargo install bootimage
$ cargo bootimage
```

install qemu
```
$ brew install qemu
```

run my os in qemu
```
$ cargo run
```
cargo run equals `bootimage runner` configured in .cargo/config.toml.  
bootimage runner command link my os and bootloader and start qemu.  
