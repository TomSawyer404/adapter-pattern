trait Computer {
    fn insert_into_lightning_port(&self);
}

struct Client {}

impl Client {
    fn insert_lightning_connector_into_computer<T: Computer>(&self, computer: T) {
        println!("Client inserts Lightning connector into computer.");
        computer.insert_into_lightning_port();
    }
}

struct Mac {}

impl Computer for Mac {
    fn insert_into_lightning_port(&self) {
        println!("Lightning connector is plugged into mac machine.");
    }
}

struct Windows {}

impl Windows {
    fn insert_into_usb_port(&self) {
        println!("USB connector is plugged into windows machine.");
    }
}

struct WindowsAdapter {
    win_machine: Windows,
}

impl Computer for WindowsAdapter {
    fn insert_into_lightning_port(&self) {
        println!("Adapter converts Lightning signal to USB ...");
        self.win_machine.insert_into_usb_port();
    }
}

fn main() {
    let client = Client {};
    let mac = Mac {};
    client.insert_lightning_connector_into_computer(mac);

    println!();

    let windows_machine = Windows {};
    let windows_adapter = WindowsAdapter {
        win_machine: windows_machine,
    };
    client.insert_lightning_connector_into_computer(windows_adapter);
}
