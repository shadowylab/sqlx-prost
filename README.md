# sqlx-prost

A small helper for using [`prost`](https://docs.rs/prost) protobuf types with [`sqlx`](https://docs.rs/sqlx).

## What it does

`sqlx-prost` provides a wrapper type that lets you store and retrieve protobuf messages as binary values in SQLx-supported databases.

## Usage

Wrap your protobuf message type with `SqlxProto(T)` and use it in your SQLx models.

## License

This project is distributed under the MIT software license – see the [LICENSE](LICENSE) file for details
