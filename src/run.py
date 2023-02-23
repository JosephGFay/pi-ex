import os

os.system("git pull pi")

os.system("cargo build")

os.system("sudo ./target/debug/pi-ex")
