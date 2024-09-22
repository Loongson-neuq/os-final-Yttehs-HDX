#!/bin/python3

import os

BASE_ADDR = 0x80400000
APP_SIZE_LIMIT = 0x20000
LINKER = 'src/linker.ld'

CARGO = 'cargo'
BUILD_MODE = 'release'

index = 0
apps = os.listdir('src/bin')
apps.sort()

for app in apps:
    app_name = app[:app.find('.')]

    raw = ""
    processed = ""

    # Read the linker file
    with open(LINKER, 'r') as f:
        raw = f.read()

        # Change BASE_ADDR for each app
        new_addr = hex(BASE_ADDR + index * APP_SIZE_LIMIT)
        processed = raw.replace(hex(BASE_ADDR), new_addr)

    # Write the new linker file
    with open(LINKER, 'w') as f:
        f.write(processed)

    # Build the app
    print(f"[BUILD] application with BASE_ADDR {new_addr}")
    os.system(f"{CARGO} build --bin {app_name} --{BUILD_MODE}")

    # Restore the linker file
    with open(LINKER, 'w') as f:
        f.write(raw)

    index += 1