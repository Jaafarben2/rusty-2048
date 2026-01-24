###  rusty_2048

Classic 2048 reinvented with multiple variants in Rust.
It is already deployed at: [https://staticfile-jaafarbn2000.wasmer.app/](https://staticfile-jaafarbn2000.wasmer.app/)

## Development

1. Open the project in **VS Code**.  
2. Reopen in the Dev Container when prompted.  
3. Inside the container, run:

```bash
dx serve --platform web
```

## Build

1. 
```bash
dx build --platform web --release
```
2. Web static files will be generated under rusty_2048/target/dx/rusty_2048/release/web/public


## License

This project is licensed under the MIT License – see the [LICENSE](LICENSE) file for details.