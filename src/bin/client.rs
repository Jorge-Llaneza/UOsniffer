use uo_sniffer::ConsoleInteractor;

fn main() {
    let console_interactor = ConsoleInteractor {};
    uo_sniffer::run_client(console_interactor)
        .expect("Fatal error on the application, restarting now");
}