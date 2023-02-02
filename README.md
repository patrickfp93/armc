# Armc
Armc é uma biblioteca em Rust que fornece um envólucro para dados compartilhados, com garantia de integridade dos dados e bloqueio de threads durante modificações e leituras.

## Instalação
Você pode adicionar a dependência Armc ao seu arquivo Cargo.toml:

### toml
 ```yaml
   [dependencies]
    armc = "1.3.2"
```
## Uso
para usar a biblioteca, basta importá-la com o seguinte código:
```rust
use armc::Armc;
```
### Funcionalidades
A seguir, algumas das funcionalidades da biblioteca:

### Criação de um objeto Armc
Para criar um objeto Armc, basta usar o método new e passar o dado que deseja armazenar:
```rust
let armc = Armc::new(5);
```

### lock_ref:
Acesso aos dados de um objeto Armc
Você pode acessar os dados armazenados bloqueando possíveis mutações. Pode ser feito multiplos acessos em paralelo.

    let data = armc.lock_ref();
    println!("Data: {:?}", data);
### lock:
Modificação de dados de um objeto Armc
Para modificar os dados de um objeto Armc, você precisa usar o método lock, o mesmo bloqueará todos os acessos de mutação:

    let mut data = armc.lock();
    *data = 10;
    println!("Data: {:?}", data);
### Clonagem de um objeto Armc
Você pode clonar um objeto Armc usando o método clone:
    let armc_clone = armc.clone();
    println!("Data: {:?}", *armc_clone.lock_ref());
## Contribuição
Contribuições são bem-vindas! Sinta-se livre para abrir uma issue ou enviar uma pull request.
