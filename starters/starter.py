import os

while True:
    inp = input(f"user@pterodactyl {os.path}>")
    if inp == "exit":
        print("bye bye")
        break
    os.system(inp)