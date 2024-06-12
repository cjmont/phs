# USDT/ARS PHS

Este microservicio obtiene la mejor tasa de cambio USDT/ARS desde APIs diferentes y proporciona la tasa más alta disponible.

## Requisitos

- Rust
- Cargo

## Dependencias

- `actix-web`: Framework web para Rust.
- `serde`: Biblioteca de serialización y deserialización.
- `reqwest`: Cliente HTTP para hacer solicitudes a las APIs.
- `std::collections::HashMap`: Utilizado para manejar la respuesta de CoinGecko.

## Instalación

1. Clonar el repositorio:
    ```sh
    git clone https://github.com/cjmont/phs.git
    cd phs
    ```

2. Construir el proyecto:
    ```sh
    cargo build
    ```

3. Ejecutar el proyecto:
    ```sh
    cargo run
    ```

## Endpoints

### `GET /get_best_usdtars`

Este endpoint devuelve la mejor tasa de cambio USDT/ARS obtenida de Binance y CoinGecko.

#### Ejemplo de respuesta

```json
{
    "Best USDT/ARS Rate": 142.56
}