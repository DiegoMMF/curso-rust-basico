use csv::{ReaderBuilder, StringRecord};
use std::collections::HashMap;
use std::fs;

// Constantes del programa que definen el nombre del archivo CSV y el primer tag de la historia
const FILENAME: &str = "history.csv";
const FIRST_TAG: &str = "INICIO";

#[derive(Debug)] // Debug trait para poder imprimir la estructura de datos con println!

// Estructura de datos que representa un dato de la historia con sus respectivas opciones
struct DatoHistoria {
    tipo_dato: String,
    tag: String,
    texto: String,
    vida: i32,
    opciones: Vec<DatoHistoria>,
}

// Implementacion de metodos para la estructura de datos DatoHistoria
impl DatoHistoria {
    // Metodo que crea un nuevo DatoHistoria a partir de un registro CSV
    fn new(row: StringRecord) -> DatoHistoria {
        // Convertir la vida a un entero
        let vida = row.get(3).unwrap().trim();
        let vida: i32 = vida.parse().unwrap_or(0);

        // Crear y retornar un nuevo DatoHistoria
        return DatoHistoria {
            tipo_dato: row.get(0).unwrap().trim().to_string(),
            tag: row.get(1).unwrap().trim().to_string(),
            texto: row.get(2).unwrap().trim().to_string(),
            vida, // vida: vida
            opciones: vec![],
        };
    }
}

// Funcion principal del programa
fn main() {
    // Variables del programa
    let mut vida = 100;
    let mut tag_actual = FIRST_TAG;
    let mut last_record: String = "".to_string();
    let mut datos_historia: HashMap<String, DatoHistoria> = HashMap::new();

    // Leer el contenido del archivo CSV
    let content = fs::read_to_string(FILENAME).unwrap();
    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(content.as_bytes());

    // Iterar sobre los registros del archivo CSV
    for result in rdr.records() {
        // Crear un nuevo DatoHistoria a partir del registro CSV
        let result = result.unwrap();
        let dato = DatoHistoria::new(result);

        if dato.tipo_dato == "SITUACION" {
            // Insertar el dato en el hashmap
            let record_tag = dato.tag.clone();
            datos_historia.insert(record_tag.clone(), dato);
            last_record = record_tag;
        } else if dato.tipo_dato == "OPCION" {
            // Obtener el dato de historia anterior (SITUACION)
            if let Some(data) = datos_historia.get_mut(&last_record) {
                // Insertar la opcion en el dato de historia
                (*data).opciones.push(dato);
            }
        }
    }

    // Game Loop
    loop {
        println!("Tienes {} de vida", vida);
        // Obtener el dato de historia actual
        if let Some(data) = datos_historia.get(tag_actual) {
            println!("{}", data.texto);
            // Mostrar las opciones del dato de historia actual y obtener la seleccion
            for (indice, option) in data.opciones.iter().enumerate() {
                println!("[{}] {}", indice, option.texto);
            }

            // Leer la seleccion del usuario y cambiar el tag actual por el tag de la opcion seleccionada
            let mut seleccion = String::new();
            std::io::stdin().read_line(&mut seleccion).unwrap();
            let seleccion = seleccion.trim().parse().unwrap_or(99);

            // Si la seleccion es valida entonces cambiar el tag actual por el tag de la opcion seleccionada
            if let Some(opcion_elegida) = &data.opciones.get(seleccion) {
                tag_actual = &opcion_elegida.tag;
            } else {
                println!("Comando no valido");
            }

            // Sumar la vida del dato de historia actual a la vida del jugador
            vida += data.vida;
            println!("");
        } else {
            break;
        }

        // Si la vida <= 0 entonces terminar juego
        if vida <= 0 {
            println!("Has perdido!");
            break;
        }
    }
}
