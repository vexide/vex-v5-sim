use std::{option::Option, path::PathBuf, pin::Pin, time::Duration};

use log::LevelFilter;
use simplelog::{ColorChoice, ConfigBuilder, TermLogger, TerminalMode};
use tokio::{
    io::{stdin, stdout, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt},
    net::TcpListener,
    process::Command,
    sync::Mutex,
    time::sleep,
};
use vex_v5_qemu_host::{
    brain::{Binary, Brain},
    peripherals::usb::{UsbRead, UsbWrite},
    protocol::battery::BatteryData,
};
use winit::event_loop::EventLoop;

use crate::display_window::DisplayWindow;

mod display_window;

#[cfg(debug_assertions)]
const DEFAULT_KERNEL: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../kernel/target/armv7a-none-eabi/debug/kernel"
);

#[cfg(not(debug_assertions))]
const DEFAULT_KERNEL: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../kernel/target/armv7a-none-eabi/release/kernel"
);

/// Simulate a VEX V5 robot program
#[derive(clap::Parser)]
struct Opt {
    /// Start the simulator in a paused state and open a GDB server.
    ///
    /// When enabled, the simulator will make a GDB server available on port
    /// 1234, allowing a debugger to set breakpoints in and step through the
    /// kernel or user code.
    #[clap(short, long, env = "V5_SIM_GDB")]
    gdb: bool,

    /// Override the kernel image.
    ///
    /// The simulator requires an emulated kernel to handle SDK calls
    /// and set up the virtual machine before running the robot code.
    /// This option defaults to a kernel designed to replicate the behavior
    /// of programs under VEXos.
    #[clap(short, long, default_value = DEFAULT_KERNEL, env = "V5_SIM_KERNEL_PATH")]
    kernel: PathBuf,

    /// Override the QEMU executable to a custom version of `qemu-system-arm`.
    #[clap(short, long, default_value = "qemu-system-arm", env = "V5_SIM_QEMU")]
    qemu: PathBuf,

    /// Main program binary
    #[clap(long)]
    program: PathBuf,

    /// Main program binary load address
    #[clap(long)]
    load_addr: Option<u32>,

    /// Linked program binary
    #[clap(long, requires("link_addr"))]
    link: Option<PathBuf>,

    /// Linked program binary load address
    #[clap(long, value_parser = validate_address_range)]
    link_addr: Option<u32>,

    /// Interpret the supplied program as a PROS project directory.
    ///
    /// When this option is set, `--program` specifies a directory containing a PROS project.
    /// In "hold-cold" mode, hot.package.bin and cold.package.bin will be simulated. In "monolith"
    /// mode, monolith.bin will be simulated.
    #[clap(long, conflicts_with_all(["link", "link_addr", "load_addr"]))]
    pros: Option<ProsMode>,

    #[clap(long)]
    save_imgs: bool,

    #[clap(long)]
    tcp: Option<u16>,

    /// Extra arguments to pass to QEMU.
    qemu_args: Vec<String>,
}

#[derive(clap::ValueEnum, Clone, Copy, PartialEq, Eq)]
enum ProsMode {
    HotCold,
    Monolith,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut opt = <Opt as clap::Parser>::parse();

    TermLogger::init(
        LevelFilter::Debug,
        ConfigBuilder::new()
            .set_thread_level(LevelFilter::Off)
            .build(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();

    let mut qemu = Command::new("qemu-system-arm");
    qemu.args(opt.qemu_args);
    if opt.gdb {
        qemu.args(["-S", "-s"]);
    }

    if let Some(pros) = opt.pros {
        let bin_dir = opt.program.join("bin");
        match pros {
            ProsMode::HotCold => {
                opt.program = bin_dir.join("hot.package.bin");
                opt.load_addr = Some(0x07800000);
                opt.link = Some(bin_dir.join("cold.package.bin"));
                opt.link_addr = Some(0x03800000);
            }
            ProsMode::Monolith => {
                opt.program = bin_dir.join("monolith.bin");
            }
        }
    }

    let mut brain = Brain::new(
        qemu,
        opt.kernel,
        Binary {
            path: opt.program,
            load_addr: opt.load_addr.unwrap_or(0x03800000),
        },
        opt.link.map(|link| Binary {
            path: link,
            load_addr: opt.link_addr.unwrap(),
        }),
    )
    .unwrap();
    let peripherals = brain.peripherals.take().unwrap();

    let tcp_port = opt.tcp;
    tokio::task::spawn(async move {
        let usb_read = peripherals.usb_read;
        let usb_write = peripherals.usb_write;

        if let Some(tcp_port) = tcp_port {
            let listener = TcpListener::bind(("127.0.0.1", tcp_port)).await.unwrap();
            let (mut stream, _) = listener.accept().await.unwrap();
            let (read, write) = stream.split();

            forward_stdio(usb_read, usb_write, write, read)
                .await
                .unwrap()
        } else {
            forward_stdio(usb_read, usb_write, stdout(), stdin())
                .await
                .unwrap();
        }
    });

    let _ = tokio::task::block_in_place(move || {
        let event_loop = EventLoop::new().unwrap();
        let mut app = DisplayWindow::new(peripherals.display, peripherals.touch, opt.save_imgs);

        event_loop.run_app(&mut app)
    });

    brain.terminate().await?;

    std::process::exit(0);
}

fn validate_address_range(s: &str) -> Result<u32, String> {
    clap_num::maybe_hex_range(s, 0x03800000, 0x8000000)
}

async fn forward_stdio(
    mut usb_read: UsbRead,
    mut usb_write: UsbWrite,
    mut stdout: impl AsyncWrite + Unpin,
    mut stdin: impl AsyncRead + Unpin,
) -> tokio::io::Result<()> {
    // Deal with USB -> stdout, and handle non-UTF-8 data using lossy conversion
    let usb_to_stdout = async {
        let mut buffer = vec![0u8; 1024];
        loop {
            match usb_read.read(&mut buffer).await {
                Ok(0) => return Ok::<_, tokio::io::Error>(()), // EOF
                Ok(n) => {
                    // 将字节转换为字符串（lossy 方式处理无效 UTF-8）
                    let text = String::from_utf8_lossy(&buffer[..n]);
                    stdout.write_all(text.as_bytes()).await?;
                    stdout.flush().await?;
                }
                Err(e) => return Err(e),
            }
        }
    };

    tokio::try_join!(
        usb_to_stdout,
        tokio::io::copy(&mut stdin, &mut usb_write),
    )?;
    Ok(())
}
