Para tu proyecto que implica un chunker semántico utilizando Rust, BERT para embeddings y Qdrant como base de datos de vectores, aquí tienes una guía detallada para el archivo `README.md`:

---

# Semantic Chunker

Este proyecto implementa un chunker semántico utilizando Rust. Utiliza el modelo BERT para generar embeddings de texto y Qdrant para almacenar y realizar búsquedas semánticas eficientes.

## Requisitos Previos

- Rust (latest stable version)
- Cargo (incluido con Rust)
- Qdrant corriendo localmente o acceso a una instancia de Qdrant Cloud

## Configuración

1. **Qdrant**: Asegúrate de tener Qdrant corriendo. Para una instancia local, puedes usar Docker:

   ```sh
   docker run -p 6333:6333 qdrant/qdrant
   ```

   Para Qdrant Cloud, asegúrate de tener tu `QDRANT_URL` y `QDRANT_API_KEY`.

2. **Variables de Entorno**: Configura las variables de entorno necesarias. Crea un archivo `.env` en la raíz del proyecto con el siguiente contenido:

   ```env
   QDRANT_URL=http://localhost:6333 # O tu URL de Qdrant Cloud
   QDRANT_API_KEY=your_api_key # Solo necesario para Qdrant Cloud
   ```

   Asegúrate de reemplazar `your_api_key` con tu clave API real si estás usando Qdrant Cloud.

3. **Dependencias**: Instala las dependencias del proyecto ejecutando:
   ```sh
   cargo build
   ```

## Uso

El proyecto soporta tres operaciones principales: convertir archivos `.txt` a `.jsonl`, insertar datos en Qdrant y realizar búsquedas semánticas.

### Convertir Archivos `.txt` a `.jsonl`

Para convertir un archivo `.txt` a `.jsonl`:

```sh
cargo run -- convert path/to/your/file.txt
```

Esto generará un archivo `output.jsonl` en la misma carpeta.

### Insertar Datos en Qdrant

Para insertar los datos de un archivo `.jsonl` en Qdrant:

```sh
cargo run -- insert path/to/your/file.jsonl
```

Asegúrate de tener la colección `points` configurada en Qdrant o la colección será creada automáticamente.

### Realizar Búsqueda Semántica

Para buscar en los datos insertados basándose en texto semántico:

```sh
cargo run -- find "your search query"
```

Esto retornará los puntos más cercanos (embeddings) basados en la similitud del coseno.

## Pruebas

Para probar todas las funcionalidades, sigue estos pasos en orden:

1. **Convierte un archivo `.txt` a `.jsonl`**.
2. **Inserta los datos del archivo `.jsonl` en Qdrant**.
3. **Realiza una búsqueda semántica** usando una consulta relevante al contenido de tu archivo `.txt`.

## Contribuciones

Las contribuciones son bienvenidas. Por favor, envía un Pull Request o abre un issue para sugerencias o reportes de bugs.

## Licencia

[MIT](LICENSE)

---

Este `README.md` proporciona una visión general del proyecto, instrucciones de configuración y uso, y la invitación a contribuir. Asegúrate de ajustar los pasos y los comandos según los detalles específicos de tu implementación y configuración de Qdrant.
