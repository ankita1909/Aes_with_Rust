#!/bin/bash

cargo build --release

arm-none-eabi-size target/thumbv7em-none-eabihf/release/psoc6_helloworld >> size.txt

echo >> size.txt
echo "===============" >> size.txt
echo >> size.txt

cargo bloat --release -n 30 >> size.txt