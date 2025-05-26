import os

os.chdir("./starters")
os.system("javac starter.java")
os.system("jar cfe starter.jar starter starter.class")
os.chdir("../")
os.system("cargo build --release --target x86_64-unknown-linux-musl")