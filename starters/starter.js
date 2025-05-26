const { exec } = require("child_process");
const { readline } = require("node:readline")

const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
});


while (true) {
    rl.question(`user@pterodactyl ` + __dirname + '>', command => {
        if (command === "exit") {
            console.log("bye bye")
            break;
        }
        exec(command)
        rl.close();
    });
}