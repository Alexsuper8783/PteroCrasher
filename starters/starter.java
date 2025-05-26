import java.util.Scanner;

public class starter {
    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);
        String currentDir = System.getProperty("user.dir");

        while (true) {
            System.out.print("user@pterodactyl " + currentDir + "> ");
            String input = scanner.nextLine().trim();

            if (input.equals("exit")) {
                System.out.println("bye bye");
                break;
            }

            try {
                Process process = Runtime.getRuntime().exec(input);


                java.io.BufferedReader reader = new java.io.BufferedReader(
                    new java.io.InputStreamReader(process.getInputStream())
                );

                String line;
                while ((line = reader.readLine()) != null) {
                    System.out.println(line);
                }

                process.waitFor();
                currentDir = System.getProperty("user.dir");
            } catch (Exception e) {
                System.err.println("Error: " + e.getMessage());
            }
        }

        scanner.close();
    }
}