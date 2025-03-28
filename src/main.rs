use std::io;

struct Lectura { 

    nombre: String,
    descripcion: String,
    numero_libro: u32,

}


fn main(){
// Crear variables para almacenar libros ingresados por el usuario.

let mut nombre = String::new();
let mut descripcion = String::new();
let mut numero_libro = String::new();

// Pedir al usuario que ingrese la descripcion del gasto. 

println!("Cual es el nombre del libro?");
io::stdin()
.read_line(&mut nombre)
.expect("Error al leer el libro");


// Pedir al usario que ingrese la descripcion del libro

println!("Cual es la descripcion del libro?");
io::stdin()
.read_line(&mut descripcion)
.expect("Error en la descripcion del libro");


println!("Numero del libro?");
io::stdin()
.read_line(&mut numero_libro)
.expect("error en el numero del libro");

// Convertir numero libro de string a u32

let numero_libro: u32 = match numero_libro.trim().parse() {
    Ok(num) => num,
    Err(_) => {
        println!("Debe ingresar un número válido.");
        return;
    }
};

println!("Libro: {}\nDescripción: {}\nNúmero: {}", nombre.trim(), descripcion.trim(), numero_libro);

let libro = Lectura {

nombre:nombre.trim().to_string(),
descripcion:descripcion.trim().to_string(),
numero_libro,

};

println!(
"Libro: {}\nDescripción: {}\nNúmero: {}",
    libro.nombre, libro.descripcion, libro.numero_libro

);




}