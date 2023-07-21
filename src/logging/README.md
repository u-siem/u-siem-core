# Logging

µSIEM is desingned to work as a micro-service, so the logging sytem must be designed so that we always know which node and component has troubles.

Because the Kernel is the one responsible of interconecting all components, it's his duty to also prepare the logging system for each component.
```rust
set_max_level(NotificationLevel::Info);
set_enabled_level(NotificationLevel::Info);
initialize_component_logger(kernel_Sender);
```
Now the component can use the logging macros to communicate with the Kernel:
```rust
error!("This is log example: {}", "LOG");
warn!("This is log example: {}", "LOG");
info!("This is log example: {}", "LOG");
debug!("This is log example: {}", "LOG");
trace!("This is log example: {}", "LOG");
```
The functions for logging are the same as the `log` crate, but there is also other methods used for sending messages, not logs.
```rust
let msg : SiemMessage = ...;
let res : SiemResult<(), SiemError> = send_message_timeout!(msg, Duration::from_millis(1_000));
let res : SiemResult<(), SiemError> = send_message!(msg);
let res : SiemResult<(), SiemError> = try_send_message!(msg);
```


```rust

pub struct MyComponent {}

impl SiemComponent for MyComponent {
    pub fn run() -> SiemResult<()> {
        info!("Hello World as Info log!");
        send_message_timeout!(
            SiemMessage::Command(
                SiemCommandHeader {
                    comm_id: 1,
                    comp_id: 2,
                    user: String::new()
                },
                SiemCommandCall::STOP_COMPONENT(format!("Test"))
            ),
            Duration::from_millis(1_000)
        ).expect("Message must be sent");

        send_message!(
            SiemMessage::Command(
                SiemCommandHeader {
                    comm_id: 1,
                    comp_id: 2,
                    user: String::new()
                },
                SiemCommandCall::STOP_COMPONENT(format!("Test"))
            )
        ).expect("Message must be sent");
    }
}
```