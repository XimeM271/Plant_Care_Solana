# 🌿 Plant Care Solana


Sistema básico de gestión de plantas desarrollado como **Solana Program** utilizando **Rust** y el framework **Anchor**.  

Este proyecto implementa un sistema **CRUD** para administrar un invernadero digital en blockchain, enfocado en simplicidad y claridad del modelo de datos:

- 🔑 Program Derived Addresses (PDAs)  
- ⚡ Optimización de memoria *On-Chain*  
- 🔒 Seguridad mediante validación de propietario  

---

## 📚 Descripción

**Plant Care Solana (Simple)** permite a un usuario:

- Crear un invernadero personal  
- Registrar plantas con sus cuidados  
- Editar parámetros de riego y luz  
- Eliminar plantas  
- Consultar toda la información almacenada  

---

## 🧠 Arquitectura y Estructuras de Datos

En Solana es necesario definir el tamaño de los datos para calcular la renta (*rent*).

### 📦 PDA Principal: `Invernadero`

Cuenta raíz que almacena todas las plantas.

```rust
#[account]
#[derive(InitSpace)]
pub struct Invernadero {
    pub owner: Pubkey,
    #[max_len(40)]
    pub nombre_invernadero: String,
    #[max_len(15)]
    pub plantas: Vec<Planta>,
}
```

---

### 🧩 Estructura Interna: `Planta`

Cada planta contiene:

- `especie (String)` → nombre de la planta  
- `volumen_ml (u16)` → cantidad de agua en mililitros  
- `frecuencia_dias (u8)` → frecuencia de riego  
- `luz_directa (bool)` → si requiere luz directa  

```rust
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Planta {
    #[max_len(30)]
    pub especie: String,
    pub volumen_ml: u16,
    pub frecuencia_dias: u8,
    pub luz_directa: bool,
}
```

---

## 🔒 Seguridad

El contrato valida que solo el propietario pueda modificar el invernadero:

```rust
require!(
    invernadero.owner == ctx.accounts.owner.key(),
    Errores::NoEresElOwner
);
```

✔ Protege los datos del usuario  
✔ Evita accesos no autorizados  

---

## ⚙️ Funcionalidad (CRUD)

### 🟢 Inicializar Invernadero

Crea la cuenta principal usando:

```rust
[b"invernadero", owner.key().as_ref()]
```

Inicializa:
- Owner  
- Nombre del invernadero  
- Lista vacía de plantas  

---

### ➕ Registrar Planta

- Recibe:
  - especie  
  - volumen de riego  
  - frecuencia  
  - luz directa  
- Inserta en el vector con `.push()`  

---

### ✏️ Editar Planta

- Busca por `especie`  
- Actualiza:
  - volumen  
  - frecuencia  
  - luz  

---

### ❌ Eliminar Planta

```rust
.iter().position(|x| x.especie == especie)
```

- Si existe → `.remove(index)`  
- Si no → error `PlantaNoExiste`  

---

### 📖 Leer Invernadero

```rust
msg!("Datos: {:#?}", invernadero.plantas);
```

Muestra todas las plantas en logs *On-Chain*

---

## 🧪 Despliegue en Solana Playground

1. Copia el código en `lib.rs`  
2. Ejecuta:

```bash
cargo clean
```

3. Haz clic en **Build**  
4. Haz clic en **Deploy (Devnet)**  

---

## 🧑‍💻 Pruebas

Puedes interactuar usando:

- Pestaña **Test** del Playground  
- Scripts en TypeScript:

```ts
pg.program.methods...
```

Parámetros:
- `especie: String`  
- `volumen_ml: u16`  
- `frecuencia_dias: u8`  
- `luz_directa: bool`  

---

## ⚠️ Manejo de Errores

```rust
#[error_code]
pub enum Errores {
    #[msg("No tienes permisos.")]
    NoEresElOwner,
    #[msg("La planta no existe en el invernadero.")]
    PlantaNoExiste,
}
```

---

## 📌 Conclusión

Este proyecto demuestra:

- Implementación básica de CRUD en Solana  
- Control de acceso mediante owner  
- Manejo de estructuras dinámicas (Vec)  
- Diseño limpio y sencillo para aprendizaje  

---

## 🚀 Próximos pasos

- Añadir alertas de riego  
- Implementar historial de cambios  
- Integrar sensores o IoT  
- Crear interfaz gráfica para monitoreo  

---
