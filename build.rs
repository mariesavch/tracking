fn main() {
    std::process::Command::new("bunx")
        .args([
            "tailwindcss",
            "-i",
            "tailwind.css",
            "-o",
            "./assets/tailwind.css",
            "--minify",
        ])
        .env("NODE_ENV", "production")
        .spawn()
        .unwrap();
}
