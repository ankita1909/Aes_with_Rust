This project contains implementation of AES algorithm with Rust. The module aes_rustway.rs contains the implementation where input stream is declared as an 2d array and all the operations happen on this input. The final result is compared with output stream which is also defined as an 2d array using the function `test_aes` defined in `aes_rustway.rs`

Another module called `aes_using_1d_array.rs` implements the same functionality of AES algorithm using input as a 1D array and changing the reference to the input as a 1D array as well.

Prerequisites:

- Install the required tools as described in [Embedded Rust Book](https://doc.rust-lang.org/beta/embedded-book/intro/install.html)
- Build this project using `cargo build`
- Upgrade Board KitProg to KitProg3
- Download TeraTerm/Putty.
- Clone the repo `ssh://git@bitbucket.cloudapps.infineon.com:7999/~kumarankita/psoc6-o2-pac.git`

- Install Cypress OpenOCD
    [Manual](https://www.cypress.com/file/463231/download)
    [Download](https://github.com/cypresssemiconductorco/openocd/releases)

TeraTerm Configuration:
    - Baudrate: 9600
    - Stop Bit: 1
    - Parity: None
    - Data: 8bit

For Debugiing/Running the project

- Clone the project
- Open the project with vscode.
- Run `cargo build` in the terminal window.
- Goto Run -> Start Debugging/ Run without debugging.


