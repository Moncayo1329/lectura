use std::io;

struct Lectura { 

    nombre: String,
    descripcion: String,
    numero_libro: u32,

}


fn main(){
// Crear variables para almacenar libros ingresados por el usuario.

let mut libros: Vec<Lectura> = Vec::new(); // se puede guardar mas libros con vec 

loop {


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


let libro = Lectura {

nombre:nombre.trim().to_string(),
descripcion:descripcion.trim().to_string(),
numero_libro,

};

libros.push(libro);

println!("¿Deseas ingresar otro libro? (s/n)");
let mut continuar = String::new();
io::stdin().read_line(&mut continuar).expect("Error al leer");
if continuar.trim().to_lowercase() != "s"{

    break;
}
}

println!("\nLista de libros ingresados:");
for (i,libro) in libros.iter().enumerate(){
println!("

{}.Libro: {}\nDescripción: {}\nNúmero: {}",
i + 1,
    libro.nombre,
     libro.descripcion, 
     libro.numero_libro


    );
}
}